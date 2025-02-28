# Input utils
Helper package providing cli operations

* `get_mode() -> Result<Mode, Box<dyn Error>>` - asks for one of mode (client/server) and then returns `Mode` enum
* `get_operation_type() -> Result<InputOperationType, Box<dyn Error>>` - asks for operation type and returns structure `InputOperationType` with provided data and `Operation` enum
* `get_address() -> Result<String, Box<dyn Error>>` - asks for address and then return it as a string, if no address provided it will return default address `localhost:1111`