use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, MouseEvent};

/// UI event types
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
    Resize(u16, u16),
}

/// Helper to check if key event should be processed
pub fn should_process_key_event(key_event: &KeyEvent) -> bool {
    // Only process key press events (not release or repeat)
    key_event.kind == KeyEventKind::Press
}

/// Check if key is a character
pub fn is_char_key(key_event: &KeyEvent, c: char) -> bool {
    should_process_key_event(key_event) && matches!(key_event.code, event::KeyCode::Char(ch) if ch == c)
}

/// Check if key is enter
pub fn is_enter_key(key_event: &KeyEvent) -> bool {
    should_process_key_event(key_event) && key_event.code == event::KeyCode::Enter
}

/// Check if key is escape
pub fn is_esc_key(key_event: &KeyEvent) -> bool {
    should_process_key_event(key_event) && key_event.code == event::KeyCode::Esc
}

/// Check if key is up arrow
pub fn is_up_key(key_event: &KeyEvent) -> bool {
    should_process_key_event(key_event) && key_event.code == event::KeyCode::Up
}

/// Check if key is down arrow
pub fn is_down_key(key_event: &KeyEvent) -> bool {
    should_process_key_event(key_event) && key_event.code == event::KeyCode::Down
}

/// Check if key is left arrow
pub fn is_left_key(key_event: &KeyEvent) -> bool {
    should_process_key_event(key_event) && key_event.code == event::KeyCode::Left
}

/// Check if key is right arrow
pub fn is_right_key(key_event: &KeyEvent) -> bool {
    should_process_key_event(key_event) && key_event.code == event::KeyCode::Right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    #[test]
    fn test_is_char_key() {
        let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        assert!(is_char_key(&event, 'a'));
        assert!(!is_char_key(&event, 'b'));
    }

    #[test]
    fn test_is_enter_key() {
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        assert!(is_enter_key(&event));
    }

    #[test]
    fn test_is_esc_key() {
        let event = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        assert!(is_esc_key(&event));
    }

    #[test]
    fn test_is_up_key() {
        let event = KeyEvent::new(KeyCode::Up, KeyModifiers::empty());
        assert!(is_up_key(&event));
    }

    #[test]
    fn test_is_down_key() {
        let event = KeyEvent::new(KeyCode::Down, KeyModifiers::empty());
        assert!(is_down_key(&event));
    }

    #[test]
    fn test_is_left_key() {
        let event = KeyEvent::new(KeyCode::Left, KeyModifiers::empty());
        assert!(is_left_key(&event));
    }

    #[test]
    fn test_is_right_key() {
        let event = KeyEvent::new(KeyCode::Right, KeyModifiers::empty());
        assert!(is_right_key(&event));
    }
}
