use leptos::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct ChalkSignals {
    pub zoom: SignalPair<u32>,
    pub clear: SignalPair<u32>,

    pub undo: SignalPair<u32>,
    pub redo: SignalPair<u32>,
}

impl ChalkSignals {
    pub fn new() -> Self {
        Self {
            zoom: SignalPair::new(100_u32),
            clear: SignalPair::new(0_u32),
            undo: SignalPair::new(0_u32),
            redo: SignalPair::new(0_u32),
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct SignalPair<T> {
    read: ReadSignal<T>,
    write: WriteSignal<T>,
}

impl<T> SignalPair<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(value: T) -> Self {
        let (r, w) = signal(value);
        Self { read: r, write: w }
    }

    pub fn set(&self, value: T) {
        self.write.set(value);
    }

    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.read.get()
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.write.update(f);
    }

    pub fn read_only(&self) -> ReadSignal<T> {
        self.read
    }

    pub fn write_only(&self) -> WriteSignal<T> {
        self.write
    }
}
