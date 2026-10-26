//! `flash.desktop.Clipboard` native methods

use crate::avm2::activation::Activation;
use crate::avm2::error::Error;
use crate::avm2::function::FunctionArgs;
use crate::avm2::parameters::ParametersExt;
use crate::avm2::value::Value;
use crate::avm2::AvmString;

/// Implements `flash.desktop.Clipboard.getData` method.
///
/// Only `TEXT_FORMAT` ("air:text") is supported; all other formats return null.
pub fn get_data<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Value<'gc>,
    args: FunctionArgs<'_, 'gc>,
) -> Result<Value<'gc>, Error<'gc>> {
    let format = args.get_string_non_null(activation, 0, "format")?;

    if format.to_string() == "air:text" {
        let content = activation.context.ui.clipboard_content();
        if content.is_empty() {
            return Ok(Value::Null);
        }
        let avm_string = AvmString::new_utf8(activation.gc(), &content);
        return Ok(avm_string.into());
    }

    Ok(Value::Null)
}

/// Implements `flash.desktop.Clipboard.hasFormat` method.
///
/// Only `TEXT_FORMAT` ("air:text") is supported.
pub fn has_format<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Value<'gc>,
    args: FunctionArgs<'_, 'gc>,
) -> Result<Value<'gc>, Error<'gc>> {
    let format = args.get_string_non_null(activation, 0, "format")?;

    if format.to_string() == "air:text" {
        let available = activation.context.ui.clipboard_available();
        return Ok(available.into());
    }

    Ok(false.into())
}
