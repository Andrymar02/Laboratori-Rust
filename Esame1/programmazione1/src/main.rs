use std::sync::{Condvar, Mutex};
use std::time::Instant;

/// Tipo della funzione (fornita esternamente) che acquisisce il token.
/// Deve essere Send + Sync perché verrà invocata da thread diversi
/// e condivisa tra loro tramite il TokenManager.
pub type TokenAcquirer = dyn Fn() -> Result<(String, Instant), String> + Send + Sync;

/// I tre possibili stati interni del TokenManager.
enum State {
    Empty,
    Pending,
    Valid { token: String, expiry: Instant },
}

/// Azione da compiere, decisa osservando lo stato corrente.
/// Serve solo a separare "lettura dello stato" da "modifica dello stato"
/// per non avere conflitti con il borrow checker.
enum Action {
    Return(String),
    Wait,
    Acquire,
}

pub struct TokenManager {
    acquire_token: Box<TokenAcquirer>,
    state: Mutex<State>,
    cond: Condvar,
}

impl TokenManager {
    /// Crea l'istanza in modo "pigro": nessuna richiesta viene fatta qui.
    pub fn new(acquire_token: Box<TokenAcquirer>) -> Self {
        TokenManager {
            acquire_token,
            state: Mutex::new(State::Empty),
            cond: Condvar::new(),
        }
    }

    /// Restituisce un token valido, acquisendolo o attendendo se necessario.
    pub fn get_token(&self) -> Result<String, String> {
        loop {
            let mut guard = self.state.lock().unwrap();

            // 1) Decidiamo l'azione guardando lo stato attuale.
            let action = match &*guard {
                State::Empty => Action::Acquire,
                State::Pending => Action::Wait,
                State::Valid { token, expiry } => {
                    if Instant::now() < *expiry {
                        Action::Return(token.clone())
                    } else {
                        Action::Acquire
                    }
                }
            };

            // 2) Eseguiamo l'azione scelta.
            match action {
                Action::Return(token) => return Ok(token),

                Action::Wait => {
                    // Si addormenta rilasciando il lock; si risveglia quando
                    // qualcun altro chiama notify_all() sulla stessa Condvar.
                    guard = self.cond.wait(guard).unwrap();
                    drop(guard);
                    // torna in cima al loop e ricontrolla lo stato
                }

                Action::Acquire => {
                    // Segnaliamo subito agli altri thread che è in corso una richiesta,
                    // POI rilasciamo il lock prima della chiamata (che è lenta).
                    *guard = State::Pending;
                    drop(guard);

                    let result = (self.acquire_token)();

                    let mut guard = self.state.lock().unwrap();
                    match result {
                        Ok((token, expiry)) => {
                            *guard = State::Valid {
                                token: token.clone(),
                                expiry,
                            };
                            drop(guard);
                            self.cond.notify_all();
                            return Ok(token);
                        }
                        Err(e) => {
                            *guard = State::Empty;
                            drop(guard);
                            self.cond.notify_all();
                            return Err(e);
                        }
                    }
                }
            }
        }
    }

    /// Restituisce subito il token se valido, senza mai attendere o acquisire.
    pub fn try_get_token(&self) -> Option<String> {
        let guard = self.state.lock().unwrap();
        if let State::Valid { token, expiry } = &*guard {
            if Instant::now() < *expiry {
                return Some(token.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn a_new_manager_contains_no_token() {
        let a: Box<TokenAcquirer> = Box::new(|| Err("failure".to_string()));
        let manager = TokenManager::new(a);
        assert!(manager.try_get_token().is_none());
    }

    #[test]
    fn a_failing_acquirer_always_returns_an_error() {
        let a: Box<TokenAcquirer> = Box::new(|| Err("failure".to_string()));
        let manager = TokenManager::new(a);
        assert_eq!(manager.get_token(), Err("failure".to_string()));
        assert_eq!(manager.get_token(), Err("failure".to_string()));
    }

    #[test]
    fn a_successful_acquirer_always_returns_success() {
        let a: Box<TokenAcquirer> = Box::new(|| {
            Ok(("tok123".to_string(), Instant::now() + Duration::from_secs(60)))
        });
        let manager = TokenManager::new(a);

        assert_eq!(manager.get_token(), Ok("tok123".to_string()));
        assert_eq!(manager.get_token(), Ok("tok123".to_string()));
        assert_eq!(manager.try_get_token(), Some("tok123".to_string()));
    }

    #[test]
    fn a_slow_acquirer_causes_other_threads_to_wait() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let counter = call_count.clone();

        let a: Box<TokenAcquirer> = Box::new(move || {
            counter.fetch_add(1, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(300)); // simula latenza di rete
            Ok(("slow-token".to_string(), Instant::now() + Duration::from_secs(60)))
        });

        let manager = Arc::new(TokenManager::new(a));

        // Più thread chiedono il token quasi contemporaneamente.
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let m = manager.clone();
                thread::spawn(move || m.get_token())
            })
            .collect();

        for h in handles {
            assert_eq!(h.join().unwrap(), Ok("slow-token".to_string()));
        }

        // La funzione di acquisizione deve essere stata chiamata UNA sola volta:
        // gli altri thread devono aver atteso, non richiesto un nuovo token.
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }
}


fn main() {
    println!("TokenManager: esegui i test con `cargo test`");
}