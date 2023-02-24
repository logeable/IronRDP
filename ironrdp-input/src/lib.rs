use bitvec::array::BitArray;
use bitvec::BitArr;
use ironrdp_core::input::fast_path::{FastPathInputEvent, KeyboardFlags};
use ironrdp_core::input::mouse::PointerFlags;
use ironrdp_core::input::mouse_x::PointerXFlags;
use ironrdp_core::input::{MousePdu, MouseXPdu};
use smallvec::SmallVec;

// TODO: unicode keyboard event support

/// Number associated to a mouse button.
///
/// Based on the MouseEvent.button property found in browsers APIs:
/// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button#value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseButton(u8);

impl MouseButton {
    const LEFT_VAL: u8 = 0;
    const MIDDLE_VAL: u8 = 1;
    const RIGHT_VAL: u8 = 2;
    const X1_VAL: u8 = 3;
    const X2_VAL: u8 = 4;

    pub const LEFT: Self = Self(Self::LEFT_VAL);
    pub const MIDDLE: Self = Self(Self::MIDDLE_VAL);
    pub const RIGHT: Self = Self(Self::RIGHT_VAL);
    pub const X1: Self = Self(Self::X1_VAL);
    pub const X2: Self = Self(Self::X2_VAL);

    pub fn is_unknown(self) -> bool {
        self.0 > Self::X2.0
    }

    pub fn as_idx(self) -> usize {
        usize::from(self.0)
    }
}

impl From<u8> for MouseButton {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MouseButton> for u8 {
    fn from(value: MouseButton) -> Self {
        value.0
    }
}

/// Keyboard scan code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scancode {
    code: u8,
    extended: bool,
}

impl Scancode {
    pub fn as_idx(self) -> usize {
        if self.extended {
            usize::from(self.code) + 256
        } else {
            usize::from(self.code)
        }
    }
}

impl From<(u8, bool)> for Scancode {
    fn from((code, extended): (u8, bool)) -> Self {
        Self { code, extended }
    }
}

impl From<u16> for Scancode {
    fn from(code: u16) -> Self {
        let extended = code & 0xE000 == 0xE000;
        let code = code as u8;
        Self { code, extended }
    }
}

impl From<Scancode> for u8 {
    fn from(value: Scancode) -> Self {
        value.code
    }
}

impl From<Scancode> for u16 {
    fn from(value: Scancode) -> Self {
        if value.extended {
            u16::from(value.code) | 0xE000
        } else {
            u16::from(value.code)
        }
    }
}

/// Cursor position for a mouse device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MousePosition {
    pub x: u16,
    pub y: u16,
}

/// Mouse wheel rotations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WheelRotations {
    pub is_vertical: bool,
    pub rotation_units: i16,
}

#[derive(Debug, Clone)]
pub enum Operation {
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    MouseMove(MousePosition),
    WheelRotations(WheelRotations),
    KeyPressed(Scancode),
    KeyReleased(Scancode),
}

pub type KeyboardState = BitArr!(for 512);
pub type MouseButtonsState = BitArr!(for 5);

