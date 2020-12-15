#[macro_export]
/// Codegen for checking for constant temperature boundary conditions
macro_rules! constant_temperature {
    ($self:ident . $varname:ident) => {
        if let Some(temp) = $self.$varname.constant_temperature() {
            return temp;
        }
    };
    ($self:ident . $varname:ident, $self2:ident . $var2:ident) => {
        if let Some(temp) = $self.$varname.constant_temperature() {
            return temp;
        }
        if let Some(temp) = $self2.$var2.constant_temperature() {
            return temp;
        }
    };
    ($self:ident . $varname:ident, $self2:ident . $var2:ident, $self3:ident . $var3:ident) => {
        if let Some(temp) = $self.$varname.constant_temperature() {
            return temp;
        }
        if let Some(temp) = $self2.$var2.constant_temperature() {
            return temp;
        }
        if let Some(temp) = $self3.$var3.constant_temperature() {
            return temp;
        }
    };
}
