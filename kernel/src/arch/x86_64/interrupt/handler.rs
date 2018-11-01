impl_handler!(divide_by_zero, frame, {
    dump_interrupt_info!("DIVIDE BY ZERO", frame);
    loop{}
});
