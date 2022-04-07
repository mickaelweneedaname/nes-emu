mod bus;
pub mod cpu;
pub mod joypad;
mod random_gen;
pub mod screen;
pub mod traits;
use bus::Bus;
use cpu::{Cpu, PROGRAM_POINTER};
use joypad::{Button, Joypad};
use random_gen::RandomGenerator;
use screen::Screen;
use std::{marker::PhantomPinned, pin::Pin, ptr};
use traits::Memory;

pub enum Player {
    One,
    Two,
}

pub struct Nes {
    memory: [u8; 0xFFFF],
    joypad_1: Joypad,
    joypad_2: Joypad,
    color_generator: RandomGenerator,
    screen: Screen,
    bus: Bus,
    cpu: Cpu,
    _pin: PhantomPinned,
}

impl Nes {
    pub fn new() -> Pin<Box<Self>> {
        let nes = Self {
            memory: [0; 0xFFFF],
            joypad_1: Joypad::new(0x4016),
            joypad_2: Joypad::new(0x4017),
            color_generator: RandomGenerator::new(0x4018, 1..16),
            screen: Screen::default(),
            bus: Bus::new(),
            cpu: Cpu::new(ptr::null_mut::<Bus>()),
            _pin: PhantomPinned,
        };

        let mut pinned_boxed_nes = Box::pin(nes);
        unsafe {
            pinned_boxed_nes.map_devices();
        }
        pinned_boxed_nes
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn press(self: &mut Pin<Box<Self>>, player: Player, button: joypad::Button) {
        self.get_mut_from_pin().player_joypad(player).press(button)
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn release(self: &mut Pin<Box<Self>>, player: Player, button: joypad::Button) {
        self.get_mut_from_pin()
            .player_joypad(player)
            .release(button)
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn run(self: &mut Pin<Box<Self>>) {
        self.get_mut_from_pin().cpu.run();
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn load(self: &mut Pin<Box<Self>>, data: &[u8], dest: u16) {
        let nes = self.get_mut_from_pin();
        nes.bus.load(data, dest);
        nes.bus.mem_write_u16(PROGRAM_POINTER, dest);
        nes.cpu.reset();
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_screen_data(self: &Pin<Box<Self>>) -> Vec<Vec<u8>> {
        self.get_from_pin().screen.get_screen_data()
    }

    pub fn player_joypad(&mut self, player: Player) -> &mut Joypad {
        match player {
            Player::One => &mut self.joypad_1,
            Player::Two => &mut self.joypad_2,
        }
    }

    unsafe fn map_devices(self: &mut Pin<Box<Self>>) {
        let nes_ref = self.get_mut_from_pin();
        nes_ref.bus.map(
            &mut nes_ref.memory,
            &[
                &mut nes_ref.joypad_1,
                &mut nes_ref.joypad_2,
                &mut nes_ref.color_generator,
                &mut nes_ref.screen,
            ],
        );
        nes_ref.cpu.set_mem(&mut nes_ref.bus)
    }

    #[allow(clippy::missing_safety_doc)]
    unsafe fn get_mut_from_pin<'a>(self: &'a mut Pin<Box<Self>>) -> &'a mut Self {
        let pinned_nes_ref: Pin<&mut Self> = Pin::as_mut(self);
        Pin::get_unchecked_mut(pinned_nes_ref)
    }

    #[allow(clippy::missing_safety_doc)]
    unsafe fn get_from_pin<'a>(self: &'a Pin<Box<Self>>) -> &'a Self {
        let pinned_nes_ref: Pin<&Self> = Pin::as_ref(self);
        Pin::get_ref(pinned_nes_ref)
    }
}
