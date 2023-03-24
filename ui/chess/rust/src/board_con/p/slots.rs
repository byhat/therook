pub enum Slots {
    Resync,

    MouseEvent{ slot: MouseEventSlots, piece_size: u32 }
}

pub enum MouseEventSlots {
    Clicked { x: f32, y: f32 },
    Drag(DragSlots)
}

pub enum DragSlots {
    Started {
        src_x: f32,
        src_y: f32,
        dest_x: f32,
        dest_y: f32,
    },
    Updated {
        x: f32,
        y: f32,
    },
    Ended {
        src_x: f32,
        src_y: f32,
        dest_x: f32,
        dest_y: f32,
    },
}
