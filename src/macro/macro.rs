/// Output macro
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Parameters
/// - `Output`: The output struct
///
/// # Code Example
///
/// ## Using the Struct
///
/// ```rust
/// use color_output::*;
/// output_macro!(Output {
///     text: "test_proc_macro",
///     color: ColorType::default(),
///     bg_color: ColorType::Use(Color::Yellow),
///     endl: true,
///     ..Default::default()
/// });
/// ```
///
/// ## Using the Constructor
///
/// ```rust
/// use color_output::*;
/// output_macro!(OutputBuilder::new()
///     .text("test_output_builder")
///     .color(ColorType::Use(Color::Cyan))
///     .blod(true)
///     .endl(true)
///     .build());
/// ```
///
/// ## Multiple Inputs
///
/// ```rust
/// use color_output::*;
/// output_macro!(
///     Output {
///         text: "test_proc_macro",
///         color: ColorType::default(),
///         bg_color: ColorType::Use(Color::Yellow),
///         endl: true,
///         ..Default::default()
///     },
///     OutputBuilder::new()
///         .text("test_output_builder1")
///         .color(ColorType::Color256(0xffffff))
///         .blod(true)
///         .endl(true)
///         .build(),
///     OutputBuilder::new()
///         .text("test_output_builder2")
///         .color(ColorType::Color256(0xffffff))
///         .blod(true)
///         .endl(true)
///         .build()
/// );
/// ```
#[macro_export]
macro_rules! output_macro {
    ($($output:expr),*) => {
        $(
            $output.output();
        )*
    };
}

#[macro_export]
macro_rules! print_message_common_handler {
    ($level:expr, $color:expr, $bg_color:expr, $endl:expr, $($data:expr),*) => {{
        use crate::*;
        use std::fmt::*;
        let binding: String = format!("[{} => {}]", time(), $level);
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let time_output: Output<'_> = time_output_builder
            .text(&binding)
            .blod(true)
            .bg_color($bg_color)
            .color(ColorType::Color256(0xffffff))
            .build();
        let mut text_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let mut text_endl_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let text_endl_output: Output<'_> = text_endl_output_builder.text("\n").endl(false).build();
        let mut text: String = String::new();
        $(
            text.push_str(&$data.to_string());
        )*
        let mut lines_iter = text.lines().peekable();
        while let Some(line) = lines_iter.next() {
            let mut output_list_builder = OutputListBuilder::new();
            output_list_builder.add(time_output.clone());
            let text_output: Output<'_> = text_output_builder
                .text(&line)
                .blod(true)
                .color($color)
                .endl(false)
                .build();
            output_list_builder.add(text_output);
            if lines_iter.peek().is_some() || $endl {
                output_list_builder.add(text_endl_output.clone());
            }
            output_list_builder.run();
        }
    }};
}

#[macro_export]
macro_rules! print_success {
    ($($data:expr),*) => {
        crate::print_message_common_handler!("success", ColorType::Rgb(0,255,0), ColorType::Rgb(0,255,0), false, $($data),*);
    };
}

#[macro_export]
macro_rules! print_warning {
    ($($data:expr),*) => {
        crate::print_message_common_handler!("warning", ColorType::Rgb(255, 255, 0), ColorType::Rgb(255, 255, 0), false, $($data),*);
    };
}

#[macro_export]
macro_rules! print_error {
    ($($data:expr),*) => {
        crate::print_message_common_handler!("error", ColorType::Rgb(255,0,0), ColorType::Rgb(255,0,0), false, $($data),*);
    };
}

#[macro_export]
macro_rules! println_success {
    ($($data:expr),*) => {
        crate::print_message_common_handler!("success", ColorType::Rgb(0,255,0), ColorType::Rgb(0,255,0), true, $($data),*);
    };
}

#[macro_export]
macro_rules! println_warning {
    ($($data:expr),*) => {
        crate::print_message_common_handler!("warning", ColorType::Rgb(255, 255, 0), ColorType::Rgb(255, 255, 0), true, $($data),*);
    };
}

#[macro_export]
macro_rules! println_error {
    ($($data:expr),*) => {
        crate::print_message_common_handler!("error", ColorType::Rgb(255,0,0), ColorType::Rgb(255,0,0), true, $($data),*);
    };
}
