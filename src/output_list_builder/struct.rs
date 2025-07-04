use crate::*;

/// OutputListBuilder struct
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Code Example
///
/// ## Using the OutputBuilder
///
/// ```rust
/// use color_output::*;
/// OutputListBuilder::new_from(vec![Output::default()])
///     .add(
///         OutputBuilder::new()
///             .text("text")
///             .bg_color(ColorType::Use(Color::Blue))
///             .endl(false)
///             .build(),
///     )
///     .add(Output {
///         text: "test_new_from_output_list_builder_1",
///         color: ColorType::Use(Color::Default),
///         bg_color: ColorType::Color256(0x3f3f3f),
///         endl: false,
///         ..Default::default()
///     })
///     .add(Output {
///         text: "test_new_from_output_list_builder_2",
///         color: ColorType::Use(Color::Default),
///         bg_color: ColorType::Use(Color::Cyan),
///         endl: true,
///         ..Default::default()
///     })
///     .run();
/// ```
#[derive(Debug, Clone)]
pub struct OutputListBuilder<'a> {
    /// A list of output structures.
    pub output_list: Vec<Output<'a>>,
}
