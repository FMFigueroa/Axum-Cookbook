use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Este tipo de alias puede contener un valor exitoso (T) o un error (Error).
pub type Result<T> = core::result::Result<T, Error>;

//Enum de posibles Errors
#[derive(Debug)]
pub enum Error {
    LoginFail,

    // -- Auth errors.
    AuthFailNotAuthtokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RESP");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

/*
                        | Should implement this as well.
                        | (not strickly needed in this course,
                        | but good best practice for error types)
                        V
// region:   --- Error boilerplate
 impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

// se implementa el trait std::error::Error para la enumeración Error.
impl std::error::Error for Error {
    ...
}
//endregion:    --- Error boilerplate

// Se implementa el trait std::fmt::Display para la enumeración Error,
// Este trait permite controlar cómo se formatea y muestra un valor de Error como una cadena de caracteres.
// En la implementación, se utiliza la función write! para formatear el valor self,
// usando el formato de depuración {:?} y escribirlo en el formateador proporcionado.
// Esto permite mostrar el error de manera legible cuando se invoca la función fmt
// en un contexto en el que se requiere una representación de cadena.
*/