/// In-memory database for maintaining the current keyboard and mouse state.
pub struct Database {
    keyboard: KeyboardState,
    mouse_buttons: MouseButtonsState,
    mouse_position: MousePosition,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            keyboard: BitArray::ZERO,
            mouse_buttons: BitArray::ZERO,
            mouse_position: MousePosition { x: 0, y: 0 },
        }
    }

    pub fn is_key_pressed(&self, scancode: Scancode) -> bool {
        self.keyboard
            .get(scancode.as_idx())
            .as_deref()
            .copied()
            .unwrap_or(false)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons
            .get(button.as_idx())
            .as_deref()
            .copied()
            .unwrap_or(false)
    }

    pub fn mouse_position(&self) -> MousePosition {
        self.mouse_position
    }

    pub fn keyboard_state(&self) -> &KeyboardState {
        &self.keyboard
    }

    pub fn mouse_buttons_state(&self) -> &MouseButtonsState {
        &self.mouse_buttons
    }

    /// Apply a transaction (list of operations) and returns a list of RDP input events to send.
    ///
    /// Operations that would cause no state change are ignored.
    pub fn apply(&mut self, transaction: impl IntoIterator<Item = Operation>) -> SmallVec<[FastPathInputEvent; 2]> {
        let mut events = SmallVec::new();

        for operation in transaction {
            match operation {
                Operation::MouseButtonPressed(button) => {
                    if button.is_unknown() {
                        continue;
                    }

                    let was_pressed = self.mouse_buttons.replace(button.as_idx(), true);

                    if !was_pressed {
                        let event = match MouseButtonFlags::from(button) {
                            MouseButtonFlags::Button(flags) => FastPathInputEvent::MouseEvent(MousePdu {
                                flags: PointerFlags::DOWN | flags,
                                number_of_wheel_rotation_units: 0,
                                x_position: self.mouse_position.x,
                                y_position: self.mouse_position.y,
                            }),
                            MouseButtonFlags::Pointer(flags) => FastPathInputEvent::MouseEventEx(MouseXPdu {
                                flags: PointerXFlags::DOWN | flags,
                                x_position: self.mouse_position.x,
                                y_position: self.mouse_position.y,
                            }),
                        };

                        events.push(event)
                    }
                }
                Operation::MouseButtonReleased(button) => {
                    if button.is_unknown() {
                        continue;
                    }

                    let was_pressed = self.mouse_buttons.replace(button.as_idx(), false);

                    if was_pressed {
                        let event = match MouseButtonFlags::from(button) {
                            MouseButtonFlags::Button(flags) => FastPathInputEvent::MouseEvent(MousePdu {
                                flags,
                                number_of_wheel_rotation_units: 0,
                                x_position: self.mouse_position.x,
                                y_position: self.mouse_position.y,
                            }),
                            MouseButtonFlags::Pointer(flags) => FastPathInputEvent::MouseEventEx(MouseXPdu {
                                flags,
                                x_position: self.mouse_position.x,
                                y_position: self.mouse_position.y,
                            }),
                        };

                        events.push(event)
                    }
                }
                Operation::MouseMove(position) => {
                    if position != self.mouse_position {
                        self.mouse_position = position;
                        events.push(FastPathInputEvent::MouseEvent(MousePdu {
                            flags: PointerFlags::MOVE,
                            number_of_wheel_rotation_units: 0,
                            x_position: position.x,
                            y_position: position.y,
                        }))
                    }
                }
                Operation::WheelRotations(rotations) => events.push(FastPathInputEvent::MouseEvent(MousePdu {
                    flags: if rotations.is_vertical {
                        PointerFlags::VERTICAL_WHEEL
                    } else {
                        PointerFlags::HORIZONTAL_WHEEL
                    },
                    number_of_wheel_rotation_units: rotations.rotation_units,
                    x_position: self.mouse_position.x,
                    y_position: self.mouse_position.y,
                })),
                Operation::KeyPressed(scancode) => {
                    let was_pressed = self.keyboard.replace(scancode.as_idx(), true);

                    let mut flags = KeyboardFlags::empty();

                    if scancode.extended {
                        flags |= KeyboardFlags::EXTENDED
                    };

                    if was_pressed {
                        events.push(FastPathInputEvent::KeyboardEvent(
                            flags | KeyboardFlags::RELEASE,
                            u8::from(scancode),
                        ));
                    }

                    events.push(FastPathInputEvent::KeyboardEvent(flags, u8::from(scancode)));
                }
                Operation::KeyReleased(scancode) => {
                    let was_pressed = self.keyboard.replace(scancode.as_idx(), false);

                    let mut flags = KeyboardFlags::RELEASE;

                    if scancode.extended {
                        flags |= KeyboardFlags::EXTENDED
                    };

                    if was_pressed {
                        events.push(FastPathInputEvent::KeyboardEvent(flags, u8::from(scancode)));
                    }
                }
            }
        }

        events
    }

    /// Releases all keys and buttons. Returns a list of RDP input events to send.
    pub fn release_all(&mut self) -> SmallVec<[FastPathInputEvent; 2]> {
        let mut events = SmallVec::new();

        for idx in self.mouse_buttons.iter_ones() {
            let button_id = u8::try_from(idx).unwrap();

            let event = match MouseButtonFlags::from(MouseButton::from(button_id)) {
                MouseButtonFlags::Button(flags) => FastPathInputEvent::MouseEvent(MousePdu {
                    flags,
                    number_of_wheel_rotation_units: 0,
                    x_position: self.mouse_position.x,
                    y_position: self.mouse_position.y,
                }),
                MouseButtonFlags::Pointer(flags) => FastPathInputEvent::MouseEventEx(MouseXPdu {
                    flags,
                    x_position: self.mouse_position.x,
                    y_position: self.mouse_position.y,
                }),
            };

            events.push(event)
        }

        for idx in self.keyboard.iter_ones() {
            let (scancode, extended) = if idx >= 256 {
                (u8::try_from(idx - 256).unwrap(), true)
            } else {
                (u8::try_from(idx).unwrap(), false)
            };

            let mut flags = KeyboardFlags::RELEASE;

            if extended {
                flags |= KeyboardFlags::EXTENDED
            };

            events.push(FastPathInputEvent::KeyboardEvent(flags, scancode));
        }

        self.mouse_buttons = BitArray::ZERO;
        self.keyboard = BitArray::ZERO;

        events
    }
}

/// Returns the RDP input event to send in order to synchronize lock keys.
pub fn synchronize_event(scroll_lock: bool, num_lock: bool, caps_lock: bool, kana_lock: bool) -> FastPathInputEvent {
    use ironrdp_core::input::fast_path::SynchronizeFlags;

    let mut flags = SynchronizeFlags::empty();

    if scroll_lock {
        flags |= SynchronizeFlags::SCROLL_LOCK;
    }

    if num_lock {
        flags |= SynchronizeFlags::NUM_LOCK;
    }

    if caps_lock {
        flags |= SynchronizeFlags::CAPS_LOCK;
    }

    if kana_lock {
        flags |= SynchronizeFlags::KANA_LOCK;
    }

    FastPathInputEvent::SyncEvent(flags)
}

enum MouseButtonFlags {
    Button(PointerFlags),
    Pointer(PointerXFlags),
}

impl From<MouseButton> for MouseButtonFlags {
    fn from(value: MouseButton) -> Self {
        match value.0 {
            MouseButton::LEFT_VAL => Self::Button(PointerFlags::LEFT_BUTTON),
            MouseButton::MIDDLE_VAL => Self::Button(PointerFlags::MIDDLE_BUTTON_OR_WHEEL),
            MouseButton::RIGHT_VAL => Self::Button(PointerFlags::RIGHT_BUTTON),
            MouseButton::X1_VAL => Self::Pointer(PointerXFlags::BUTTON1),
            MouseButton::X2_VAL => Self::Pointer(PointerXFlags::BUTTON2),
            _ => Self::Button(PointerFlags::empty()),
        }
    }
}