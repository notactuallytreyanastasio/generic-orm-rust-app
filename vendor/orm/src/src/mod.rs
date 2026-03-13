#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            Ok(())
    }).clone()
}
struct ChangesetErrorStruct {
    field: std::sync::Arc<String>, message: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct ChangesetError(std::sync::Arc<ChangesetErrorStruct>);
#[derive(Clone)]
pub struct ChangesetErrorBuilder {
    pub field: std::sync::Arc<String>, pub message: std::sync::Arc<String>
}
impl ChangesetErrorBuilder {
    pub fn build(self) -> ChangesetError {
        ChangesetError::new(self.field, self.message)
    }
}
impl ChangesetError {
    pub fn new(field__408: impl temper_core::ToArcString, message__409: impl temper_core::ToArcString) -> ChangesetError {
        let field__408 = field__408.to_arc_string();
        let message__409 = message__409.to_arc_string();
        let field;
        let message;
        field = field__408.clone();
        message = message__409.clone();
        let selfish = ChangesetError(std::sync::Arc::new(ChangesetErrorStruct {
                    field, message
        }));
        return selfish;
    }
    pub fn field(& self) -> std::sync::Arc<String> {
        return self.0.field.clone();
    }
    pub fn message(& self) -> std::sync::Arc<String> {
        return self.0.message.clone();
    }
}
temper_core::impl_any_value_trait!(ChangesetError, []);
pub enum ChangesetEnum {
    ChangesetImpl(ChangesetImpl)
}
pub trait ChangesetTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> ChangesetEnum;
    fn clone_boxed(& self) -> Changeset;
    fn table_def(& self) -> TableDef;
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
    fn errors(& self) -> temper_core::List<ChangesetError>;
    fn is_valid(& self) -> bool;
    fn cast(& self, allowedFields__419: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__422: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__425: SafeIdentifier, min__426: i32, max__427: i32) -> Changeset;
    fn validate_int(& self, field__430: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__433: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__436: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__439: SafeIdentifier) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__444: i32) -> temper_core::Result<SqlFragment>;
}
#[derive(Clone)]
pub struct Changeset(std::sync::Arc<dyn ChangesetTrait>);
impl Changeset {
    pub fn new(selfish: impl ChangesetTrait + 'static) -> Changeset {
        Changeset(std::sync::Arc::new(selfish))
    }
}
impl ChangesetTrait for Changeset {
    fn as_enum(& self) -> ChangesetEnum {
        ChangesetTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> Changeset {
        ChangesetTrait::clone_boxed( & ( * self.0))
    }
    fn table_def(& self) -> TableDef {
        ChangesetTrait::table_def( & ( * self.0))
    }
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        ChangesetTrait::changes( & ( * self.0))
    }
    fn errors(& self) -> temper_core::List<ChangesetError> {
        ChangesetTrait::errors( & ( * self.0))
    }
    fn is_valid(& self) -> bool {
        ChangesetTrait::is_valid( & ( * self.0))
    }
    fn cast(& self, arg1: temper_core::List<SafeIdentifier>) -> Changeset {
        ChangesetTrait::cast( & ( * self.0), arg1)
    }
    fn validate_required(& self, arg1: temper_core::List<SafeIdentifier>) -> Changeset {
        ChangesetTrait::validate_required( & ( * self.0), arg1)
    }
    fn validate_length(& self, arg1: SafeIdentifier, arg2: i32, arg3: i32) -> Changeset {
        ChangesetTrait::validate_length( & ( * self.0), arg1, arg2, arg3)
    }
    fn validate_int(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_int( & ( * self.0), arg1)
    }
    fn validate_int64(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_int64( & ( * self.0), arg1)
    }
    fn validate_float(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_float( & ( * self.0), arg1)
    }
    fn validate_bool(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_bool( & ( * self.0), arg1)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        ChangesetTrait::to_insert_sql( & ( * self.0))
    }
    fn to_update_sql(& self, arg1: i32) -> temper_core::Result<SqlFragment> {
        ChangesetTrait::to_update_sql( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(Changeset);
impl std::ops::Deref for Changeset {
    type Target = dyn ChangesetTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ChangesetImplStruct {
    table_def: TableDef, params: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, changes: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, errors: temper_core::List<ChangesetError>, is_valid: bool
}
#[derive(Clone)]
pub (crate) struct ChangesetImpl(std::sync::Arc<ChangesetImplStruct>);
impl ChangesetImpl {
    pub fn table_def(& self) -> TableDef {
        return self.0.table_def.clone();
    }
    pub fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        return self.0.changes.clone();
    }
    pub fn errors(& self) -> temper_core::List<ChangesetError> {
        return self.0.errors.clone();
    }
    pub fn is_valid(& self) -> bool {
        return self.0.is_valid;
    }
    pub fn cast(& self, allowedFields__460: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__460 = allowedFields__460.to_list();
        let mb__462: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__134: ChangesetImpl, mb__462: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__7132(& self, f__463: SafeIdentifier) {
                let mut t___7130: std::sync::Arc<String>;
                let mut t___7127: std::sync::Arc<String> = f__463.sql_value();
                let val__464: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__134.0.params, t___7127.clone(), std::sync::Arc::new("".to_string()));
                if ! val__464.is_empty() {
                    t___7130 = f__463.sql_value();
                    temper_core::MapBuilder::set( & self.mb__462, t___7130.clone(), val__464.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__134: self.clone(), mb__462: mb__462.clone()
        };
        let fn__7132 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__463: SafeIdentifier | closure_group.fn__7132(f__463))
        };
        temper_core::listed::list_for_each( & allowedFields__460, & ( * fn__7132.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__462), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__466: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__466 = fields__466.to_list();
        let return__245: Changeset;
        let mut t___7125: temper_core::List<ChangesetError>;
        let mut t___4163: TableDef;
        let mut t___4164: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4165: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__467: {
            if ! self.0.is_valid {
                return__245 = Changeset::new(self.clone());
                break 'fn__467;
            }
            let eb__468: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__469: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__135: ChangesetImpl, eb__468: temper_core::ListBuilder<ChangesetError>, valid__469: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__7121(& self, f__470: SafeIdentifier) {
                    let mut t___7119: ChangesetError;
                    let mut t___7116: std::sync::Arc<String> = f__470.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__135.0.changes, t___7116.clone()) {
                        t___7119 = ChangesetError::new(f__470.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__468, t___7119.clone(), None);
                        {
                            * self.valid__469.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__135: self.clone(), eb__468: eb__468.clone(), valid__469: valid__469.clone()
            };
            let fn__7121 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__470: SafeIdentifier | closure_group.fn__7121(f__470))
            };
            temper_core::listed::list_for_each( & fields__466, & ( * fn__7121.clone()));
            t___4163 = self.0.table_def.clone();
            t___4164 = self.0.params.clone();
            t___4165 = self.0.changes.clone();
            t___7125 = temper_core::ListedTrait::to_list( & eb__468);
            return__245 = Changeset::new(ChangesetImpl::new(t___4163.clone(), t___4164.clone(), t___4165.clone(), t___7125.clone(), temper_core::read_locked( & valid__469)));
        }
        return return__245.clone();
    }
    pub fn validate_length(& self, field__472: SafeIdentifier, min__473: i32, max__474: i32) -> Changeset {
        let return__246: Changeset;
        let mut t___7103: std::sync::Arc<String>;
        let mut t___7114: temper_core::List<ChangesetError>;
        let mut t___4146: bool;
        let mut t___4152: TableDef;
        let mut t___4153: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4154: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__475: {
            if ! self.0.is_valid {
                return__246 = Changeset::new(self.clone());
                break 'fn__475;
            }
            t___7103 = field__472.sql_value();
            let val__476: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___7103.clone(), std::sync::Arc::new("".to_string()));
            let len__477: i32 = temper_core::string::count_between( & val__476, 0usize, val__476.len());
            if Some(len__477) < Some(min__473) {
                t___4146 = true;
            } else {
                t___4146 = Some(len__477) > Some(max__474);
            }
            if t___4146 {
                let msg__478: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__473, max__474));
                let eb__479: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__479, ChangesetError::new(field__472.sql_value(), msg__478.clone()), None);
                t___4152 = self.0.table_def.clone();
                t___4153 = self.0.params.clone();
                t___4154 = self.0.changes.clone();
                t___7114 = temper_core::ListedTrait::to_list( & eb__479);
                return__246 = Changeset::new(ChangesetImpl::new(t___4152.clone(), t___4153.clone(), t___4154.clone(), t___7114.clone(), false));
                break 'fn__475;
            }
            return__246 = Changeset::new(self.clone());
        }
        return return__246.clone();
    }
    pub fn validate_int(& self, field__481: SafeIdentifier) -> Changeset {
        let return__247: Changeset;
        let mut t___7094: std::sync::Arc<String>;
        let mut t___7101: temper_core::List<ChangesetError>;
        let mut t___4137: TableDef;
        let mut t___4138: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4139: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__482: {
            if ! self.0.is_valid {
                return__247 = Changeset::new(self.clone());
                break 'fn__482;
            }
            t___7094 = field__481.sql_value();
            let val__483: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___7094.clone(), std::sync::Arc::new("".to_string()));
            if val__483.is_empty() {
                return__247 = Changeset::new(self.clone());
                break 'fn__482;
            }
            let parseOk__484: bool;
            'ok___7237: {
                'orelse___1407: {
                    match temper_core::string::to_int( & val__483, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1407
                    };
                    parseOk__484 = true;
                    break 'ok___7237;
                }
                parseOk__484 = false;
            }
            if ! parseOk__484 {
                let eb__485: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__485, ChangesetError::new(field__481.sql_value(), "must be an integer"), None);
                t___4137 = self.0.table_def.clone();
                t___4138 = self.0.params.clone();
                t___4139 = self.0.changes.clone();
                t___7101 = temper_core::ListedTrait::to_list( & eb__485);
                return__247 = Changeset::new(ChangesetImpl::new(t___4137.clone(), t___4138.clone(), t___4139.clone(), t___7101.clone(), false));
                break 'fn__482;
            }
            return__247 = Changeset::new(self.clone());
        }
        return return__247.clone();
    }
    pub fn validate_int64(& self, field__487: SafeIdentifier) -> Changeset {
        let return__248: Changeset;
        let mut t___7085: std::sync::Arc<String>;
        let mut t___7092: temper_core::List<ChangesetError>;
        let mut t___4124: TableDef;
        let mut t___4125: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4126: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__488: {
            if ! self.0.is_valid {
                return__248 = Changeset::new(self.clone());
                break 'fn__488;
            }
            t___7085 = field__487.sql_value();
            let val__489: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___7085.clone(), std::sync::Arc::new("".to_string()));
            if val__489.is_empty() {
                return__248 = Changeset::new(self.clone());
                break 'fn__488;
            }
            let parseOk__490: bool;
            'ok___7239: {
                'orelse___1408: {
                    match temper_core::string::to_int64( & val__489, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1408
                    };
                    parseOk__490 = true;
                    break 'ok___7239;
                }
                parseOk__490 = false;
            }
            if ! parseOk__490 {
                let eb__491: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__491, ChangesetError::new(field__487.sql_value(), "must be a 64-bit integer"), None);
                t___4124 = self.0.table_def.clone();
                t___4125 = self.0.params.clone();
                t___4126 = self.0.changes.clone();
                t___7092 = temper_core::ListedTrait::to_list( & eb__491);
                return__248 = Changeset::new(ChangesetImpl::new(t___4124.clone(), t___4125.clone(), t___4126.clone(), t___7092.clone(), false));
                break 'fn__488;
            }
            return__248 = Changeset::new(self.clone());
        }
        return return__248.clone();
    }
    pub fn validate_float(& self, field__493: SafeIdentifier) -> Changeset {
        let return__249: Changeset;
        let mut t___7076: std::sync::Arc<String>;
        let mut t___7083: temper_core::List<ChangesetError>;
        let mut t___4111: TableDef;
        let mut t___4112: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4113: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__494: {
            if ! self.0.is_valid {
                return__249 = Changeset::new(self.clone());
                break 'fn__494;
            }
            t___7076 = field__493.sql_value();
            let val__495: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___7076.clone(), std::sync::Arc::new("".to_string()));
            if val__495.is_empty() {
                return__249 = Changeset::new(self.clone());
                break 'fn__494;
            }
            let parseOk__496: bool;
            'ok___7241: {
                'orelse___1409: {
                    match temper_core::string::to_float64( & val__495) {
                        Ok(x) => x,
                        _ => break 'orelse___1409
                    };
                    parseOk__496 = true;
                    break 'ok___7241;
                }
                parseOk__496 = false;
            }
            if ! parseOk__496 {
                let eb__497: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__497, ChangesetError::new(field__493.sql_value(), "must be a number"), None);
                t___4111 = self.0.table_def.clone();
                t___4112 = self.0.params.clone();
                t___4113 = self.0.changes.clone();
                t___7083 = temper_core::ListedTrait::to_list( & eb__497);
                return__249 = Changeset::new(ChangesetImpl::new(t___4111.clone(), t___4112.clone(), t___4113.clone(), t___7083.clone(), false));
                break 'fn__494;
            }
            return__249 = Changeset::new(self.clone());
        }
        return return__249.clone();
    }
    pub fn validate_bool(& self, field__499: SafeIdentifier) -> Changeset {
        let return__250: Changeset;
        let mut t___7067: std::sync::Arc<String>;
        let mut t___7074: temper_core::List<ChangesetError>;
        let mut t___4086: bool;
        let mut t___4087: bool;
        let mut t___4089: bool;
        let mut t___4090: bool;
        let mut t___4092: bool;
        let mut t___4098: TableDef;
        let mut t___4099: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4100: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__500: {
            if ! self.0.is_valid {
                return__250 = Changeset::new(self.clone());
                break 'fn__500;
            }
            t___7067 = field__499.sql_value();
            let val__501: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___7067.clone(), std::sync::Arc::new("".to_string()));
            if val__501.is_empty() {
                return__250 = Changeset::new(self.clone());
                break 'fn__500;
            }
            let isTrue__502: bool;
            if Some(val__501.as_str()) == Some("true") {
                isTrue__502 = true;
            } else {
                if Some(val__501.as_str()) == Some("1") {
                    t___4087 = true;
                } else {
                    if Some(val__501.as_str()) == Some("yes") {
                        t___4086 = true;
                    } else {
                        t___4086 = Some(val__501.as_str()) == Some("on");
                    }
                    t___4087 = t___4086;
                }
                isTrue__502 = t___4087;
            }
            let isFalse__503: bool;
            if Some(val__501.as_str()) == Some("false") {
                isFalse__503 = true;
            } else {
                if Some(val__501.as_str()) == Some("0") {
                    t___4090 = true;
                } else {
                    if Some(val__501.as_str()) == Some("no") {
                        t___4089 = true;
                    } else {
                        t___4089 = Some(val__501.as_str()) == Some("off");
                    }
                    t___4090 = t___4089;
                }
                isFalse__503 = t___4090;
            }
            if ! isTrue__502 {
                t___4092 = ! isFalse__503;
            } else {
                t___4092 = false;
            }
            if t___4092 {
                let eb__504: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__504, ChangesetError::new(field__499.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___4098 = self.0.table_def.clone();
                t___4099 = self.0.params.clone();
                t___4100 = self.0.changes.clone();
                t___7074 = temper_core::ListedTrait::to_list( & eb__504);
                return__250 = Changeset::new(ChangesetImpl::new(t___4098.clone(), t___4099.clone(), t___4100.clone(), t___7074.clone(), false));
                break 'fn__500;
            }
            return__250 = Changeset::new(self.clone());
        }
        return return__250.clone();
    }
    fn parse_bool_sql_part(& self, val__506: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__506 = val__506.to_arc_string();
        let return__251: SqlBoolean;
        let mut t___4075: bool;
        let mut t___4076: bool;
        let mut t___4077: bool;
        let mut t___4079: bool;
        let mut t___4080: bool;
        let mut t___4081: bool;
        'fn__507: {
            if Some(val__506.as_str()) == Some("true") {
                t___4077 = true;
            } else {
                if Some(val__506.as_str()) == Some("1") {
                    t___4076 = true;
                } else {
                    if Some(val__506.as_str()) == Some("yes") {
                        t___4075 = true;
                    } else {
                        t___4075 = Some(val__506.as_str()) == Some("on");
                    }
                    t___4076 = t___4075;
                }
                t___4077 = t___4076;
            }
            if t___4077 {
                return__251 = SqlBoolean::new(true);
                break 'fn__507;
            }
            if Some(val__506.as_str()) == Some("false") {
                t___4081 = true;
            } else {
                if Some(val__506.as_str()) == Some("0") {
                    t___4080 = true;
                } else {
                    if Some(val__506.as_str()) == Some("no") {
                        t___4079 = true;
                    } else {
                        t___4079 = Some(val__506.as_str()) == Some("off");
                    }
                    t___4080 = t___4079;
                }
                t___4081 = t___4080;
            }
            if t___4081 {
                return__251 = SqlBoolean::new(false);
                break 'fn__507;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__251.clone());
    }
    fn value_to_sql_part(& self, fieldDef__509: FieldDef, val__510: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__510 = val__510.to_arc_string();
        let return__252: SqlPart;
        let mut t___4062: i32;
        let mut t___4065: i64;
        let mut t___4068: f64;
        let mut t___4073: temper_std::temporal::Date;
        'fn__511: {
            let ft__512: FieldType = fieldDef__509.field_type();
            if temper_core::is::<StringField>(ft__512.clone()) {
                return__252 = SqlPart::new(SqlString::new(val__510.clone()));
                break 'fn__511;
            }
            if temper_core::is::<IntField>(ft__512.clone()) {
                t___4062 = temper_core::string::to_int( & val__510, None) ? ;
                return__252 = SqlPart::new(SqlInt32::new(t___4062));
                break 'fn__511;
            }
            if temper_core::is::<Int64Field>(ft__512.clone()) {
                t___4065 = temper_core::string::to_int64( & val__510, None) ? ;
                return__252 = SqlPart::new(SqlInt64::new(t___4065));
                break 'fn__511;
            }
            if temper_core::is::<FloatField>(ft__512.clone()) {
                t___4068 = temper_core::string::to_float64( & val__510) ? ;
                return__252 = SqlPart::new(SqlFloat64::new(t___4068));
                break 'fn__511;
            }
            if temper_core::is::<BoolField>(ft__512.clone()) {
                return__252 = SqlPart::new(self.parse_bool_sql_part(val__510.clone()) ? );
                break 'fn__511;
            }
            if temper_core::is::<DateField>(ft__512.clone()) {
                t___4073 = temper_std::temporal::Date::from_iso_string(val__510.clone()) ? ;
                return__252 = SqlPart::new(SqlDate::new(t___4073.clone()));
                break 'fn__511;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__252.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___7015: i32;
        let mut t___7020: std::sync::Arc<String>;
        let mut t___7021: bool;
        let mut t___7026: i32;
        let mut t___7028: std::sync::Arc<String>;
        let mut t___7032: std::sync::Arc<String>;
        let mut t___7047: i32;
        let mut t___4026: bool;
        let mut t___4034: FieldDef;
        let mut t___4039: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__515: i32 = 0;
        'loop___7302: loop {
            t___7015 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__515) < Some(t___7015)) {
                break;
            }
            let f__516: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__515);
            if ! f__516.nullable() {
                t___7020 = f__516.name().sql_value();
                t___7021 = temper_core::MappedTrait::has( & self.0.changes, t___7020.clone());
                t___4026 = ! t___7021;
            } else {
                t___4026 = false;
            }
            if t___4026 {
                return Err(temper_core::Error::new());
            }
            i__515 = i__515.wrapping_add(1);
        }
        let pairs__517: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__517)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__518: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__519: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__520: i32 = 0;
        'loop___7303: loop {
            t___7026 = temper_core::ListedTrait::len( & pairs__517);
            if ! (Some(i__520) < Some(t___7026)) {
                break;
            }
            let pair__521: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__517, i__520);
            t___7028 = pair__521.key();
            t___4034 = self.0.table_def.field(t___7028.clone()) ? ;
            let fd__522: FieldDef = t___4034.clone();
            temper_core::listed::add( & colNames__518, fd__522.name().sql_value(), None);
            t___7032 = pair__521.value();
            t___4039 = self.value_to_sql_part(fd__522.clone(), t___7032.clone()) ? ;
            temper_core::listed::add( & valParts__519, t___4039.clone(), None);
            i__520 = i__520.wrapping_add(1);
        }
        let b__523: SqlBuilder = SqlBuilder::new();
        b__523.append_safe("INSERT INTO ");
        b__523.append_safe(self.0.table_def.table_name().sql_value());
        b__523.append_safe(" (");
        let mut t___7040: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__518);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__7013(& self, c__524: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__524 = c__524.to_arc_string();
                return c__524.clone();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__7013 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__524: std::sync::Arc<String> | closure_group.fn__7013(c__524))
        };
        b__523.append_safe(temper_core::listed::join( & t___7040, std::sync::Arc::new(", ".to_string()), & ( * fn__7013.clone())));
        b__523.append_safe(") VALUES (");
        b__523.append_part(temper_core::ListedTrait::get( & valParts__519, 0));
        let mut j__525: i32 = 1;
        'loop___7304: loop {
            t___7047 = temper_core::ListedTrait::len( & valParts__519);
            if ! (Some(j__525) < Some(t___7047)) {
                break;
            }
            b__523.append_safe(", ");
            b__523.append_part(temper_core::ListedTrait::get( & valParts__519, j__525));
            j__525 = j__525.wrapping_add(1);
        }
        b__523.append_safe(")");
        return Ok(b__523.accumulated());
    }
    pub fn to_update_sql(& self, id__527: i32) -> temper_core::Result<SqlFragment> {
        let mut t___7000: i32;
        let mut t___7003: std::sync::Arc<String>;
        let mut t___7008: std::sync::Arc<String>;
        let mut t___4007: FieldDef;
        let mut t___4013: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__529: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__529)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__530: SqlBuilder = SqlBuilder::new();
        b__530.append_safe("UPDATE ");
        b__530.append_safe(self.0.table_def.table_name().sql_value());
        b__530.append_safe(" SET ");
        let mut i__531: i32 = 0;
        'loop___7305: loop {
            t___7000 = temper_core::ListedTrait::len( & pairs__529);
            if ! (Some(i__531) < Some(t___7000)) {
                break;
            }
            if Some(i__531) > Some(0) {
                b__530.append_safe(", ");
            }
            let pair__532: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__529, i__531);
            t___7003 = pair__532.key();
            t___4007 = self.0.table_def.field(t___7003.clone()) ? ;
            let fd__533: FieldDef = t___4007.clone();
            b__530.append_safe(fd__533.name().sql_value());
            b__530.append_safe(" = ");
            t___7008 = pair__532.value();
            t___4013 = self.value_to_sql_part(fd__533.clone(), t___7008.clone()) ? ;
            b__530.append_part(t___4013.clone());
            i__531 = i__531.wrapping_add(1);
        }
        b__530.append_safe(" WHERE id = ");
        b__530.append_int32(id__527);
        return Ok(b__530.accumulated());
    }
    pub fn new(_tableDef__535: TableDef, _params__536: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__537: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__538: impl temper_core::ToList<ChangesetError>, _isValid__539: bool) -> ChangesetImpl {
        let _errors__538 = _errors__538.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__535.clone();
        params = _params__536.clone();
        changes = _changes__537.clone();
        errors = _errors__538.clone();
        is_valid = _isValid__539;
        let selfish = ChangesetImpl(std::sync::Arc::new(ChangesetImplStruct {
                    table_def, params, changes, errors, is_valid
        }));
        return selfish;
    }
}
impl ChangesetTrait for ChangesetImpl {
    fn as_enum(& self) -> ChangesetEnum {
        ChangesetEnum::ChangesetImpl(self.clone())
    }
    fn clone_boxed(& self) -> Changeset {
        Changeset::new(self.clone())
    }
    fn table_def(& self) -> TableDef {
        self.table_def()
    }
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        self.changes()
    }
    fn errors(& self) -> temper_core::List<ChangesetError> {
        self.errors()
    }
    fn is_valid(& self) -> bool {
        self.is_valid()
    }
    fn cast(& self, allowedFields__460: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__460)
    }
    fn validate_required(& self, fields__466: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__466)
    }
    fn validate_length(& self, field__472: SafeIdentifier, min__473: i32, max__474: i32) -> Changeset {
        self.validate_length(field__472, min__473, max__474)
    }
    fn validate_int(& self, field__481: SafeIdentifier) -> Changeset {
        self.validate_int(field__481)
    }
    fn validate_int64(& self, field__487: SafeIdentifier) -> Changeset {
        self.validate_int64(field__487)
    }
    fn validate_float(& self, field__493: SafeIdentifier) -> Changeset {
        self.validate_float(field__493)
    }
    fn validate_bool(& self, field__499: SafeIdentifier) -> Changeset {
        self.validate_bool(field__499)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__527: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__527)
    }
}
temper_core::impl_any_value_trait!(ChangesetImpl, [Changeset]);
pub enum JoinTypeEnum {
    InnerJoin(InnerJoin), LeftJoin(LeftJoin), RightJoin(RightJoin), FullJoin(FullJoin)
}
pub trait JoinTypeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> JoinTypeEnum;
    fn clone_boxed(& self) -> JoinType;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct JoinType(std::sync::Arc<dyn JoinTypeTrait>);
impl JoinType {
    pub fn new(selfish: impl JoinTypeTrait + 'static) -> JoinType {
        JoinType(std::sync::Arc::new(selfish))
    }
}
impl JoinTypeTrait for JoinType {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> JoinType {
        JoinTypeTrait::clone_boxed( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        JoinTypeTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(JoinType);
impl std::ops::Deref for JoinType {
    type Target = dyn JoinTypeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct InnerJoinStruct {}
#[derive(Clone)]
pub struct InnerJoin(std::sync::Arc<InnerJoinStruct>);
impl InnerJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("INNER JOIN".to_string());
    }
    pub fn new() -> InnerJoin {
        let selfish = InnerJoin(std::sync::Arc::new(InnerJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for InnerJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::InnerJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(InnerJoin, [JoinType]);
struct LeftJoinStruct {}
#[derive(Clone)]
pub struct LeftJoin(std::sync::Arc<LeftJoinStruct>);
impl LeftJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("LEFT JOIN".to_string());
    }
    pub fn new() -> LeftJoin {
        let selfish = LeftJoin(std::sync::Arc::new(LeftJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for LeftJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::LeftJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(LeftJoin, [JoinType]);
struct RightJoinStruct {}
#[derive(Clone)]
pub struct RightJoin(std::sync::Arc<RightJoinStruct>);
impl RightJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("RIGHT JOIN".to_string());
    }
    pub fn new() -> RightJoin {
        let selfish = RightJoin(std::sync::Arc::new(RightJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for RightJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::RightJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(RightJoin, [JoinType]);
struct FullJoinStruct {}
#[derive(Clone)]
pub struct FullJoin(std::sync::Arc<FullJoinStruct>);
impl FullJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("FULL OUTER JOIN".to_string());
    }
    pub fn new() -> FullJoin {
        let selfish = FullJoin(std::sync::Arc::new(FullJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for FullJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::FullJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(FullJoin, [JoinType]);
struct JoinClauseStruct {
    join_type: JoinType, table: SafeIdentifier, on_condition: SqlFragment
}
#[derive(Clone)]
pub struct JoinClause(std::sync::Arc<JoinClauseStruct>);
#[derive(Clone)]
pub struct JoinClauseBuilder {
    pub join_type: JoinType, pub table: SafeIdentifier, pub on_condition: SqlFragment
}
impl JoinClauseBuilder {
    pub fn build(self) -> JoinClause {
        JoinClause::new(self.join_type, self.table, self.on_condition)
    }
}
impl JoinClause {
    pub fn new(joinType__652: JoinType, table__653: SafeIdentifier, onCondition__654: SqlFragment) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__652.clone();
        table = table__653.clone();
        on_condition = onCondition__654.clone();
        let selfish = JoinClause(std::sync::Arc::new(JoinClauseStruct {
                    join_type, table, on_condition
        }));
        return selfish;
    }
    pub fn join_type(& self) -> JoinType {
        return self.0.join_type.clone();
    }
    pub fn table(& self) -> SafeIdentifier {
        return self.0.table.clone();
    }
    pub fn on_condition(& self) -> SqlFragment {
        return self.0.on_condition.clone();
    }
}
temper_core::impl_any_value_trait!(JoinClause, []);
struct OrderClauseStruct {
    field: SafeIdentifier, ascending: bool
}
#[derive(Clone)]
pub struct OrderClause(std::sync::Arc<OrderClauseStruct>);
#[derive(Clone)]
pub struct OrderClauseBuilder {
    pub field: SafeIdentifier, pub ascending: bool
}
impl OrderClauseBuilder {
    pub fn build(self) -> OrderClause {
        OrderClause::new(self.field, self.ascending)
    }
}
impl OrderClause {
    pub fn new(field__658: SafeIdentifier, ascending__659: bool) -> OrderClause {
        let field;
        let ascending;
        field = field__658.clone();
        ascending = ascending__659;
        let selfish = OrderClause(std::sync::Arc::new(OrderClauseStruct {
                    field, ascending
        }));
        return selfish;
    }
    pub fn field(& self) -> SafeIdentifier {
        return self.0.field.clone();
    }
    pub fn ascending(& self) -> bool {
        return self.0.ascending;
    }
}
temper_core::impl_any_value_trait!(OrderClause, []);
pub enum WhereClauseEnum {
    AndCondition(AndCondition), OrCondition(OrCondition)
}
pub trait WhereClauseTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> WhereClauseEnum;
    fn clone_boxed(& self) -> WhereClause;
    fn condition(& self) -> SqlFragment;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct WhereClause(std::sync::Arc<dyn WhereClauseTrait>);
impl WhereClause {
    pub fn new(selfish: impl WhereClauseTrait + 'static) -> WhereClause {
        WhereClause(std::sync::Arc::new(selfish))
    }
}
impl WhereClauseTrait for WhereClause {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClauseTrait::clone_boxed( & ( * self.0))
    }
    fn condition(& self) -> SqlFragment {
        WhereClauseTrait::condition( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        WhereClauseTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(WhereClause);
impl std::ops::Deref for WhereClause {
    type Target = dyn WhereClauseTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct AndConditionStruct {
    condition: SqlFragment
}
#[derive(Clone)]
pub struct AndCondition(std::sync::Arc<AndConditionStruct>);
impl AndCondition {
    pub fn condition(& self) -> SqlFragment {
        return self.0.condition.clone();
    }
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("AND".to_string());
    }
    pub fn new(_condition__670: SqlFragment) -> AndCondition {
        let condition;
        condition = _condition__670.clone();
        let selfish = AndCondition(std::sync::Arc::new(AndConditionStruct {
                    condition
        }));
        return selfish;
    }
}
impl WhereClauseTrait for AndCondition {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseEnum::AndCondition(self.clone())
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClause::new(self.clone())
    }
    fn condition(& self) -> SqlFragment {
        self.condition()
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(AndCondition, [WhereClause]);
struct OrConditionStruct {
    condition: SqlFragment
}
#[derive(Clone)]
pub struct OrCondition(std::sync::Arc<OrConditionStruct>);
impl OrCondition {
    pub fn condition(& self) -> SqlFragment {
        return self.0.condition.clone();
    }
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("OR".to_string());
    }
    pub fn new(_condition__677: SqlFragment) -> OrCondition {
        let condition;
        condition = _condition__677.clone();
        let selfish = OrCondition(std::sync::Arc::new(OrConditionStruct {
                    condition
        }));
        return selfish;
    }
}
impl WhereClauseTrait for OrCondition {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseEnum::OrCondition(self.clone())
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClause::new(self.clone())
    }
    fn condition(& self) -> SqlFragment {
        self.condition()
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(OrCondition, [WhereClause]);
struct QueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<WhereClause>, selected_fields: temper_core::List<SafeIdentifier>, order_clauses: temper_core::List<OrderClause>, limit_val: Option<i32>, offset_val: Option<i32>, join_clauses: temper_core::List<JoinClause>
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<QueryStruct>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<WhereClause>, pub selected_fields: temper_core::List<SafeIdentifier>, pub order_clauses: temper_core::List<OrderClause>, pub limit_val: Option<i32>, pub offset_val: Option<i32>, pub join_clauses: temper_core::List<JoinClause>
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.table_name, self.conditions, self.selected_fields, self.order_clauses, self.limit_val, self.offset_val, self.join_clauses)
    }
}
impl Query {
    pub fn r#where(& self, condition__686: SqlFragment) -> Query {
        let nb__688: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__688, WhereClause::new(AndCondition::new(condition__686.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__688), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone());
    }
    pub fn or_where(& self, condition__690: SqlFragment) -> Query {
        let nb__692: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__692, WhereClause::new(OrCondition::new(condition__690.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__692), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone());
    }
    pub fn where_null(& self, field__694: SafeIdentifier) -> Query {
        let b__696: SqlBuilder = SqlBuilder::new();
        b__696.append_safe(field__694.sql_value());
        b__696.append_safe(" IS NULL");
        let mut t___6599: SqlFragment = b__696.accumulated();
        return self.r#where(t___6599.clone());
    }
    pub fn where_not_null(& self, field__698: SafeIdentifier) -> Query {
        let b__700: SqlBuilder = SqlBuilder::new();
        b__700.append_safe(field__698.sql_value());
        b__700.append_safe(" IS NOT NULL");
        let mut t___6593: SqlFragment = b__700.accumulated();
        return self.r#where(t___6593.clone());
    }
    pub fn where_in(& self, field__702: SafeIdentifier, values__703: impl temper_core::ToList<SqlPart>) -> Query {
        let values__703 = values__703.to_list();
        let return__301: Query;
        let mut t___6574: SqlFragment;
        let mut t___6582: i32;
        let mut t___6587: SqlFragment;
        'fn__704: {
            if temper_core::ListedTrait::is_empty( & values__703) {
                let b__705: SqlBuilder = SqlBuilder::new();
                b__705.append_safe("1 = 0");
                t___6574 = b__705.accumulated();
                return__301 = self.r#where(t___6574.clone());
                break 'fn__704;
            }
            let b__706: SqlBuilder = SqlBuilder::new();
            b__706.append_safe(field__702.sql_value());
            b__706.append_safe(" IN (");
            b__706.append_part(temper_core::ListedTrait::get( & values__703, 0));
            let mut i__707: i32 = 1;
            'loop___7310: loop {
                t___6582 = temper_core::ListedTrait::len( & values__703);
                if ! (Some(i__707) < Some(t___6582)) {
                    break;
                }
                b__706.append_safe(", ");
                b__706.append_part(temper_core::ListedTrait::get( & values__703, i__707));
                i__707 = i__707.wrapping_add(1);
            }
            b__706.append_safe(")");
            t___6587 = b__706.accumulated();
            return__301 = self.r#where(t___6587.clone());
        }
        return return__301.clone();
    }
    pub fn where_not(& self, condition__709: SqlFragment) -> Query {
        let b__711: SqlBuilder = SqlBuilder::new();
        b__711.append_safe("NOT (");
        b__711.append_fragment(condition__709.clone());
        b__711.append_safe(")");
        let mut t___6569: SqlFragment = b__711.accumulated();
        return self.r#where(t___6569.clone());
    }
    pub fn where_between(& self, field__713: SafeIdentifier, low__714: SqlPart, high__715: SqlPart) -> Query {
        let b__717: SqlBuilder = SqlBuilder::new();
        b__717.append_safe(field__713.sql_value());
        b__717.append_safe(" BETWEEN ");
        b__717.append_part(low__714.clone());
        b__717.append_safe(" AND ");
        b__717.append_part(high__715.clone());
        let mut t___6563: SqlFragment = b__717.accumulated();
        return self.r#where(t___6563.clone());
    }
    pub fn where_like(& self, field__719: SafeIdentifier, pattern__720: impl temper_core::ToArcString) -> Query {
        let pattern__720 = pattern__720.to_arc_string();
        let b__722: SqlBuilder = SqlBuilder::new();
        b__722.append_safe(field__719.sql_value());
        b__722.append_safe(" LIKE ");
        b__722.append_string(pattern__720.clone());
        let mut t___6554: SqlFragment = b__722.accumulated();
        return self.r#where(t___6554.clone());
    }
    pub fn where_i_like(& self, field__724: SafeIdentifier, pattern__725: impl temper_core::ToArcString) -> Query {
        let pattern__725 = pattern__725.to_arc_string();
        let b__727: SqlBuilder = SqlBuilder::new();
        b__727.append_safe(field__724.sql_value());
        b__727.append_safe(" ILIKE ");
        b__727.append_string(pattern__725.clone());
        let mut t___6547: SqlFragment = b__727.accumulated();
        return self.r#where(t___6547.clone());
    }
    pub fn select(& self, fields__729: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__729 = fields__729.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__729.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone());
    }
    pub fn order_by(& self, field__732: SafeIdentifier, ascending__733: bool) -> Query {
        let nb__735: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__735, OrderClause::new(field__732.clone(), ascending__733), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__735), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone());
    }
    pub fn limit(& self, n__737: i32) -> temper_core::Result<Query> {
        if Some(n__737) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__737), self.0.offset_val, self.0.join_clauses.clone()));
    }
    pub fn offset(& self, n__740: i32) -> temper_core::Result<Query> {
        if Some(n__740) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__740), self.0.join_clauses.clone()));
    }
    pub fn join(& self, joinType__743: JoinType, table__744: SafeIdentifier, onCondition__745: SqlFragment) -> Query {
        let nb__747: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__747, JoinClause::new(joinType__743.clone(), table__744.clone(), onCondition__745.clone()), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__747));
    }
    pub fn inner_join(& self, table__749: SafeIdentifier, onCondition__750: SqlFragment) -> Query {
        let mut t___6518: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___6518.clone()), table__749.clone(), onCondition__750.clone());
    }
    pub fn left_join(& self, table__753: SafeIdentifier, onCondition__754: SqlFragment) -> Query {
        let mut t___6516: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___6516.clone()), table__753.clone(), onCondition__754.clone());
    }
    pub fn right_join(& self, table__757: SafeIdentifier, onCondition__758: SqlFragment) -> Query {
        let mut t___6514: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___6514.clone()), table__757.clone(), onCondition__758.clone());
    }
    pub fn full_join(& self, table__761: SafeIdentifier, onCondition__762: SqlFragment) -> Query {
        let mut t___6512: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___6512.clone()), table__761.clone(), onCondition__762.clone());
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___6494: i32;
        let b__766: SqlBuilder = SqlBuilder::new();
        b__766.append_safe("SELECT ");
        if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
            b__766.append_safe("*");
        } else {
            #[derive(Clone)]
            struct ClosureGroup___4 {}
            impl ClosureGroup___4 {
                fn fn__6476(& self, f__767: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__767.sql_value();
                }
            }
            let closure_group = ClosureGroup___4 {};
            let fn__6476 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__767: SafeIdentifier | closure_group.fn__6476(f__767))
            };
            b__766.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__6476.clone())));
        }
        b__766.append_safe(" FROM ");
        b__766.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___5 {
            b__766: SqlBuilder
        }
        impl ClosureGroup___5 {
            fn fn__6475(& self, jc__768: JoinClause) {
                self.b__766.append_safe(" ");
                let mut t___6464: std::sync::Arc<String> = jc__768.join_type().keyword();
                self.b__766.append_safe(t___6464.clone());
                self.b__766.append_safe(" ");
                let mut t___6468: std::sync::Arc<String> = jc__768.table().sql_value();
                self.b__766.append_safe(t___6468.clone());
                self.b__766.append_safe(" ON ");
                let mut t___6471: SqlFragment = jc__768.on_condition();
                self.b__766.append_fragment(t___6471.clone());
            }
        }
        let closure_group = ClosureGroup___5 {
            b__766: b__766.clone()
        };
        let fn__6475 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__768: JoinClause | closure_group.fn__6475(jc__768))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__6475.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__766.append_safe(" WHERE ");
            b__766.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__769: i32 = 1;
            'loop___7319: loop {
                t___6494 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__769) < Some(t___6494)) {
                    break;
                }
                b__766.append_safe(" ");
                b__766.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__769).keyword());
                b__766.append_safe(" ");
                b__766.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__769).condition());
                i__769 = i__769.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__766.append_safe(" ORDER BY ");
            let mut first__770: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___6 {
                first__770: std::sync::Arc<std::sync::RwLock<bool>>, b__766: SqlBuilder
            }
            impl ClosureGroup___6 {
                fn fn__6474(& self, oc__771: OrderClause) {
                    let mut t___3520: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__770) {
                        self.b__766.append_safe(", ");
                    }
                    {
                        * self.first__770.write().unwrap() = false;
                    }
                    let mut t___6458: std::sync::Arc<String> = oc__771.field().sql_value();
                    self.b__766.append_safe(t___6458.clone());
                    if oc__771.ascending() {
                        t___3520 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___3520 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__766.append_safe(t___3520.clone());
                }
            }
            let closure_group = ClosureGroup___6 {
                first__770: first__770.clone(), b__766: b__766.clone()
            };
            let fn__6474 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | oc__771: OrderClause | closure_group.fn__6474(oc__771))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__6474.clone()));
        }
        let lv__772: Option<i32> = self.0.limit_val;
        if ! lv__772.is_none() {
            let lv___1452: i32 = lv__772.unwrap();
            b__766.append_safe(" LIMIT ");
            b__766.append_int32(lv___1452);
        }
        let ov__773: Option<i32> = self.0.offset_val;
        if ! ov__773.is_none() {
            let ov___1453: i32 = ov__773.unwrap();
            b__766.append_safe(" OFFSET ");
            b__766.append_int32(ov___1453);
        }
        return b__766.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__775: i32) -> temper_core::Result<SqlFragment> {
        let return__316: SqlFragment;
        let mut t___3513: Query;
        if Some(defaultLimit__775) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__316 = self.to_sql();
        } else {
            t___3513 = self.limit(defaultLimit__775) ? ;
            return__316 = t___3513.to_sql();
        }
        return Ok(return__316.clone());
    }
    pub fn new(tableName__778: SafeIdentifier, conditions__779: impl temper_core::ToList<WhereClause>, selectedFields__780: impl temper_core::ToList<SafeIdentifier>, orderClauses__781: impl temper_core::ToList<OrderClause>, limitVal__782: Option<i32>, offsetVal__783: Option<i32>, joinClauses__784: impl temper_core::ToList<JoinClause>) -> Query {
        let conditions__779 = conditions__779.to_list();
        let selectedFields__780 = selectedFields__780.to_list();
        let orderClauses__781 = orderClauses__781.to_list();
        let joinClauses__784 = joinClauses__784.to_list();
        let table_name;
        let conditions;
        let selected_fields;
        let order_clauses;
        let limit_val;
        let offset_val;
        let join_clauses;
        table_name = tableName__778.clone();
        conditions = conditions__779.clone();
        selected_fields = selectedFields__780.clone();
        order_clauses = orderClauses__781.clone();
        limit_val = limitVal__782;
        offset_val = offsetVal__783;
        join_clauses = joinClauses__784.clone();
        let selfish = Query(std::sync::Arc::new(QueryStruct {
                    table_name, conditions, selected_fields, order_clauses, limit_val, offset_val, join_clauses
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.conditions.clone();
    }
    pub fn selected_fields(& self) -> temper_core::List<SafeIdentifier> {
        return self.0.selected_fields.clone();
    }
    pub fn order_clauses(& self) -> temper_core::List<OrderClause> {
        return self.0.order_clauses.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
    pub fn offset_val(& self) -> Option<i32> {
        return self.0.offset_val;
    }
    pub fn join_clauses(& self) -> temper_core::List<JoinClause> {
        return self.0.join_clauses.clone();
    }
}
temper_core::impl_any_value_trait!(Query, []);
pub enum SafeIdentifierEnum {
    ValidatedIdentifier(ValidatedIdentifier)
}
pub trait SafeIdentifierTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SafeIdentifierEnum;
    fn clone_boxed(& self) -> SafeIdentifier;
    fn sql_value(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct SafeIdentifier(std::sync::Arc<dyn SafeIdentifierTrait>);
impl SafeIdentifier {
    pub fn new(selfish: impl SafeIdentifierTrait + 'static) -> SafeIdentifier {
        SafeIdentifier(std::sync::Arc::new(selfish))
    }
}
impl SafeIdentifierTrait for SafeIdentifier {
    fn as_enum(& self) -> SafeIdentifierEnum {
        SafeIdentifierTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> SafeIdentifier {
        SafeIdentifierTrait::clone_boxed( & ( * self.0))
    }
    fn sql_value(& self) -> std::sync::Arc<String> {
        SafeIdentifierTrait::sql_value( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(SafeIdentifier);
impl std::ops::Deref for SafeIdentifier {
    type Target = dyn SafeIdentifierTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ValidatedIdentifierStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub (crate) struct ValidatedIdentifier(std::sync::Arc<ValidatedIdentifierStruct>);
impl ValidatedIdentifier {
    pub fn sql_value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
    pub fn new(_value__898: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__898 = _value__898.to_arc_string();
        let value;
        value = _value__898.clone();
        let selfish = ValidatedIdentifier(std::sync::Arc::new(ValidatedIdentifierStruct {
                    value
        }));
        return selfish;
    }
}
impl SafeIdentifierTrait for ValidatedIdentifier {
    fn as_enum(& self) -> SafeIdentifierEnum {
        SafeIdentifierEnum::ValidatedIdentifier(self.clone())
    }
    fn clone_boxed(& self) -> SafeIdentifier {
        SafeIdentifier::new(self.clone())
    }
    fn sql_value(& self) -> std::sync::Arc<String> {
        self.sql_value()
    }
}
temper_core::impl_any_value_trait!(ValidatedIdentifier, [SafeIdentifier]);
pub enum FieldTypeEnum {
    StringField(StringField), IntField(IntField), Int64Field(Int64Field), FloatField(FloatField), BoolField(BoolField), DateField(DateField)
}
pub trait FieldTypeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> FieldTypeEnum;
    fn clone_boxed(& self) -> FieldType;
}
#[derive(Clone)]
pub struct FieldType(std::sync::Arc<dyn FieldTypeTrait>);
impl FieldType {
    pub fn new(selfish: impl FieldTypeTrait + 'static) -> FieldType {
        FieldType(std::sync::Arc::new(selfish))
    }
}
impl FieldTypeTrait for FieldType {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> FieldType {
        FieldTypeTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(FieldType);
impl std::ops::Deref for FieldType {
    type Target = dyn FieldTypeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct StringFieldStruct {}
#[derive(Clone)]
pub struct StringField(std::sync::Arc<StringFieldStruct>);
impl StringField {
    pub fn new() -> StringField {
        let selfish = StringField(std::sync::Arc::new(StringFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for StringField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::StringField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(StringField, [FieldType]);
struct IntFieldStruct {}
#[derive(Clone)]
pub struct IntField(std::sync::Arc<IntFieldStruct>);
impl IntField {
    pub fn new() -> IntField {
        let selfish = IntField(std::sync::Arc::new(IntFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for IntField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::IntField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(IntField, [FieldType]);
struct Int64FieldStruct {}
#[derive(Clone)]
pub struct Int64Field(std::sync::Arc<Int64FieldStruct>);
impl Int64Field {
    pub fn new() -> Int64Field {
        let selfish = Int64Field(std::sync::Arc::new(Int64FieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for Int64Field {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::Int64Field(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Int64Field, [FieldType]);
struct FloatFieldStruct {}
#[derive(Clone)]
pub struct FloatField(std::sync::Arc<FloatFieldStruct>);
impl FloatField {
    pub fn new() -> FloatField {
        let selfish = FloatField(std::sync::Arc::new(FloatFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for FloatField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::FloatField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(FloatField, [FieldType]);
struct BoolFieldStruct {}
#[derive(Clone)]
pub struct BoolField(std::sync::Arc<BoolFieldStruct>);
impl BoolField {
    pub fn new() -> BoolField {
        let selfish = BoolField(std::sync::Arc::new(BoolFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for BoolField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::BoolField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(BoolField, [FieldType]);
struct DateFieldStruct {}
#[derive(Clone)]
pub struct DateField(std::sync::Arc<DateFieldStruct>);
impl DateField {
    pub fn new() -> DateField {
        let selfish = DateField(std::sync::Arc::new(DateFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for DateField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::DateField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(DateField, [FieldType]);
struct FieldDefStruct {
    name: SafeIdentifier, field_type: FieldType, nullable: bool
}
#[derive(Clone)]
pub struct FieldDef(std::sync::Arc<FieldDefStruct>);
#[derive(Clone)]
pub struct FieldDefBuilder {
    pub name: SafeIdentifier, pub field_type: FieldType, pub nullable: bool
}
impl FieldDefBuilder {
    pub fn build(self) -> FieldDef {
        FieldDef::new(self.name, self.field_type, self.nullable)
    }
}
impl FieldDef {
    pub fn new(name__916: SafeIdentifier, fieldType__917: FieldType, nullable__918: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__916.clone();
        field_type = fieldType__917.clone();
        nullable = nullable__918;
        let selfish = FieldDef(std::sync::Arc::new(FieldDefStruct {
                    name, field_type, nullable
        }));
        return selfish;
    }
    pub fn name(& self) -> SafeIdentifier {
        return self.0.name.clone();
    }
    pub fn field_type(& self) -> FieldType {
        return self.0.field_type.clone();
    }
    pub fn nullable(& self) -> bool {
        return self.0.nullable;
    }
}
temper_core::impl_any_value_trait!(FieldDef, []);
struct TableDefStruct {
    table_name: SafeIdentifier, fields: temper_core::List<FieldDef>
}
#[derive(Clone)]
pub struct TableDef(std::sync::Arc<TableDefStruct>);
#[derive(Clone)]
pub struct TableDefBuilder {
    pub table_name: SafeIdentifier, pub fields: temper_core::List<FieldDef>
}
impl TableDefBuilder {
    pub fn build(self) -> TableDef {
        TableDef::new(self.table_name, self.fields)
    }
}
impl TableDef {
    pub fn field(& self, name__922: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__922 = name__922.to_arc_string();
        let return__346: FieldDef;
        'fn__923: {
            let this__4373: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__4374: i32 = temper_core::ListedTrait::len( & this__4373);
            let mut i__4375: i32 = 0;
            'loop___7326: while Some(i__4375) < Some(n__4374) {
                let el__4376: FieldDef = temper_core::ListedTrait::get( & this__4373, i__4375);
                i__4375 = i__4375.wrapping_add(1);
                let f__924: FieldDef = el__4376.clone();
                if Some(f__924.name().sql_value().as_str()) == Some(name__922.as_str()) {
                    return__346 = f__924.clone();
                    break 'fn__923;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__346.clone());
    }
    pub fn new(tableName__926: SafeIdentifier, fields__927: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__927 = fields__927.to_list();
        let table_name;
        let fields;
        table_name = tableName__926.clone();
        fields = fields__927.clone();
        let selfish = TableDef(std::sync::Arc::new(TableDefStruct {
                    table_name, fields
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn fields(& self) -> temper_core::List<FieldDef> {
        return self.0.fields.clone();
    }
}
temper_core::impl_any_value_trait!(TableDef, []);
struct SqlBuilderStruct {
    buffer: temper_core::ListBuilder<SqlPart>
}
#[derive(Clone)]
pub struct SqlBuilder(std::sync::Arc<SqlBuilderStruct>);
impl SqlBuilder {
    pub fn append_safe(& self, sqlSource__949: impl temper_core::ToArcString) {
        let sqlSource__949 = sqlSource__949.to_arc_string();
        let mut t___7190: SqlSource = SqlSource::new(sqlSource__949.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___7190.clone()), None);
    }
    pub fn append_fragment(& self, fragment__952: SqlFragment) {
        let mut t___7188: temper_core::List<SqlPart> = fragment__952.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___7188.clone()), None);
    }
    pub fn append_part(& self, part__955: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__955.clone(), None);
    }
    pub fn append_part_list(& self, values__958: impl temper_core::ToList<SqlPart>) {
        let values__958 = values__958.to_list();
        #[derive(Clone)]
        struct ClosureGroup___7 {
            this__183: SqlBuilder
        }
        impl ClosureGroup___7 {
            fn fn__7184(& self, x__960: SqlPart) {
                self.this__183.append_part(x__960.clone());
            }
        }
        let closure_group = ClosureGroup___7 {
            this__183: self.clone()
        };
        let fn__7184 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__960: SqlPart | closure_group.fn__7184(x__960))
        };
        self.append_list(temper_core::ToListed::to_listed(values__958.clone()), fn__7184.clone());
    }
    pub fn append_boolean(& self, value__962: bool) {
        let mut t___7181: SqlBoolean = SqlBoolean::new(value__962);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___7181.clone()), None);
    }
    pub fn append_boolean_list(& self, values__965: impl temper_core::ToListed<bool>) {
        let values__965 = values__965.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___8 {
            this__185: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__7178(& self, x__967: bool) {
                self.this__185.append_boolean(x__967);
            }
        }
        let closure_group = ClosureGroup___8 {
            this__185: self.clone()
        };
        let fn__7178 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__967: bool | closure_group.fn__7178(x__967))
        };
        self.append_list(values__965.clone(), fn__7178.clone());
    }
    pub fn append_date(& self, value__969: temper_std::temporal::Date) {
        let mut t___7175: SqlDate = SqlDate::new(value__969.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___7175.clone()), None);
    }
    pub fn append_date_list(& self, values__972: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__972 = values__972.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___9 {
            this__187: SqlBuilder
        }
        impl ClosureGroup___9 {
            fn fn__7172(& self, x__974: temper_std::temporal::Date) {
                self.this__187.append_date(x__974.clone());
            }
        }
        let closure_group = ClosureGroup___9 {
            this__187: self.clone()
        };
        let fn__7172 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__974: temper_std::temporal::Date | closure_group.fn__7172(x__974))
        };
        self.append_list(values__972.clone(), fn__7172.clone());
    }
    pub fn append_float64(& self, value__976: f64) {
        let mut t___7169: SqlFloat64 = SqlFloat64::new(value__976);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___7169.clone()), None);
    }
    pub fn append_float64_list(& self, values__979: impl temper_core::ToListed<f64>) {
        let values__979 = values__979.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__189: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__7166(& self, x__981: f64) {
                self.this__189.append_float64(x__981);
            }
        }
        let closure_group = ClosureGroup___10 {
            this__189: self.clone()
        };
        let fn__7166 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__981: f64 | closure_group.fn__7166(x__981))
        };
        self.append_list(values__979.clone(), fn__7166.clone());
    }
    pub fn append_int32(& self, value__983: i32) {
        let mut t___7163: SqlInt32 = SqlInt32::new(value__983);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___7163.clone()), None);
    }
    pub fn append_int32_list(& self, values__986: impl temper_core::ToListed<i32>) {
        let values__986 = values__986.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__191: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__7160(& self, x__988: i32) {
                self.this__191.append_int32(x__988);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__191: self.clone()
        };
        let fn__7160 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__988: i32 | closure_group.fn__7160(x__988))
        };
        self.append_list(values__986.clone(), fn__7160.clone());
    }
    pub fn append_int64(& self, value__990: i64) {
        let mut t___7157: SqlInt64 = SqlInt64::new(value__990);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___7157.clone()), None);
    }
    pub fn append_int64_list(& self, values__993: impl temper_core::ToListed<i64>) {
        let values__993 = values__993.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__193: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__7154(& self, x__995: i64) {
                self.this__193.append_int64(x__995);
            }
        }
        let closure_group = ClosureGroup___12 {
            this__193: self.clone()
        };
        let fn__7154 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__995: i64 | closure_group.fn__7154(x__995))
        };
        self.append_list(values__993.clone(), fn__7154.clone());
    }
    pub fn append_string(& self, value__997: impl temper_core::ToArcString) {
        let value__997 = value__997.to_arc_string();
        let mut t___7151: SqlString = SqlString::new(value__997.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___7151.clone()), None);
    }
    pub fn append_string_list(& self, values__1000: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__1000 = values__1000.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__195: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__7148(& self, x__1002: impl temper_core::ToArcString) {
                let x__1002 = x__1002.to_arc_string();
                self.this__195.append_string(x__1002.clone());
            }
        }
        let closure_group = ClosureGroup___13 {
            this__195: self.clone()
        };
        let fn__7148 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1002: std::sync::Arc<String> | closure_group.fn__7148(x__1002))
        };
        self.append_list(values__1000.clone(), fn__7148.clone());
    }
    fn append_list<T>(& self, values__1004: impl temper_core::ToListed<T>, appendValue__1005: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__1004 = values__1004.to_listed();
        let mut t___7143: i32;
        let mut t___7145: T;
        let mut i__1007: i32 = 0;
        'loop___7327: loop {
            t___7143 = temper_core::ListedTrait::len( & ( * values__1004));
            if ! (Some(i__1007) < Some(t___7143)) {
                break;
            }
            if Some(i__1007) > Some(0) {
                self.append_safe(", ");
            }
            t___7145 = temper_core::ListedTrait::get( & ( * values__1004), i__1007);
            appendValue__1005(t___7145.clone());
            i__1007 = i__1007.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___7140: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___7140.clone();
        let selfish = SqlBuilder(std::sync::Arc::new(SqlBuilderStruct {
                    buffer
        }));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(SqlBuilder, []);
struct SqlFragmentStruct {
    parts: temper_core::List<SqlPart>
}
#[derive(Clone)]
pub struct SqlFragment(std::sync::Arc<SqlFragmentStruct>);
impl SqlFragment {
    pub fn to_source(& self) -> SqlSource {
        return SqlSource::new(self.to_string());
    }
    pub fn to_string(& self) -> std::sync::Arc<String> {
        let mut t___7214: i32;
        let builder__1019: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__1020: i32 = 0;
        'loop___7328: loop {
            t___7214 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__1020) < Some(t___7214)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__1020).format_to(builder__1019.clone());
            i__1020 = i__1020.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__1019);
    }
    pub fn new(parts__1022: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__1022 = parts__1022.to_list();
        let parts;
        parts = parts__1022.clone();
        let selfish = SqlFragment(std::sync::Arc::new(SqlFragmentStruct {
                    parts
        }));
        return selfish;
    }
    pub fn parts(& self) -> temper_core::List<SqlPart> {
        return self.0.parts.clone();
    }
}
temper_core::impl_any_value_trait!(SqlFragment, []);
pub enum SqlPartEnum {
    SqlSource(SqlSource), SqlBoolean(SqlBoolean), SqlString(SqlString), SqlInt32(SqlInt32), SqlInt64(SqlInt64), SqlFloat64(SqlFloat64), SqlDate(SqlDate)
}
pub trait SqlPartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SqlPartEnum;
    fn clone_boxed(& self) -> SqlPart;
    fn format_to(& self, builder__1024: std::sync::Arc<std::sync::RwLock<String>>);
}
#[derive(Clone)]
pub struct SqlPart(std::sync::Arc<dyn SqlPartTrait>);
impl SqlPart {
    pub fn new(selfish: impl SqlPartTrait + 'static) -> SqlPart {
        SqlPart(std::sync::Arc::new(selfish))
    }
}
impl SqlPartTrait for SqlPart {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPartTrait::clone_boxed( & ( * self.0))
    }
    fn format_to(& self, arg1: std::sync::Arc<std::sync::RwLock<String>>) -> () {
        SqlPartTrait::format_to( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(SqlPart);
impl std::ops::Deref for SqlPart {
    type Target = dyn SqlPartTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct SqlSourceStruct {
    source: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlSource(std::sync::Arc<SqlSourceStruct>);
impl SqlSource {
    pub fn format_to(& self, builder__1028: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1028, self.0.source.clone());
    }
    pub fn new(source__1031: impl temper_core::ToArcString) -> SqlSource {
        let source__1031 = source__1031.to_arc_string();
        let source;
        source = source__1031.clone();
        let selfish = SqlSource(std::sync::Arc::new(SqlSourceStruct {
                    source
        }));
        return selfish;
    }
    pub fn source(& self) -> std::sync::Arc<String> {
        return self.0.source.clone();
    }
}
impl SqlPartTrait for SqlSource {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlSource(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__1028: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1028)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__1034: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___4218: std::sync::Arc<String>;
        if self.0.value {
            t___4218 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___4218 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__1034, t___4218.clone());
    }
    pub fn new(value__1037: bool) -> SqlBoolean {
        let value;
        value = value__1037;
        let selfish = SqlBoolean(std::sync::Arc::new(SqlBooleanStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> bool {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlBoolean {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlBoolean(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__1034: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1034)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__1040: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1040, "'");
        let mut t___7195: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            builder__1040: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___14 {
            fn fn__7193(& self, c__1042: i32) {
                if Some(c__1042) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1040, "''");
                } else {
                    'ok___7253: {
                        'orelse___1406: {
                            match temper_core::string::builder::append_code_point( & self.builder__1040, c__1042) {
                                Ok(x) => x,
                                _ => break 'orelse___1406
                            };
                            break 'ok___7253;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___14 {
            builder__1040: builder__1040.clone()
        };
        let fn__7193 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1042: i32 | closure_group.fn__7193(c__1042))
        };
        temper_core::string::for_each( & t___7195, & ( * fn__7193.clone()));
        temper_core::string::builder::append( & builder__1040, "'");
    }
    pub fn new(value__1044: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__1044.clone();
        let selfish = SqlDate(std::sync::Arc::new(SqlDateStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> temper_std::temporal::Date {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlDate {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlDate(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__1040: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1040)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__1047: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___4207: bool;
        let mut t___4208: bool;
        let s__1049: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__1049.as_str()) == Some("NaN") {
            t___4208 = true;
        } else {
            if Some(s__1049.as_str()) == Some("Infinity") {
                t___4207 = true;
            } else {
                t___4207 = Some(s__1049.as_str()) == Some("-Infinity");
            }
            t___4208 = t___4207;
        }
        if t___4208 {
            temper_core::string::builder::append( & builder__1047, "NULL");
        } else {
            temper_core::string::builder::append( & builder__1047, s__1049.clone());
        }
    }
    pub fn new(value__1051: f64) -> SqlFloat64 {
        let value;
        value = value__1051;
        let selfish = SqlFloat64(std::sync::Arc::new(SqlFloat64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> f64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlFloat64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlFloat64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__1047: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1047)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__1054: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___7204: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1054, t___7204.clone());
    }
    pub fn new(value__1057: i32) -> SqlInt32 {
        let value;
        value = value__1057;
        let selfish = SqlInt32(std::sync::Arc::new(SqlInt32Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i32 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt32 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt32(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__1054: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1054)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__1060: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___7202: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1060, t___7202.clone());
    }
    pub fn new(value__1063: i64) -> SqlInt64 {
        let value;
        value = value__1063;
        let selfish = SqlInt64(std::sync::Arc::new(SqlInt64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__1060: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1060)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__1066: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1066, "'");
        #[derive(Clone)]
        struct ClosureGroup___15 {
            builder__1066: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___15 {
            fn fn__7207(& self, c__1068: i32) {
                if Some(c__1068) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1066, "''");
                } else {
                    'ok___7258: {
                        'orelse___1405: {
                            match temper_core::string::builder::append_code_point( & self.builder__1066, c__1068) {
                                Ok(x) => x,
                                _ => break 'orelse___1405
                            };
                            break 'ok___7258;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___15 {
            builder__1066: builder__1066.clone()
        };
        let fn__7207 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1068: i32 | closure_group.fn__7207(c__1068))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__7207.clone()));
        temper_core::string::builder::append( & builder__1066, "'");
    }
    pub fn new(value__1070: impl temper_core::ToArcString) -> SqlString {
        let value__1070 = value__1070.to_arc_string();
        let value;
        value = value__1070.clone();
        let selfish = SqlString(std::sync::Arc::new(SqlStringStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlString {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlString(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__1066: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1066)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__540: TableDef, params__541: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___6990: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__540.clone(), params__541.clone(), t___6990.clone(), [], true));
}
fn isIdentStart__401(c__899: i32) -> bool {
    let return__326: bool;
    let mut t___3981: bool;
    let mut t___3982: bool;
    if Some(c__899) >= Some(97) {
        t___3981 = Some(c__899) <= Some(122);
    } else {
        t___3981 = false;
    }
    if t___3981 {
        return__326 = true;
    } else {
        if Some(c__899) >= Some(65) {
            t___3982 = Some(c__899) <= Some(90);
        } else {
            t___3982 = false;
        }
        if t___3982 {
            return__326 = true;
        } else {
            return__326 = Some(c__899) == Some(95);
        }
    }
    return return__326;
}
fn isIdentPart__402(c__901: i32) -> bool {
    let return__327: bool;
    if isIdentStart__401(c__901) {
        return__327 = true;
    } else {
        if Some(c__901) >= Some(48) {
            return__327 = Some(c__901) <= Some(57);
        } else {
            return__327 = false;
        }
    }
    return return__327;
}
pub fn safe_identifier(name__903: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__903 = name__903.to_arc_string();
    let mut t___6988: usize;
    if name__903.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__905: usize = 0usize;
    if ! isIdentStart__401(temper_core::string::get( & name__903, idx__905)) {
        return Err(temper_core::Error::new());
    }
    let mut t___6985: usize = temper_core::string::next( & name__903, idx__905);
    idx__905 = t___6985;
    'loop___7329: loop {
        if ! temper_core::string::has_index( & name__903, idx__905) {
            break;
        }
        if ! isIdentPart__402(temper_core::string::get( & name__903, idx__905)) {
            return Err(temper_core::Error::new());
        }
        t___6988 = temper_core::string::next( & name__903, idx__905);
        idx__905 = t___6988;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__903.clone())));
}
fn csid__398(name__543: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__543 = name__543.to_arc_string();
    let return__256: SafeIdentifier;
    let mut t___3969: SafeIdentifier;
    'ok___7263: {
        'orelse___1410: {
            t___3969 = match safe_identifier(name__543.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1410
            };
            return__256 = t___3969.clone();
            break 'ok___7263;
        }
        return__256 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__256.clone();
}
fn userTable__399() -> TableDef {
    return TableDef::new(csid__398("users"), [FieldDef::new(csid__398("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__398("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__398("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__398("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__398("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__630: TableDef, id__631: i32) -> SqlFragment {
    let b__633: SqlBuilder = SqlBuilder::new();
    b__633.append_safe("DELETE FROM ");
    b__633.append_safe(tableDef__630.table_name().sql_value());
    b__633.append_safe(" WHERE id = ");
    b__633.append_int32(id__631);
    return b__633.accumulated();
}
pub fn from(tableName__785: SafeIdentifier) -> Query {
    return Query::new(tableName__785.clone(), [], [], [], None, None, []);
}
pub fn col(table__787: SafeIdentifier, column__788: SafeIdentifier) -> SqlFragment {
    let b__790: SqlBuilder = SqlBuilder::new();
    b__790.append_safe(table__787.sql_value());
    b__790.append_safe(".");
    b__790.append_safe(column__788.sql_value());
    return b__790.accumulated();
}
fn sid__400(name__791: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__791 = name__791.to_arc_string();
    let return__319: SafeIdentifier;
    let mut t___3489: SafeIdentifier;
    'ok___7274: {
        'orelse___1418: {
            t___3489 = match safe_identifier(name__791.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1418
            };
            return__319 = t___3489.clone();
            break 'ok___7274;
        }
        return__319 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__319.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__1178() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___22 = temper_std::testing::Test::new();
        let params__547: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___6946: TableDef = userTable__399();
        let mut t___6947: SafeIdentifier = csid__398("name");
        let mut t___6948: SafeIdentifier = csid__398("email");
        let cs__548: Changeset = changeset(t___6946.clone(), params__547.clone()).cast(std::sync::Arc::new(vec![t___6947.clone(), t___6948.clone()]));
        let mut t___6951: bool = temper_core::MappedTrait::has( & cs__548.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___16 {}
        impl ClosureGroup___16 {
            fn fn__6941(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___16 {};
        let fn__6941 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6941())
        };
        test___22.assert(t___6951, fn__6941.clone());
        let mut t___6955: bool = temper_core::MappedTrait::has( & cs__548.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___17 {}
        impl ClosureGroup___17 {
            fn fn__6940(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___17 {};
        let fn__6940 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6940())
        };
        test___22.assert(t___6955, fn__6940.clone());
        let mut t___6961: bool = ! temper_core::MappedTrait::has( & cs__548.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___18 {}
        impl ClosureGroup___18 {
            fn fn__6939(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___18 {};
        let fn__6939 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6939())
        };
        test___22.assert(t___6961, fn__6939.clone());
        let mut t___6963: bool = cs__548.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__6938(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__6938 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6938())
        };
        test___22.assert(t___6963, fn__6938.clone());
        test___22.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__1179() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___23 = temper_std::testing::Test::new();
        let params__550: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___6924: TableDef = userTable__399();
        let mut t___6925: SafeIdentifier = csid__398("name");
        let cs__551: Changeset = changeset(t___6924.clone(), params__550.clone()).cast(std::sync::Arc::new(vec![t___6925.clone()])).cast(std::sync::Arc::new(vec![csid__398("email")]));
        let mut t___6932: bool = ! temper_core::MappedTrait::has( & cs__551.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__6920(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__6920 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6920())
        };
        test___23.assert(t___6932, fn__6920.clone());
        let mut t___6935: bool = temper_core::MappedTrait::has( & cs__551.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__6919(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__6919 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6919())
        };
        test___23.assert(t___6935, fn__6919.clone());
        test___23.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__1180() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let params__553: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___6906: TableDef = userTable__399();
        let mut t___6907: SafeIdentifier = csid__398("name");
        let mut t___6908: SafeIdentifier = csid__398("email");
        let cs__554: Changeset = changeset(t___6906.clone(), params__553.clone()).cast(std::sync::Arc::new(vec![t___6907.clone(), t___6908.clone()]));
        let mut t___6913: bool = ! temper_core::MappedTrait::has( & cs__554.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__6902(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__6902 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6902())
        };
        test___24.assert(t___6913, fn__6902.clone());
        let mut t___6916: bool = temper_core::MappedTrait::has( & cs__554.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__6901(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__6901 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6901())
        };
        test___24.assert(t___6916, fn__6901.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__1181() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let params__556: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___6888: TableDef = userTable__399();
        let mut t___6889: SafeIdentifier = csid__398("name");
        let cs__557: Changeset = changeset(t___6888.clone(), params__556.clone()).cast(std::sync::Arc::new(vec![t___6889.clone()])).validate_required(std::sync::Arc::new(vec![csid__398("name")]));
        let mut t___6893: bool = cs__557.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__6885(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__6885 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6885())
        };
        test___25.assert(t___6893, fn__6885.clone());
        let mut t___6899: bool = Some(temper_core::ListedTrait::len( & cs__557.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__6884(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__6884 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6884())
        };
        test___25.assert(t___6899, fn__6884.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__1182() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let params__559: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___6864: TableDef = userTable__399();
        let mut t___6865: SafeIdentifier = csid__398("name");
        let cs__560: Changeset = changeset(t___6864.clone(), params__559.clone()).cast(std::sync::Arc::new(vec![t___6865.clone()])).validate_required(std::sync::Arc::new(vec![csid__398("name")]));
        let mut t___6871: bool = ! cs__560.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__6862(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__6862 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6862())
        };
        test___26.assert(t___6871, fn__6862.clone());
        let mut t___6876: bool = Some(temper_core::ListedTrait::len( & cs__560.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__6861(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__6861 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6861())
        };
        test___26.assert(t___6876, fn__6861.clone());
        let mut t___6882: bool = Some(temper_core::ListedTrait::get( & cs__560.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__6860(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__6860 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6860())
        };
        test___26.assert(t___6882, fn__6860.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__1183() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let params__562: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___6852: TableDef = userTable__399();
        let mut t___6853: SafeIdentifier = csid__398("name");
        let cs__563: Changeset = changeset(t___6852.clone(), params__562.clone()).cast(std::sync::Arc::new(vec![t___6853.clone()])).validate_length(csid__398("name"), 2, 50);
        let mut t___6857: bool = cs__563.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__6849(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__6849 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6849())
        };
        test___27.assert(t___6857, fn__6849.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__1184() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let params__565: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___6840: TableDef = userTable__399();
        let mut t___6841: SafeIdentifier = csid__398("name");
        let cs__566: Changeset = changeset(t___6840.clone(), params__565.clone()).cast(std::sync::Arc::new(vec![t___6841.clone()])).validate_length(csid__398("name"), 2, 50);
        let mut t___6847: bool = ! cs__566.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__6837(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__6837 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6837())
        };
        test___28.assert(t___6847, fn__6837.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__1185() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let params__568: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___6828: TableDef = userTable__399();
        let mut t___6829: SafeIdentifier = csid__398("name");
        let cs__569: Changeset = changeset(t___6828.clone(), params__568.clone()).cast(std::sync::Arc::new(vec![t___6829.clone()])).validate_length(csid__398("name"), 2, 10);
        let mut t___6835: bool = ! cs__569.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__6825(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__6825 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6825())
        };
        test___29.assert(t___6835, fn__6825.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__1186() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let params__571: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___6817: TableDef = userTable__399();
        let mut t___6818: SafeIdentifier = csid__398("age");
        let cs__572: Changeset = changeset(t___6817.clone(), params__571.clone()).cast(std::sync::Arc::new(vec![t___6818.clone()])).validate_int(csid__398("age"));
        let mut t___6822: bool = cs__572.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__6814(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__6814 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6814())
        };
        test___30.assert(t___6822, fn__6814.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__1187() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__574: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___6805: TableDef = userTable__399();
        let mut t___6806: SafeIdentifier = csid__398("age");
        let cs__575: Changeset = changeset(t___6805.clone(), params__574.clone()).cast(std::sync::Arc::new(vec![t___6806.clone()])).validate_int(csid__398("age"));
        let mut t___6812: bool = ! cs__575.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__6802(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__6802 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6802())
        };
        test___31.assert(t___6812, fn__6802.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__1188() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__577: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___6794: TableDef = userTable__399();
        let mut t___6795: SafeIdentifier = csid__398("score");
        let cs__578: Changeset = changeset(t___6794.clone(), params__577.clone()).cast(std::sync::Arc::new(vec![t___6795.clone()])).validate_float(csid__398("score"));
        let mut t___6799: bool = cs__578.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__6791(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__6791 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6791())
        };
        test___32.assert(t___6799, fn__6791.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__1189() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__580: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___6783: TableDef = userTable__399();
        let mut t___6784: SafeIdentifier = csid__398("age");
        let cs__581: Changeset = changeset(t___6783.clone(), params__580.clone()).cast(std::sync::Arc::new(vec![t___6784.clone()])).validate_int64(csid__398("age"));
        let mut t___6788: bool = cs__581.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__6780(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__6780 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6780())
        };
        test___33.assert(t___6788, fn__6780.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__1190() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__583: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___6771: TableDef = userTable__399();
        let mut t___6772: SafeIdentifier = csid__398("age");
        let cs__584: Changeset = changeset(t___6771.clone(), params__583.clone()).cast(std::sync::Arc::new(vec![t___6772.clone()])).validate_int64(csid__398("age"));
        let mut t___6778: bool = ! cs__584.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__6768(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__6768 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6768())
        };
        test___34.assert(t___6778, fn__6768.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__1191() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___37 {
            test___35: temper_std::testing::Test
        }
        impl ClosureGroup___37 {
            fn fn__6765(& self, v__586: impl temper_core::ToArcString) {
                let v__586 = v__586.to_arc_string();
                let params__587: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__586.clone())]);
                let mut t___6757: TableDef = userTable__399();
                let mut t___6758: SafeIdentifier = csid__398("active");
                let cs__588: Changeset = changeset(t___6757.clone(), params__587.clone()).cast(std::sync::Arc::new(vec![t___6758.clone()])).validate_bool(csid__398("active"));
                let mut t___6762: bool = cs__588.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___38 {
                    v__586: std::sync::Arc<String>
                }
                impl ClosureGroup___38 {
                    fn fn__6754(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__586.clone()));
                    }
                }
                let closure_group = ClosureGroup___38 {
                    v__586: v__586.clone()
                };
                let fn__6754 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__6754())
                };
                self.test___35.assert(t___6762, fn__6754.clone());
            }
        }
        let closure_group = ClosureGroup___37 {
            test___35: test___35.clone()
        };
        let fn__6765 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__586: std::sync::Arc<String> | closure_group.fn__6765(v__586))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__6765.clone()));
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__1192() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___39 {
            test___36: temper_std::testing::Test
        }
        impl ClosureGroup___39 {
            fn fn__6751(& self, v__590: impl temper_core::ToArcString) {
                let v__590 = v__590.to_arc_string();
                let params__591: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__590.clone())]);
                let mut t___6743: TableDef = userTable__399();
                let mut t___6744: SafeIdentifier = csid__398("active");
                let cs__592: Changeset = changeset(t___6743.clone(), params__591.clone()).cast(std::sync::Arc::new(vec![t___6744.clone()])).validate_bool(csid__398("active"));
                let mut t___6748: bool = cs__592.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___40 {
                    v__590: std::sync::Arc<String>
                }
                impl ClosureGroup___40 {
                    fn fn__6740(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__590.clone()));
                    }
                }
                let closure_group = ClosureGroup___40 {
                    v__590: v__590.clone()
                };
                let fn__6740 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__6740())
                };
                self.test___36.assert(t___6748, fn__6740.clone());
            }
        }
        let closure_group = ClosureGroup___39 {
            test___36: test___36.clone()
        };
        let fn__6751 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__590: std::sync::Arc<String> | closure_group.fn__6751(v__590))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__6751.clone()));
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__1193() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___41 {
            test___37: temper_std::testing::Test
        }
        impl ClosureGroup___41 {
            fn fn__6737(& self, v__594: impl temper_core::ToArcString) {
                let v__594 = v__594.to_arc_string();
                let params__595: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__594.clone())]);
                let mut t___6728: TableDef = userTable__399();
                let mut t___6729: SafeIdentifier = csid__398("active");
                let cs__596: Changeset = changeset(t___6728.clone(), params__595.clone()).cast(std::sync::Arc::new(vec![t___6729.clone()])).validate_bool(csid__398("active"));
                let mut t___6735: bool = ! cs__596.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___42 {
                    v__594: std::sync::Arc<String>
                }
                impl ClosureGroup___42 {
                    fn fn__6725(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__594.clone()));
                    }
                }
                let closure_group = ClosureGroup___42 {
                    v__594: v__594.clone()
                };
                let fn__6725 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__6725())
                };
                self.test___37.assert(t___6735, fn__6725.clone());
            }
        }
        let closure_group = ClosureGroup___41 {
            test___37: test___37.clone()
        };
        let fn__6737 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__594: std::sync::Arc<String> | closure_group.fn__6737(v__594))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__6737.clone()));
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__1194() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        let mut t___3770: SqlFragment;
        let params__598: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___6713: TableDef = userTable__399();
        let mut t___6714: SafeIdentifier = csid__398("name");
        let mut t___6715: SafeIdentifier = csid__398("email");
        let cs__599: Changeset = changeset(t___6713.clone(), params__598.clone()).cast(std::sync::Arc::new(vec![t___6714.clone(), t___6715.clone()])).validate_required(std::sync::Arc::new(vec![csid__398("name"), csid__398("email")]));
        let sqlFrag__600: SqlFragment;
        'ok___7265: {
            'orelse___1411: {
                t___3770 = match cs__599.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1411
                };
                sqlFrag__600 = t___3770.clone();
                break 'ok___7265;
            }
            sqlFrag__600 = panic!();
        }
        let s__601: std::sync::Arc<String> = sqlFrag__600.to_string();
        let mut t___6722: bool = temper_core::string::index_of( & s__601, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___43 {
            s__601: std::sync::Arc<String>
        }
        impl ClosureGroup___43 {
            fn fn__6709(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__601.clone()));
            }
        }
        let closure_group = ClosureGroup___43 {
            s__601: s__601.clone()
        };
        let fn__6709 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6709())
        };
        test___38.assert(t___6722, fn__6709.clone());
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__1195() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        let mut t___3749: SqlFragment;
        let params__603: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___6693: TableDef = userTable__399();
        let mut t___6694: SafeIdentifier = csid__398("name");
        let mut t___6695: SafeIdentifier = csid__398("email");
        let cs__604: Changeset = changeset(t___6693.clone(), params__603.clone()).cast(std::sync::Arc::new(vec![t___6694.clone(), t___6695.clone()])).validate_required(std::sync::Arc::new(vec![csid__398("name"), csid__398("email")]));
        let sqlFrag__605: SqlFragment;
        'ok___7268: {
            'orelse___1412: {
                t___3749 = match cs__604.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1412
                };
                sqlFrag__605 = t___3749.clone();
                break 'ok___7268;
            }
            sqlFrag__605 = panic!();
        }
        let s__606: std::sync::Arc<String> = sqlFrag__605.to_string();
        let mut t___6702: bool = temper_core::string::index_of( & s__606, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            s__606: std::sync::Arc<String>
        }
        impl ClosureGroup___44 {
            fn fn__6689(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__606.clone()));
            }
        }
        let closure_group = ClosureGroup___44 {
            s__606: s__606.clone()
        };
        let fn__6689 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6689())
        };
        test___39.assert(t___6702, fn__6689.clone());
        let mut t___6706: bool = temper_core::string::index_of( & s__606, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___45 {
            s__606: std::sync::Arc<String>
        }
        impl ClosureGroup___45 {
            fn fn__6688(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__606.clone()));
            }
        }
        let closure_group = ClosureGroup___45 {
            s__606: s__606.clone()
        };
        let fn__6688 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6688())
        };
        test___39.assert(t___6706, fn__6688.clone());
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__1196() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let mut t___3732: SqlFragment;
        let params__608: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___6675: TableDef = userTable__399();
        let mut t___6676: SafeIdentifier = csid__398("name");
        let mut t___6677: SafeIdentifier = csid__398("email");
        let mut t___6678: SafeIdentifier = csid__398("age");
        let cs__609: Changeset = changeset(t___6675.clone(), params__608.clone()).cast(std::sync::Arc::new(vec![t___6676.clone(), t___6677.clone(), t___6678.clone()])).validate_required(std::sync::Arc::new(vec![csid__398("name"), csid__398("email")]));
        let sqlFrag__610: SqlFragment;
        'ok___7269: {
            'orelse___1413: {
                t___3732 = match cs__609.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1413
                };
                sqlFrag__610 = t___3732.clone();
                break 'ok___7269;
            }
            sqlFrag__610 = panic!();
        }
        let s__611: std::sync::Arc<String> = sqlFrag__610.to_string();
        let mut t___6685: bool = temper_core::string::index_of( & s__611, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            s__611: std::sync::Arc<String>
        }
        impl ClosureGroup___46 {
            fn fn__6670(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__611.clone()));
            }
        }
        let closure_group = ClosureGroup___46 {
            s__611: s__611.clone()
        };
        let fn__6670 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6670())
        };
        test___40.assert(t___6685, fn__6670.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__1197() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let params__613: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___6663: TableDef = userTable__399();
        let mut t___6664: SafeIdentifier = csid__398("name");
        let cs__614: Changeset = changeset(t___6663.clone(), params__613.clone()).cast(std::sync::Arc::new(vec![t___6664.clone()])).validate_required(std::sync::Arc::new(vec![csid__398("name")]));
        let didBubble__615: bool;
        'ok___7270: {
            'orelse___1414: {
                match cs__614.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1414
                };
                didBubble__615 = false;
                break 'ok___7270;
            }
            didBubble__615 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___47 {}
        impl ClosureGroup___47 {
            fn fn__6661(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___47 {};
        let fn__6661 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6661())
        };
        test___41.assert(didBubble__615, fn__6661.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__1198() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let strictTable__617: TableDef = TableDef::new(csid__398("posts"), [FieldDef::new(csid__398("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__398("body"), FieldType::new(StringField::new()), true)]);
        let params__618: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___6654: SafeIdentifier = csid__398("body");
        let cs__619: Changeset = changeset(strictTable__617.clone(), params__618.clone()).cast(std::sync::Arc::new(vec![t___6654.clone()]));
        let mut t___6656: bool = cs__619.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___48 {}
        impl ClosureGroup___48 {
            fn fn__6643(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___48 {};
        let fn__6643 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6643())
        };
        test___42.assert(t___6656, fn__6643.clone());
        let didBubble__620: bool;
        'ok___7271: {
            'orelse___1415: {
                match cs__619.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1415
                };
                didBubble__620 = false;
                break 'ok___7271;
            }
            didBubble__620 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___49 {}
        impl ClosureGroup___49 {
            fn fn__6642(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___49 {};
        let fn__6642 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6642())
        };
        test___42.assert(didBubble__620, fn__6642.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__1199() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let mut t___3692: SqlFragment;
        let params__622: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___6633: TableDef = userTable__399();
        let mut t___6634: SafeIdentifier = csid__398("name");
        let cs__623: Changeset = changeset(t___6633.clone(), params__622.clone()).cast(std::sync::Arc::new(vec![t___6634.clone()])).validate_required(std::sync::Arc::new(vec![csid__398("name")]));
        let sqlFrag__624: SqlFragment;
        'ok___7272: {
            'orelse___1416: {
                t___3692 = match cs__623.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___1416
                };
                sqlFrag__624 = t___3692.clone();
                break 'ok___7272;
            }
            sqlFrag__624 = panic!();
        }
        let s__625: std::sync::Arc<String> = sqlFrag__624.to_string();
        let mut t___6640: bool = Some(s__625.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___50 {
            s__625: std::sync::Arc<String>
        }
        impl ClosureGroup___50 {
            fn fn__6630(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__625.clone()));
            }
        }
        let closure_group = ClosureGroup___50 {
            s__625: s__625.clone()
        };
        let fn__6630 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6630())
        };
        test___43.assert(t___6640, fn__6630.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__1200() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let params__627: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___6623: TableDef = userTable__399();
        let mut t___6624: SafeIdentifier = csid__398("name");
        let cs__628: Changeset = changeset(t___6623.clone(), params__627.clone()).cast(std::sync::Arc::new(vec![t___6624.clone()])).validate_required(std::sync::Arc::new(vec![csid__398("name")]));
        let didBubble__629: bool;
        'ok___7273: {
            'orelse___1417: {
                match cs__628.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___1417
                };
                didBubble__629 = false;
                break 'ok___7273;
            }
            didBubble__629 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___51 {}
        impl ClosureGroup___51 {
            fn fn__6621(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___51 {};
        let fn__6621 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6621())
        };
        test___44.assert(didBubble__629, fn__6621.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__1237() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let q__794: Query = from(sid__400("users"));
        let mut t___6444: bool = Some(q__794.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__6439(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__6439 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6439())
        };
        test___45.assert(t___6444, fn__6439.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__1238() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let mut t___6430: SafeIdentifier = sid__400("users");
        let mut t___6431: SafeIdentifier = sid__400("id");
        let mut t___6432: SafeIdentifier = sid__400("name");
        let q__796: Query = from(t___6430.clone()).select([t___6431.clone(), t___6432.clone()]);
        let mut t___6437: bool = Some(q__796.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___53 {}
        impl ClosureGroup___53 {
            fn fn__6429(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___53 {};
        let fn__6429 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6429())
        };
        test___46.assert(t___6437, fn__6429.clone());
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__1239() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let mut t___6418: SafeIdentifier = sid__400("users");
        let mut t___6419: SqlBuilder = SqlBuilder::new();
        t___6419.append_safe("age > ");
        t___6419.append_int32(18);
        let mut t___6422: SqlFragment = t___6419.accumulated();
        let q__798: Query = from(t___6418.clone()).r#where(t___6422.clone());
        let mut t___6427: bool = Some(q__798.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__6417(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__6417 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6417())
        };
        test___47.assert(t___6427, fn__6417.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__1241() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___6406: SafeIdentifier = sid__400("users");
        let mut t___6407: SqlBuilder = SqlBuilder::new();
        t___6407.append_safe("active = ");
        t___6407.append_boolean(true);
        let mut t___6410: SqlFragment = t___6407.accumulated();
        let q__800: Query = from(t___6406.clone()).r#where(t___6410.clone());
        let mut t___6415: bool = Some(q__800.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__6405(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__6405 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6405())
        };
        test___48.assert(t___6415, fn__6405.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__1243() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___6389: SafeIdentifier = sid__400("users");
        let mut t___6390: SqlBuilder = SqlBuilder::new();
        t___6390.append_safe("age > ");
        t___6390.append_int32(18);
        let mut t___6393: SqlFragment = t___6390.accumulated();
        let mut t___6394: Query = from(t___6389.clone()).r#where(t___6393.clone());
        let mut t___6395: SqlBuilder = SqlBuilder::new();
        t___6395.append_safe("active = ");
        t___6395.append_boolean(true);
        let q__802: Query = t___6394.r#where(t___6395.accumulated());
        let mut t___6403: bool = Some(q__802.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__6388(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__6388 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6388())
        };
        test___49.assert(t___6403, fn__6388.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__1246() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___6380: SafeIdentifier = sid__400("users");
        let mut t___6381: SafeIdentifier = sid__400("name");
        let q__804: Query = from(t___6380.clone()).order_by(t___6381.clone(), true);
        let mut t___6386: bool = Some(q__804.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__6379(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__6379 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6379())
        };
        test___50.assert(t___6386, fn__6379.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__1247() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let mut t___6371: SafeIdentifier = sid__400("users");
        let mut t___6372: SafeIdentifier = sid__400("created_at");
        let q__806: Query = from(t___6371.clone()).order_by(t___6372.clone(), false);
        let mut t___6377: bool = Some(q__806.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__6370(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__6370 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6370())
        };
        test___51.assert(t___6377, fn__6370.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__1248() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___3423: Query;
        let mut t___3424: Query;
        let q__808: Query;
        'ok___7275: {
            'orelse___1419: {
                t___3423 = match from(sid__400("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1419
                };
                t___3424 = match t___3423.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1419
                };
                q__808 = t___3424.clone();
                break 'ok___7275;
            }
            q__808 = panic!();
        }
        let mut t___6368: bool = Some(q__808.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__6363(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__6363 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6363())
        };
        test___52.assert(t___6368, fn__6363.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__1249() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let didBubble__810: bool;
        'ok___7276: {
            'orelse___1420: {
                match from(sid__400("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1420
                };
                didBubble__810 = false;
                break 'ok___7276;
            }
            didBubble__810 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__6359(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__6359 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6359())
        };
        test___53.assert(didBubble__810, fn__6359.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__1250() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let didBubble__812: bool;
        'ok___7277: {
            'orelse___1421: {
                match from(sid__400("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1421
                };
                didBubble__812 = false;
                break 'ok___7277;
            }
            didBubble__812 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__6355(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__6355 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6355())
        };
        test___54.assert(didBubble__812, fn__6355.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__1251() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let mut t___6333: SafeIdentifier;
        let mut t___6334: SafeIdentifier;
        let mut t___6335: SafeIdentifier;
        let mut t___6336: SafeIdentifier;
        let mut t___6337: Query;
        let mut t___6338: SqlBuilder;
        let mut t___6342: Query;
        let mut t___6343: SqlBuilder;
        let mut t___3409: Query;
        let mut t___3410: Query;
        let minAge__814: i32 = 21;
        let q__815: Query;
        'ok___7278: {
            'orelse___1422: {
                t___6333 = sid__400("users");
                t___6334 = sid__400("id");
                t___6335 = sid__400("name");
                t___6336 = sid__400("email");
                t___6337 = from(t___6333.clone()).select([t___6334.clone(), t___6335.clone(), t___6336.clone()]);
                t___6338 = SqlBuilder::new();
                t___6338.append_safe("age >= ");
                t___6338.append_int32(21);
                t___6342 = t___6337.r#where(t___6338.accumulated());
                t___6343 = SqlBuilder::new();
                t___6343.append_safe("active = ");
                t___6343.append_boolean(true);
                t___3409 = match t___6342.r#where(t___6343.accumulated()).order_by(sid__400("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___1422
                };
                t___3410 = match t___3409.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___1422
                };
                q__815 = t___3410.clone();
                break 'ok___7278;
            }
            q__815 = panic!();
        }
        let mut t___6353: bool = Some(q__815.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__6332(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__6332 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6332())
        };
        test___55.assert(t___6353, fn__6332.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__1254() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let mut t___3386: SqlFragment;
        let mut t___3387: SqlFragment;
        let q__817: Query = from(sid__400("users"));
        'ok___7279: {
            'orelse___1423: {
                t___3386 = match q__817.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1423
                };
                t___3387 = t___3386.clone();
                break 'ok___7279;
            }
            t___3387 = panic!();
        }
        let s__818: std::sync::Arc<String> = t___3387.to_string();
        let mut t___6330: bool = Some(s__818.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___63 {
            s__818: std::sync::Arc<String>
        }
        impl ClosureGroup___63 {
            fn fn__6326(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__818.clone()));
            }
        }
        let closure_group = ClosureGroup___63 {
            s__818: s__818.clone()
        };
        let fn__6326 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6326())
        };
        test___56.assert(t___6330, fn__6326.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__1255() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___3378: Query;
        let mut t___3381: SqlFragment;
        let mut t___3382: SqlFragment;
        let q__820: Query;
        'ok___7280: {
            'orelse___1424: {
                t___3378 = match from(sid__400("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___1424
                };
                q__820 = t___3378.clone();
                break 'ok___7280;
            }
            q__820 = panic!();
        }
        'ok___7281: {
            'orelse___1425: {
                t___3381 = match q__820.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1425
                };
                t___3382 = t___3381.clone();
                break 'ok___7281;
            }
            t___3382 = panic!();
        }
        let s__821: std::sync::Arc<String> = t___3382.to_string();
        let mut t___6324: bool = Some(s__821.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___64 {
            s__821: std::sync::Arc<String>
        }
        impl ClosureGroup___64 {
            fn fn__6320(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__821.clone()));
            }
        }
        let closure_group = ClosureGroup___64 {
            s__821: s__821.clone()
        };
        let fn__6320 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6320())
        };
        test___57.assert(t___6324, fn__6320.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__1256() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let didBubble__823: bool;
        'ok___7282: {
            'orelse___1426: {
                match from(sid__400("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1426
                };
                didBubble__823 = false;
                break 'ok___7282;
            }
            didBubble__823 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__6316(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__6316 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6316())
        };
        test___58.assert(didBubble__823, fn__6316.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__1257() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let evil__825: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___6300: SafeIdentifier = sid__400("users");
        let mut t___6301: SqlBuilder = SqlBuilder::new();
        t___6301.append_safe("name = ");
        t___6301.append_string("'; DROP TABLE users; --");
        let mut t___6304: SqlFragment = t___6301.accumulated();
        let q__826: Query = from(t___6300.clone()).r#where(t___6304.clone());
        let s__827: std::sync::Arc<String> = q__826.to_sql().to_string();
        let mut t___6309: bool = temper_core::string::index_of( & s__827, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___66 {
            s__827: std::sync::Arc<String>
        }
        impl ClosureGroup___66 {
            fn fn__6299(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__827.clone()));
            }
        }
        let closure_group = ClosureGroup___66 {
            s__827: s__827.clone()
        };
        let fn__6299 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6299())
        };
        test___59.assert(t___6309, fn__6299.clone());
        let mut t___6313: bool = temper_core::string::index_of( & s__827, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___67 {
            s__827: std::sync::Arc<String>
        }
        impl ClosureGroup___67 {
            fn fn__6298(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__827.clone()));
            }
        }
        let closure_group = ClosureGroup___67 {
            s__827: s__827.clone()
        };
        let fn__6298 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6298())
        };
        test___59.assert(t___6313, fn__6298.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__1259() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let attack__829: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__830: bool;
        'ok___7283: {
            'orelse___1427: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___1427
                };
                didBubble__830 = false;
                break 'ok___7283;
            }
            didBubble__830 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__6295(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__6295 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6295())
        };
        test___60.assert(didBubble__830, fn__6295.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__1260() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let mut t___6284: SafeIdentifier = sid__400("users");
        let mut t___6285: SafeIdentifier = sid__400("orders");
        let mut t___6286: SqlBuilder = SqlBuilder::new();
        t___6286.append_safe("users.id = orders.user_id");
        let mut t___6288: SqlFragment = t___6286.accumulated();
        let q__832: Query = from(t___6284.clone()).inner_join(t___6285.clone(), t___6288.clone());
        let mut t___6293: bool = Some(q__832.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___69 {}
        impl ClosureGroup___69 {
            fn fn__6283(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___69 {};
        let fn__6283 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6283())
        };
        test___61.assert(t___6293, fn__6283.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__1262() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let mut t___6272: SafeIdentifier = sid__400("users");
        let mut t___6273: SafeIdentifier = sid__400("profiles");
        let mut t___6274: SqlBuilder = SqlBuilder::new();
        t___6274.append_safe("users.id = profiles.user_id");
        let mut t___6276: SqlFragment = t___6274.accumulated();
        let q__834: Query = from(t___6272.clone()).left_join(t___6273.clone(), t___6276.clone());
        let mut t___6281: bool = Some(q__834.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___70 {}
        impl ClosureGroup___70 {
            fn fn__6271(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___70 {};
        let fn__6271 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6271())
        };
        test___62.assert(t___6281, fn__6271.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__1264() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let mut t___6260: SafeIdentifier = sid__400("orders");
        let mut t___6261: SafeIdentifier = sid__400("users");
        let mut t___6262: SqlBuilder = SqlBuilder::new();
        t___6262.append_safe("orders.user_id = users.id");
        let mut t___6264: SqlFragment = t___6262.accumulated();
        let q__836: Query = from(t___6260.clone()).right_join(t___6261.clone(), t___6264.clone());
        let mut t___6269: bool = Some(q__836.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__6259(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__6259 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6259())
        };
        test___63.assert(t___6269, fn__6259.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__1266() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let mut t___6248: SafeIdentifier = sid__400("users");
        let mut t___6249: SafeIdentifier = sid__400("orders");
        let mut t___6250: SqlBuilder = SqlBuilder::new();
        t___6250.append_safe("users.id = orders.user_id");
        let mut t___6252: SqlFragment = t___6250.accumulated();
        let q__838: Query = from(t___6248.clone()).full_join(t___6249.clone(), t___6252.clone());
        let mut t___6257: bool = Some(q__838.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__6247(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__6247 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6247())
        };
        test___64.assert(t___6257, fn__6247.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__1268() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let mut t___6231: SafeIdentifier = sid__400("users");
        let mut t___6232: SafeIdentifier = sid__400("orders");
        let mut t___6233: SqlBuilder = SqlBuilder::new();
        t___6233.append_safe("users.id = orders.user_id");
        let mut t___6235: SqlFragment = t___6233.accumulated();
        let mut t___6236: Query = from(t___6231.clone()).inner_join(t___6232.clone(), t___6235.clone());
        let mut t___6237: SafeIdentifier = sid__400("profiles");
        let mut t___6238: SqlBuilder = SqlBuilder::new();
        t___6238.append_safe("users.id = profiles.user_id");
        let q__840: Query = t___6236.left_join(t___6237.clone(), t___6238.accumulated());
        let mut t___6245: bool = Some(q__840.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__6230(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__6230 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6230())
        };
        test___65.assert(t___6245, fn__6230.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__1271() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let mut t___6212: SafeIdentifier;
        let mut t___6213: SafeIdentifier;
        let mut t___6214: SqlBuilder;
        let mut t___6216: SqlFragment;
        let mut t___6217: Query;
        let mut t___6218: SqlBuilder;
        let mut t___3293: Query;
        let q__842: Query;
        'ok___7284: {
            'orelse___1428: {
                t___6212 = sid__400("users");
                t___6213 = sid__400("orders");
                t___6214 = SqlBuilder::new();
                t___6214.append_safe("users.id = orders.user_id");
                t___6216 = t___6214.accumulated();
                t___6217 = from(t___6212.clone()).inner_join(t___6213.clone(), t___6216.clone());
                t___6218 = SqlBuilder::new();
                t___6218.append_safe("orders.total > ");
                t___6218.append_int32(100);
                t___3293 = match t___6217.r#where(t___6218.accumulated()).order_by(sid__400("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1428
                };
                q__842 = t___3293.clone();
                break 'ok___7284;
            }
            q__842 = panic!();
        }
        let mut t___6228: bool = Some(q__842.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__6211(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__6211 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6211())
        };
        test___66.assert(t___6228, fn__6211.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__1274() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let c__844: SqlFragment = col(sid__400("users"), sid__400("id"));
        let mut t___6209: bool = Some(c__844.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__6203(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__6203 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6203())
        };
        test___67.assert(t___6209, fn__6203.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__1275() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let onCond__846: SqlFragment = col(sid__400("users"), sid__400("id"));
        let b__847: SqlBuilder = SqlBuilder::new();
        b__847.append_fragment(onCond__846.clone());
        b__847.append_safe(" = ");
        b__847.append_fragment(col(sid__400("orders"), sid__400("user_id")));
        let mut t___6194: SafeIdentifier = sid__400("users");
        let mut t___6195: SafeIdentifier = sid__400("orders");
        let mut t___6196: SqlFragment = b__847.accumulated();
        let q__848: Query = from(t___6194.clone()).inner_join(t___6195.clone(), t___6196.clone());
        let mut t___6201: bool = Some(q__848.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__6183(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__6183 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6183())
        };
        test___68.assert(t___6201, fn__6183.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn orWhereBasic__1276() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let mut t___6172: SafeIdentifier = sid__400("users");
        let mut t___6173: SqlBuilder = SqlBuilder::new();
        t___6173.append_safe("status = ");
        t___6173.append_string("active");
        let mut t___6176: SqlFragment = t___6173.accumulated();
        let q__850: Query = from(t___6172.clone()).or_where(t___6176.clone());
        let mut t___6181: bool = Some(q__850.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE status = 'active'");
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__6171(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orWhere basic".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__6171 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6171())
        };
        test___69.assert(t___6181, fn__6171.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn whereThenOrWhere__1278() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let mut t___6155: SafeIdentifier = sid__400("users");
        let mut t___6156: SqlBuilder = SqlBuilder::new();
        t___6156.append_safe("age > ");
        t___6156.append_int32(18);
        let mut t___6159: SqlFragment = t___6156.accumulated();
        let mut t___6160: Query = from(t___6155.clone()).r#where(t___6159.clone());
        let mut t___6161: SqlBuilder = SqlBuilder::new();
        t___6161.append_safe("vip = ");
        t___6161.append_boolean(true);
        let q__852: Query = t___6160.or_where(t___6161.accumulated());
        let mut t___6169: bool = Some(q__852.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__6154(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where then orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__6154 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6154())
        };
        test___70.assert(t___6169, fn__6154.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn multipleOrWhere__1281() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let mut t___6133: SafeIdentifier = sid__400("users");
        let mut t___6134: SqlBuilder = SqlBuilder::new();
        t___6134.append_safe("active = ");
        t___6134.append_boolean(true);
        let mut t___6137: SqlFragment = t___6134.accumulated();
        let mut t___6138: Query = from(t___6133.clone()).r#where(t___6137.clone());
        let mut t___6139: SqlBuilder = SqlBuilder::new();
        t___6139.append_safe("role = ");
        t___6139.append_string("admin");
        let mut t___6143: Query = t___6138.or_where(t___6139.accumulated());
        let mut t___6144: SqlBuilder = SqlBuilder::new();
        t___6144.append_safe("role = ");
        t___6144.append_string("moderator");
        let q__854: Query = t___6143.or_where(t___6144.accumulated());
        let mut t___6152: bool = Some(q__854.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'");
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__6132(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("multiple orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__6132 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6132())
        };
        test___71.assert(t___6152, fn__6132.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn mixedWhereAndOrWhere__1285() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let mut t___6111: SafeIdentifier = sid__400("users");
        let mut t___6112: SqlBuilder = SqlBuilder::new();
        t___6112.append_safe("age > ");
        t___6112.append_int32(18);
        let mut t___6115: SqlFragment = t___6112.accumulated();
        let mut t___6116: Query = from(t___6111.clone()).r#where(t___6115.clone());
        let mut t___6117: SqlBuilder = SqlBuilder::new();
        t___6117.append_safe("active = ");
        t___6117.append_boolean(true);
        let mut t___6121: Query = t___6116.r#where(t___6117.accumulated());
        let mut t___6122: SqlBuilder = SqlBuilder::new();
        t___6122.append_safe("vip = ");
        t___6122.append_boolean(true);
        let q__856: Query = t___6121.or_where(t___6122.accumulated());
        let mut t___6130: bool = Some(q__856.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___80 {}
        impl ClosureGroup___80 {
            fn fn__6110(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed where and orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___80 {};
        let fn__6110 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6110())
        };
        test___72.assert(t___6130, fn__6110.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn whereNull__1289() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let mut t___6102: SafeIdentifier = sid__400("users");
        let mut t___6103: SafeIdentifier = sid__400("deleted_at");
        let q__858: Query = from(t___6102.clone()).where_null(t___6103.clone());
        let mut t___6108: bool = Some(q__858.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___81 {}
        impl ClosureGroup___81 {
            fn fn__6101(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull".to_string());
            }
        }
        let closure_group = ClosureGroup___81 {};
        let fn__6101 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6101())
        };
        test___73.assert(t___6108, fn__6101.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNull__1290() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let mut t___6093: SafeIdentifier = sid__400("users");
        let mut t___6094: SafeIdentifier = sid__400("email");
        let q__860: Query = from(t___6093.clone()).where_not_null(t___6094.clone());
        let mut t___6099: bool = Some(q__860.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email IS NOT NULL");
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__6092(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__6092 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6092())
        };
        test___74.assert(t___6099, fn__6092.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn whereNullChainedWithWhere__1291() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let mut t___6079: SafeIdentifier = sid__400("users");
        let mut t___6080: SqlBuilder = SqlBuilder::new();
        t___6080.append_safe("active = ");
        t___6080.append_boolean(true);
        let mut t___6083: SqlFragment = t___6080.accumulated();
        let q__862: Query = from(t___6079.clone()).r#where(t___6083.clone()).where_null(sid__400("deleted_at"));
        let mut t___6090: bool = Some(q__862.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__6078(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull chained".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__6078 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6078())
        };
        test___75.assert(t___6090, fn__6078.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNullChainedWithOrWhere__1293() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        let mut t___6065: SafeIdentifier = sid__400("users");
        let mut t___6066: SafeIdentifier = sid__400("deleted_at");
        let mut t___6067: Query = from(t___6065.clone()).where_null(t___6066.clone());
        let mut t___6068: SqlBuilder = SqlBuilder::new();
        t___6068.append_safe("role = ");
        t___6068.append_string("admin");
        let q__864: Query = t___6067.or_where(t___6068.accumulated());
        let mut t___6076: bool = Some(q__864.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'");
        #[derive(Clone)]
        struct ClosureGroup___84 {}
        impl ClosureGroup___84 {
            fn fn__6064(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull with orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___84 {};
        let fn__6064 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6064())
        };
        test___76.assert(t___6076, fn__6064.clone());
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithIntValues__1295() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let mut t___6053: SafeIdentifier = sid__400("users");
        let mut t___6054: SafeIdentifier = sid__400("id");
        let mut t___6055: SqlInt32 = SqlInt32::new(1);
        let mut t___6056: SqlInt32 = SqlInt32::new(2);
        let mut t___6057: SqlInt32 = SqlInt32::new(3);
        let q__866: Query = from(t___6053.clone()).where_in(t___6054.clone(), [SqlPart::new(t___6055.clone()), SqlPart::new(t___6056.clone()), SqlPart::new(t___6057.clone())]);
        let mut t___6062: bool = Some(q__866.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___85 {}
        impl ClosureGroup___85 {
            fn fn__6052(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn ints".to_string());
            }
        }
        let closure_group = ClosureGroup___85 {};
        let fn__6052 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6052())
        };
        test___77.assert(t___6062, fn__6052.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithStringValuesEscaping__1296() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let mut t___6042: SafeIdentifier = sid__400("users");
        let mut t___6043: SafeIdentifier = sid__400("name");
        let mut t___6044: SqlString = SqlString::new("Alice");
        let mut t___6045: SqlString = SqlString::new("Bob's");
        let q__868: Query = from(t___6042.clone()).where_in(t___6043.clone(), [SqlPart::new(t___6044.clone()), SqlPart::new(t___6045.clone())]);
        let mut t___6050: bool = Some(q__868.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')");
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn fn__6041(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn strings".to_string());
            }
        }
        let closure_group = ClosureGroup___86 {};
        let fn__6041 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6041())
        };
        test___78.assert(t___6050, fn__6041.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithEmptyListProduces1_0__1297() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let mut t___6033: SafeIdentifier = sid__400("users");
        let mut t___6034: SafeIdentifier = sid__400("id");
        let q__870: Query = from(t___6033.clone()).where_in(t___6034.clone(), []);
        let mut t___6039: bool = Some(q__870.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___87 {}
        impl ClosureGroup___87 {
            fn fn__6032(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn empty".to_string());
            }
        }
        let closure_group = ClosureGroup___87 {};
        let fn__6032 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6032())
        };
        test___79.assert(t___6039, fn__6032.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn whereInChained__1298() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let mut t___6017: SafeIdentifier = sid__400("users");
        let mut t___6018: SqlBuilder = SqlBuilder::new();
        t___6018.append_safe("active = ");
        t___6018.append_boolean(true);
        let mut t___6021: SqlFragment = t___6018.accumulated();
        let q__872: Query = from(t___6017.clone()).r#where(t___6021.clone()).where_in(sid__400("role"), [SqlPart::new(SqlString::new("admin")), SqlPart::new(SqlString::new("user"))]);
        let mut t___6030: bool = Some(q__872.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')");
        #[derive(Clone)]
        struct ClosureGroup___88 {}
        impl ClosureGroup___88 {
            fn fn__6016(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn chained".to_string());
            }
        }
        let closure_group = ClosureGroup___88 {};
        let fn__6016 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6016())
        };
        test___80.assert(t___6030, fn__6016.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn whereInSingleElement__1300() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let mut t___6007: SafeIdentifier = sid__400("users");
        let mut t___6008: SafeIdentifier = sid__400("id");
        let mut t___6009: SqlInt32 = SqlInt32::new(42);
        let q__874: Query = from(t___6007.clone()).where_in(t___6008.clone(), [SqlPart::new(t___6009.clone())]);
        let mut t___6014: bool = Some(q__874.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__6006(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn single".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__6006 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6006())
        };
        test___81.assert(t___6014, fn__6006.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn whereNotBasic__1301() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let mut t___5995: SafeIdentifier = sid__400("users");
        let mut t___5996: SqlBuilder = SqlBuilder::new();
        t___5996.append_safe("active = ");
        t___5996.append_boolean(true);
        let mut t___5999: SqlFragment = t___5996.accumulated();
        let q__876: Query = from(t___5995.clone()).where_not(t___5999.clone());
        let mut t___6004: bool = Some(q__876.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE NOT (active = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___90 {}
        impl ClosureGroup___90 {
            fn fn__5994(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot".to_string());
            }
        }
        let closure_group = ClosureGroup___90 {};
        let fn__5994 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5994())
        };
        test___82.assert(t___6004, fn__5994.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn whereNotChained__1303() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let mut t___5978: SafeIdentifier = sid__400("users");
        let mut t___5979: SqlBuilder = SqlBuilder::new();
        t___5979.append_safe("age > ");
        t___5979.append_int32(18);
        let mut t___5982: SqlFragment = t___5979.accumulated();
        let mut t___5983: Query = from(t___5978.clone()).r#where(t___5982.clone());
        let mut t___5984: SqlBuilder = SqlBuilder::new();
        t___5984.append_safe("banned = ");
        t___5984.append_boolean(true);
        let q__878: Query = t___5983.where_not(t___5984.accumulated());
        let mut t___5992: bool = Some(q__878.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___91 {}
        impl ClosureGroup___91 {
            fn fn__5977(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot chained".to_string());
            }
        }
        let closure_group = ClosureGroup___91 {};
        let fn__5977 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5977())
        };
        test___83.assert(t___5992, fn__5977.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenIntegers__1306() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let mut t___5967: SafeIdentifier = sid__400("users");
        let mut t___5968: SafeIdentifier = sid__400("age");
        let mut t___5969: SqlInt32 = SqlInt32::new(18);
        let mut t___5970: SqlInt32 = SqlInt32::new(65);
        let q__880: Query = from(t___5967.clone()).where_between(t___5968.clone(), SqlPart::new(t___5969.clone()), SqlPart::new(t___5970.clone()));
        let mut t___5975: bool = Some(q__880.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age BETWEEN 18 AND 65");
        #[derive(Clone)]
        struct ClosureGroup___92 {}
        impl ClosureGroup___92 {
            fn fn__5966(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween ints".to_string());
            }
        }
        let closure_group = ClosureGroup___92 {};
        let fn__5966 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5966())
        };
        test___84.assert(t___5975, fn__5966.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenChained__1307() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let mut t___5951: SafeIdentifier = sid__400("users");
        let mut t___5952: SqlBuilder = SqlBuilder::new();
        t___5952.append_safe("active = ");
        t___5952.append_boolean(true);
        let mut t___5955: SqlFragment = t___5952.accumulated();
        let q__882: Query = from(t___5951.clone()).r#where(t___5955.clone()).where_between(sid__400("age"), SqlPart::new(SqlInt32::new(21)), SqlPart::new(SqlInt32::new(30)));
        let mut t___5964: bool = Some(q__882.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30");
        #[derive(Clone)]
        struct ClosureGroup___93 {}
        impl ClosureGroup___93 {
            fn fn__5950(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween chained".to_string());
            }
        }
        let closure_group = ClosureGroup___93 {};
        let fn__5950 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5950())
        };
        test___85.assert(t___5964, fn__5950.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeBasic__1309() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let mut t___5942: SafeIdentifier = sid__400("users");
        let mut t___5943: SafeIdentifier = sid__400("name");
        let q__884: Query = from(t___5942.clone()).where_like(t___5943.clone(), "John%");
        let mut t___5948: bool = Some(q__884.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE 'John%'");
        #[derive(Clone)]
        struct ClosureGroup___94 {}
        impl ClosureGroup___94 {
            fn fn__5941(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike".to_string());
            }
        }
        let closure_group = ClosureGroup___94 {};
        let fn__5941 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5941())
        };
        test___86.assert(t___5948, fn__5941.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn whereIlikeBasic__1310() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let mut t___5933: SafeIdentifier = sid__400("users");
        let mut t___5934: SafeIdentifier = sid__400("email");
        let q__886: Query = from(t___5933.clone()).where_i_like(t___5934.clone(), "%@gmail.com");
        let mut t___5939: bool = Some(q__886.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email ILIKE '%@gmail.com'");
        #[derive(Clone)]
        struct ClosureGroup___95 {}
        impl ClosureGroup___95 {
            fn fn__5932(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereILike".to_string());
            }
        }
        let closure_group = ClosureGroup___95 {};
        let fn__5932 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5932())
        };
        test___87.assert(t___5939, fn__5932.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWithInjectionAttempt__1311() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let mut t___5919: SafeIdentifier = sid__400("users");
        let mut t___5920: SafeIdentifier = sid__400("name");
        let q__888: Query = from(t___5919.clone()).where_like(t___5920.clone(), "'; DROP TABLE users; --");
        let s__889: std::sync::Arc<String> = q__888.to_sql().to_string();
        let mut t___5925: bool = temper_core::string::index_of( & s__889, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___96 {
            s__889: std::sync::Arc<String>
        }
        impl ClosureGroup___96 {
            fn fn__5918(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like injection escaped: {}", self.s__889.clone()));
            }
        }
        let closure_group = ClosureGroup___96 {
            s__889: s__889.clone()
        };
        let fn__5918 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5918())
        };
        test___88.assert(t___5925, fn__5918.clone());
        let mut t___5929: bool = temper_core::string::index_of( & s__889, "LIKE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___97 {
            s__889: std::sync::Arc<String>
        }
        impl ClosureGroup___97 {
            fn fn__5917(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like structure intact: {}", self.s__889.clone()));
            }
        }
        let closure_group = ClosureGroup___97 {
            s__889: s__889.clone()
        };
        let fn__5917 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5917())
        };
        test___88.assert(t___5929, fn__5917.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWildcardPatterns__1312() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let mut t___5909: SafeIdentifier = sid__400("users");
        let mut t___5910: SafeIdentifier = sid__400("name");
        let q__891: Query = from(t___5909.clone()).where_like(t___5910.clone(), "%son%");
        let mut t___5915: bool = Some(q__891.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE '%son%'");
        #[derive(Clone)]
        struct ClosureGroup___98 {}
        impl ClosureGroup___98 {
            fn fn__5908(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike wildcard".to_string());
            }
        }
        let closure_group = ClosureGroup___98 {};
        let fn__5908 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5908())
        };
        test___89.assert(t___5915, fn__5908.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__1313() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___96 = temper_std::testing::Test::new();
        let mut t___3023: SafeIdentifier;
        let id__929: SafeIdentifier;
        'ok___7285: {
            'orelse___1429: {
                t___3023 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___1429
                };
                id__929 = t___3023.clone();
                break 'ok___7285;
            }
            id__929 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___5906: bool = Some(id__929.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___99 {}
        impl ClosureGroup___99 {
            fn fn__5903(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___99 {};
        let fn__5903 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5903())
        };
        test___96.assert(t___5906, fn__5903.clone());
        test___96.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__1314() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___97 = temper_std::testing::Test::new();
        let didBubble__931: bool;
        'ok___7286: {
            'orelse___1430: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___1430
                };
                didBubble__931 = false;
                break 'ok___7286;
            }
            didBubble__931 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___100 {}
        impl ClosureGroup___100 {
            fn fn__5900(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___100 {};
        let fn__5900 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5900())
        };
        test___97.assert(didBubble__931, fn__5900.clone());
        test___97.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__1315() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___98 = temper_std::testing::Test::new();
        let didBubble__933: bool;
        'ok___7287: {
            'orelse___1431: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___1431
                };
                didBubble__933 = false;
                break 'ok___7287;
            }
            didBubble__933 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___101 {}
        impl ClosureGroup___101 {
            fn fn__5897(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___101 {};
        let fn__5897 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5897())
        };
        test___98.assert(didBubble__933, fn__5897.clone());
        test___98.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__1316() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___99 = temper_std::testing::Test::new();
        let cases__935: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___102 {
            test___99: temper_std::testing::Test
        }
        impl ClosureGroup___102 {
            fn fn__5894(& self, c__936: impl temper_core::ToArcString) {
                let c__936 = c__936.to_arc_string();
                let didBubble__937: bool;
                'ok___7288: {
                    'orelse___1432: {
                        match safe_identifier(c__936.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___1432
                        };
                        didBubble__937 = false;
                        break 'ok___7288;
                    }
                    didBubble__937 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___103 {
                    c__936: std::sync::Arc<String>
                }
                impl ClosureGroup___103 {
                    fn fn__5891(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__936.clone()));
                    }
                }
                let closure_group = ClosureGroup___103 {
                    c__936: c__936.clone()
                };
                let fn__5891 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__5891())
                };
                self.test___99.assert(didBubble__937, fn__5891.clone());
            }
        }
        let closure_group = ClosureGroup___102 {
            test___99: test___99.clone()
        };
        let fn__5894 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__936: std::sync::Arc<String> | closure_group.fn__5894(c__936))
        };
        temper_core::listed::list_for_each( & cases__935, & ( * fn__5894.clone()));
        test___99.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__1317() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___100 = temper_std::testing::Test::new();
        let mut t___3000: SafeIdentifier;
        let mut t___3001: SafeIdentifier;
        let mut t___3002: SafeIdentifier;
        let mut t___3003: SafeIdentifier;
        let mut t___3006: SafeIdentifier;
        let mut t___3007: SafeIdentifier;
        let mut t___3011: FieldDef;
        'ok___7289: {
            'orelse___1433: {
                t___3000 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1433
                };
                t___3001 = t___3000.clone();
                break 'ok___7289;
            }
            t___3001 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___7290: {
            'orelse___1434: {
                t___3002 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1434
                };
                t___3003 = t___3002.clone();
                break 'ok___7290;
            }
            t___3003 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___5881: StringField = StringField::new();
        let mut t___5882: FieldDef = FieldDef::new(t___3003.clone(), FieldType::new(t___5881.clone()), false);
        'ok___7291: {
            'orelse___1435: {
                t___3006 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1435
                };
                t___3007 = t___3006.clone();
                break 'ok___7291;
            }
            t___3007 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___5883: IntField = IntField::new();
        let mut t___5884: FieldDef = FieldDef::new(t___3007.clone(), FieldType::new(t___5883.clone()), false);
        let td__939: TableDef = TableDef::new(t___3001.clone(), [t___5882.clone(), t___5884.clone()]);
        let f__940: FieldDef;
        'ok___7292: {
            'orelse___1436: {
                t___3011 = match td__939.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1436
                };
                f__940 = t___3011.clone();
                break 'ok___7292;
            }
            f__940 = panic!();
        }
        let mut t___5889: bool = Some(f__940.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___104 {}
        impl ClosureGroup___104 {
            fn fn__5880(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___104 {};
        let fn__5880 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5880())
        };
        test___100.assert(t___5889, fn__5880.clone());
        test___100.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__1318() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___101 = temper_std::testing::Test::new();
        let mut t___2991: SafeIdentifier;
        let mut t___2992: SafeIdentifier;
        let mut t___2993: SafeIdentifier;
        let mut t___2994: SafeIdentifier;
        'ok___7293: {
            'orelse___1437: {
                t___2991 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1437
                };
                t___2992 = t___2991.clone();
                break 'ok___7293;
            }
            t___2992 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___7294: {
            'orelse___1438: {
                t___2993 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1438
                };
                t___2994 = t___2993.clone();
                break 'ok___7294;
            }
            t___2994 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___5875: StringField = StringField::new();
        let mut t___5876: FieldDef = FieldDef::new(t___2994.clone(), FieldType::new(t___5875.clone()), false);
        let td__942: TableDef = TableDef::new(t___2992.clone(), [t___5876.clone()]);
        let didBubble__943: bool;
        'ok___7295: {
            'orelse___1439: {
                match td__942.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___1439
                };
                didBubble__943 = false;
                break 'ok___7295;
            }
            didBubble__943 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___105 {}
        impl ClosureGroup___105 {
            fn fn__5874(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___105 {};
        let fn__5874 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5874())
        };
        test___101.assert(didBubble__943, fn__5874.clone());
        test___101.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__1319() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___102 = temper_std::testing::Test::new();
        let mut t___2979: SafeIdentifier;
        let mut t___2980: SafeIdentifier;
        let mut t___2983: SafeIdentifier;
        let mut t___2984: SafeIdentifier;
        'ok___7296: {
            'orelse___1440: {
                t___2979 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___1440
                };
                t___2980 = t___2979.clone();
                break 'ok___7296;
            }
            t___2980 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___5863: StringField = StringField::new();
        let required__945: FieldDef = FieldDef::new(t___2980.clone(), FieldType::new(t___5863.clone()), false);
        'ok___7297: {
            'orelse___1441: {
                t___2983 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___1441
                };
                t___2984 = t___2983.clone();
                break 'ok___7297;
            }
            t___2984 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___5865: StringField = StringField::new();
        let optional__946: FieldDef = FieldDef::new(t___2984.clone(), FieldType::new(t___5865.clone()), true);
        let mut t___5869: bool = ! required__945.nullable();
        #[derive(Clone)]
        struct ClosureGroup___106 {}
        impl ClosureGroup___106 {
            fn fn__5862(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___106 {};
        let fn__5862 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5862())
        };
        test___102.assert(t___5869, fn__5862.clone());
        let mut t___5871: bool = optional__946.nullable();
        #[derive(Clone)]
        struct ClosureGroup___107 {}
        impl ClosureGroup___107 {
            fn fn__5861(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___107 {};
        let fn__5861 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5861())
        };
        test___102.assert(t___5871, fn__5861.clone());
        test___102.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__1320() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___106 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___108 {}
        impl ClosureGroup___108 {
            fn build__1072(& self, name__1074: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1074 = name__1074.to_arc_string();
                let mut t___5843: SqlBuilder = SqlBuilder::new();
                t___5843.append_safe("select * from hi where name = ");
                t___5843.append_string(name__1074.clone());
                return t___5843.accumulated().to_string();
            }
            fn buildWrong__1073(& self, name__1076: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1076 = name__1076.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__1076.clone()));
            }
        }
        let closure_group = ClosureGroup___108 {};
        let build__1072 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1074: std::sync::Arc<String> | closure_group.build__1072(name__1074))
        };
        let buildWrong__1073 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1076: std::sync::Arc<String> | closure_group.buildWrong__1073(name__1076))
        };
        let actual___1322: std::sync::Arc<String> = build__1072(std::sync::Arc::new("world".to_string()));
        let mut t___5853: bool = Some(actual___1322.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___109 {
            actual___1322: std::sync::Arc<String>
        }
        impl ClosureGroup___109 {
            fn fn__5850(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___1322.clone()));
            }
        }
        let closure_group = ClosureGroup___109 {
            actual___1322: actual___1322.clone()
        };
        let fn__5850 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5850())
        };
        test___106.assert(t___5853, fn__5850.clone());
        let bobbyTables__1078: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___1324: std::sync::Arc<String> = build__1072(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___5857: bool = Some(actual___1324.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___110 {
            actual___1324: std::sync::Arc<String>
        }
        impl ClosureGroup___110 {
            fn fn__5849(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___1324.clone()));
            }
        }
        let closure_group = ClosureGroup___110 {
            actual___1324: actual___1324.clone()
        };
        let fn__5849 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5849())
        };
        test___106.assert(t___5857, fn__5849.clone());
        #[derive(Clone)]
        struct ClosureGroup___111 {}
        impl ClosureGroup___111 {
            fn fn__5848(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___111 {};
        let fn__5848 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5848())
        };
        test___106.assert(true, fn__5848.clone());
        test___106.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__1328() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___107 = temper_std::testing::Test::new();
        let mut t___5811: SqlBuilder = SqlBuilder::new();
        t___5811.append_safe("v = ");
        t___5811.append_string("");
        let actual___1329: std::sync::Arc<String> = t___5811.accumulated().to_string();
        let mut t___5817: bool = Some(actual___1329.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___112 {
            actual___1329: std::sync::Arc<String>
        }
        impl ClosureGroup___112 {
            fn fn__5810(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___1329.clone()));
            }
        }
        let closure_group = ClosureGroup___112 {
            actual___1329: actual___1329.clone()
        };
        let fn__5810 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5810())
        };
        test___107.assert(t___5817, fn__5810.clone());
        let mut t___5819: SqlBuilder = SqlBuilder::new();
        t___5819.append_safe("v = ");
        t___5819.append_string("a''b");
        let actual___1332: std::sync::Arc<String> = t___5819.accumulated().to_string();
        let mut t___5825: bool = Some(actual___1332.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___113 {
            actual___1332: std::sync::Arc<String>
        }
        impl ClosureGroup___113 {
            fn fn__5809(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___1332.clone()));
            }
        }
        let closure_group = ClosureGroup___113 {
            actual___1332: actual___1332.clone()
        };
        let fn__5809 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5809())
        };
        test___107.assert(t___5825, fn__5809.clone());
        let mut t___5827: SqlBuilder = SqlBuilder::new();
        t___5827.append_safe("v = ");
        t___5827.append_string("Hello 世界");
        let actual___1335: std::sync::Arc<String> = t___5827.accumulated().to_string();
        let mut t___5833: bool = Some(actual___1335.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___114 {
            actual___1335: std::sync::Arc<String>
        }
        impl ClosureGroup___114 {
            fn fn__5808(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___1335.clone()));
            }
        }
        let closure_group = ClosureGroup___114 {
            actual___1335: actual___1335.clone()
        };
        let fn__5808 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5808())
        };
        test___107.assert(t___5833, fn__5808.clone());
        let mut t___5835: SqlBuilder = SqlBuilder::new();
        t___5835.append_safe("v = ");
        t___5835.append_string("Line1\x0aLine2");
        let actual___1338: std::sync::Arc<String> = t___5835.accumulated().to_string();
        let mut t___5841: bool = Some(actual___1338.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___115 {
            actual___1338: std::sync::Arc<String>
        }
        impl ClosureGroup___115 {
            fn fn__5807(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___1338.clone()));
            }
        }
        let closure_group = ClosureGroup___115 {
            actual___1338: actual___1338.clone()
        };
        let fn__5807 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5807())
        };
        test___107.assert(t___5841, fn__5807.clone());
        test___107.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__1341() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___108 = temper_std::testing::Test::new();
        let mut t___2924: temper_std::temporal::Date;
        let mut t___5782: SqlBuilder = SqlBuilder::new();
        t___5782.append_safe("select ");
        t___5782.append_int32(42);
        t___5782.append_safe(", ");
        t___5782.append_int64(43);
        t___5782.append_safe(", ");
        t___5782.append_float64(19.99f64);
        t___5782.append_safe(", ");
        t___5782.append_boolean(true);
        t___5782.append_safe(", ");
        t___5782.append_boolean(false);
        let actual___1342: std::sync::Arc<String> = t___5782.accumulated().to_string();
        let mut t___5796: bool = Some(actual___1342.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___116 {
            actual___1342: std::sync::Arc<String>
        }
        impl ClosureGroup___116 {
            fn fn__5781(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___1342.clone()));
            }
        }
        let closure_group = ClosureGroup___116 {
            actual___1342: actual___1342.clone()
        };
        let fn__5781 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5781())
        };
        test___108.assert(t___5796, fn__5781.clone());
        let date__1081: temper_std::temporal::Date;
        'ok___7298: {
            'orelse___1442: {
                t___2924 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1442
                };
                date__1081 = t___2924.clone();
                break 'ok___7298;
            }
            date__1081 = panic!();
        }
        let mut t___5798: SqlBuilder = SqlBuilder::new();
        t___5798.append_safe("insert into t values (");
        t___5798.append_date(date__1081.clone());
        t___5798.append_safe(")");
        let actual___1345: std::sync::Arc<String> = t___5798.accumulated().to_string();
        let mut t___5805: bool = Some(actual___1345.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___117 {
            actual___1345: std::sync::Arc<String>
        }
        impl ClosureGroup___117 {
            fn fn__5780(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___1345.clone()));
            }
        }
        let closure_group = ClosureGroup___117 {
            actual___1345: actual___1345.clone()
        };
        let fn__5780 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5780())
        };
        test___108.assert(t___5805, fn__5780.clone());
        test___108.soft_fail_to_hard()
    }
    #[test]
    fn lists__1348() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___109 = temper_std::testing::Test::new();
        let mut t___2896: temper_std::temporal::Date;
        let mut t___2897: temper_std::temporal::Date;
        let mut t___2898: temper_std::temporal::Date;
        let mut t___2899: temper_std::temporal::Date;
        let mut t___5726: SqlBuilder = SqlBuilder::new();
        t___5726.append_safe("v IN (");
        t___5726.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___5726.append_safe(")");
        let actual___1349: std::sync::Arc<String> = t___5726.accumulated().to_string();
        let mut t___5733: bool = Some(actual___1349.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___118 {
            actual___1349: std::sync::Arc<String>
        }
        impl ClosureGroup___118 {
            fn fn__5725(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___1349.clone()));
            }
        }
        let closure_group = ClosureGroup___118 {
            actual___1349: actual___1349.clone()
        };
        let fn__5725 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5725())
        };
        test___109.assert(t___5733, fn__5725.clone());
        let mut t___5735: SqlBuilder = SqlBuilder::new();
        t___5735.append_safe("v IN (");
        t___5735.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___5735.append_safe(")");
        let actual___1352: std::sync::Arc<String> = t___5735.accumulated().to_string();
        let mut t___5742: bool = Some(actual___1352.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___119 {
            actual___1352: std::sync::Arc<String>
        }
        impl ClosureGroup___119 {
            fn fn__5724(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___1352.clone()));
            }
        }
        let closure_group = ClosureGroup___119 {
            actual___1352: actual___1352.clone()
        };
        let fn__5724 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5724())
        };
        test___109.assert(t___5742, fn__5724.clone());
        let mut t___5744: SqlBuilder = SqlBuilder::new();
        t___5744.append_safe("v IN (");
        t___5744.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___5744.append_safe(")");
        let actual___1355: std::sync::Arc<String> = t___5744.accumulated().to_string();
        let mut t___5751: bool = Some(actual___1355.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___120 {
            actual___1355: std::sync::Arc<String>
        }
        impl ClosureGroup___120 {
            fn fn__5723(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___1355.clone()));
            }
        }
        let closure_group = ClosureGroup___120 {
            actual___1355: actual___1355.clone()
        };
        let fn__5723 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5723())
        };
        test___109.assert(t___5751, fn__5723.clone());
        let mut t___5753: SqlBuilder = SqlBuilder::new();
        t___5753.append_safe("v IN (");
        t___5753.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___5753.append_safe(")");
        let actual___1358: std::sync::Arc<String> = t___5753.accumulated().to_string();
        let mut t___5760: bool = Some(actual___1358.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___121 {
            actual___1358: std::sync::Arc<String>
        }
        impl ClosureGroup___121 {
            fn fn__5722(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___1358.clone()));
            }
        }
        let closure_group = ClosureGroup___121 {
            actual___1358: actual___1358.clone()
        };
        let fn__5722 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5722())
        };
        test___109.assert(t___5760, fn__5722.clone());
        let mut t___5762: SqlBuilder = SqlBuilder::new();
        t___5762.append_safe("v IN (");
        t___5762.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___5762.append_safe(")");
        let actual___1361: std::sync::Arc<String> = t___5762.accumulated().to_string();
        let mut t___5769: bool = Some(actual___1361.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___122 {
            actual___1361: std::sync::Arc<String>
        }
        impl ClosureGroup___122 {
            fn fn__5721(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___1361.clone()));
            }
        }
        let closure_group = ClosureGroup___122 {
            actual___1361: actual___1361.clone()
        };
        let fn__5721 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5721())
        };
        test___109.assert(t___5769, fn__5721.clone());
        'ok___7299: {
            'orelse___1443: {
                t___2896 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___1443
                };
                t___2897 = t___2896.clone();
                break 'ok___7299;
            }
            t___2897 = panic!();
        }
        'ok___7300: {
            'orelse___1444: {
                t___2898 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1444
                };
                t___2899 = t___2898.clone();
                break 'ok___7300;
            }
            t___2899 = panic!();
        }
        let dates__1083: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___2897.clone(), t___2899.clone()]);
        let mut t___5771: SqlBuilder = SqlBuilder::new();
        t___5771.append_safe("v IN (");
        t___5771.append_date_list(temper_core::ToListed::to_listed(dates__1083.clone()));
        t___5771.append_safe(")");
        let actual___1364: std::sync::Arc<String> = t___5771.accumulated().to_string();
        let mut t___5778: bool = Some(actual___1364.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___123 {
            actual___1364: std::sync::Arc<String>
        }
        impl ClosureGroup___123 {
            fn fn__5720(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___1364.clone()));
            }
        }
        let closure_group = ClosureGroup___123 {
            actual___1364: actual___1364.clone()
        };
        let fn__5720 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5720())
        };
        test___109.assert(t___5778, fn__5720.clone());
        test___109.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__1367() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___110 = temper_std::testing::Test::new();
        let nan__1085: f64;
        nan__1085 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___5712: SqlBuilder = SqlBuilder::new();
        t___5712.append_safe("v = ");
        t___5712.append_float64(nan__1085);
        let actual___1368: std::sync::Arc<String> = t___5712.accumulated().to_string();
        let mut t___5718: bool = Some(actual___1368.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___124 {
            actual___1368: std::sync::Arc<String>
        }
        impl ClosureGroup___124 {
            fn fn__5711(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___1368.clone()));
            }
        }
        let closure_group = ClosureGroup___124 {
            actual___1368: actual___1368.clone()
        };
        let fn__5711 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5711())
        };
        test___110.assert(t___5718, fn__5711.clone());
        test___110.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__1371() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___111 = temper_std::testing::Test::new();
        let inf__1087: f64;
        inf__1087 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___5703: SqlBuilder = SqlBuilder::new();
        t___5703.append_safe("v = ");
        t___5703.append_float64(inf__1087);
        let actual___1372: std::sync::Arc<String> = t___5703.accumulated().to_string();
        let mut t___5709: bool = Some(actual___1372.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___125 {
            actual___1372: std::sync::Arc<String>
        }
        impl ClosureGroup___125 {
            fn fn__5702(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___1372.clone()));
            }
        }
        let closure_group = ClosureGroup___125 {
            actual___1372: actual___1372.clone()
        };
        let fn__5702 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5702())
        };
        test___111.assert(t___5709, fn__5702.clone());
        test___111.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__1375() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___112 = temper_std::testing::Test::new();
        let ninf__1089: f64;
        ninf__1089 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___5694: SqlBuilder = SqlBuilder::new();
        t___5694.append_safe("v = ");
        t___5694.append_float64(ninf__1089);
        let actual___1376: std::sync::Arc<String> = t___5694.accumulated().to_string();
        let mut t___5700: bool = Some(actual___1376.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___126 {
            actual___1376: std::sync::Arc<String>
        }
        impl ClosureGroup___126 {
            fn fn__5693(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___1376.clone()));
            }
        }
        let closure_group = ClosureGroup___126 {
            actual___1376: actual___1376.clone()
        };
        let fn__5693 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5693())
        };
        test___112.assert(t___5700, fn__5693.clone());
        test___112.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__1379() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___113 = temper_std::testing::Test::new();
        let mut t___5669: SqlBuilder = SqlBuilder::new();
        t___5669.append_safe("v = ");
        t___5669.append_float64(3.14f64);
        let actual___1380: std::sync::Arc<String> = t___5669.accumulated().to_string();
        let mut t___5675: bool = Some(actual___1380.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___127 {
            actual___1380: std::sync::Arc<String>
        }
        impl ClosureGroup___127 {
            fn fn__5668(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___1380.clone()));
            }
        }
        let closure_group = ClosureGroup___127 {
            actual___1380: actual___1380.clone()
        };
        let fn__5668 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5668())
        };
        test___113.assert(t___5675, fn__5668.clone());
        let mut t___5677: SqlBuilder = SqlBuilder::new();
        t___5677.append_safe("v = ");
        t___5677.append_float64(0.0f64);
        let actual___1383: std::sync::Arc<String> = t___5677.accumulated().to_string();
        let mut t___5683: bool = Some(actual___1383.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___128 {
            actual___1383: std::sync::Arc<String>
        }
        impl ClosureGroup___128 {
            fn fn__5667(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___1383.clone()));
            }
        }
        let closure_group = ClosureGroup___128 {
            actual___1383: actual___1383.clone()
        };
        let fn__5667 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5667())
        };
        test___113.assert(t___5683, fn__5667.clone());
        let mut t___5685: SqlBuilder = SqlBuilder::new();
        t___5685.append_safe("v = ");
        t___5685.append_float64(-42.5f64);
        let actual___1386: std::sync::Arc<String> = t___5685.accumulated().to_string();
        let mut t___5691: bool = Some(actual___1386.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___129 {
            actual___1386: std::sync::Arc<String>
        }
        impl ClosureGroup___129 {
            fn fn__5666(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___1386.clone()));
            }
        }
        let closure_group = ClosureGroup___129 {
            actual___1386: actual___1386.clone()
        };
        let fn__5666 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5666())
        };
        test___113.assert(t___5691, fn__5666.clone());
        test___113.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__1389() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___114 = temper_std::testing::Test::new();
        let mut t___2792: temper_std::temporal::Date;
        let d__1092: temper_std::temporal::Date;
        'ok___7301: {
            'orelse___1445: {
                t___2792 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___1445
                };
                d__1092 = t___2792.clone();
                break 'ok___7301;
            }
            d__1092 = panic!();
        }
        let mut t___5658: SqlBuilder = SqlBuilder::new();
        t___5658.append_safe("v = ");
        t___5658.append_date(d__1092.clone());
        let actual___1390: std::sync::Arc<String> = t___5658.accumulated().to_string();
        let mut t___5664: bool = Some(actual___1390.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___130 {
            actual___1390: std::sync::Arc<String>
        }
        impl ClosureGroup___130 {
            fn fn__5657(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___1390.clone()));
            }
        }
        let closure_group = ClosureGroup___130 {
            actual___1390: actual___1390.clone()
        };
        let fn__5657 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5657())
        };
        test___114.assert(t___5664, fn__5657.clone());
        test___114.soft_fail_to_hard()
    }
    #[test]
    fn nesting__1393() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___115 = temper_std::testing::Test::new();
        let name__1094: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___5626: SqlBuilder = SqlBuilder::new();
        t___5626.append_safe("where p.last_name = ");
        t___5626.append_string("Someone");
        let condition__1095: SqlFragment = t___5626.accumulated();
        let mut t___5630: SqlBuilder = SqlBuilder::new();
        t___5630.append_safe("select p.id from person p ");
        t___5630.append_fragment(condition__1095.clone());
        let actual___1395: std::sync::Arc<String> = t___5630.accumulated().to_string();
        let mut t___5636: bool = Some(actual___1395.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___131 {
            actual___1395: std::sync::Arc<String>
        }
        impl ClosureGroup___131 {
            fn fn__5625(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1395.clone()));
            }
        }
        let closure_group = ClosureGroup___131 {
            actual___1395: actual___1395.clone()
        };
        let fn__5625 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5625())
        };
        test___115.assert(t___5636, fn__5625.clone());
        let mut t___5638: SqlBuilder = SqlBuilder::new();
        t___5638.append_safe("select p.id from person p ");
        t___5638.append_part(SqlPart::new(condition__1095.to_source()));
        let actual___1398: std::sync::Arc<String> = t___5638.accumulated().to_string();
        let mut t___5645: bool = Some(actual___1398.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___132 {
            actual___1398: std::sync::Arc<String>
        }
        impl ClosureGroup___132 {
            fn fn__5624(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1398.clone()));
            }
        }
        let closure_group = ClosureGroup___132 {
            actual___1398: actual___1398.clone()
        };
        let fn__5624 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5624())
        };
        test___115.assert(t___5645, fn__5624.clone());
        let parts__1096: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___5649: SqlBuilder = SqlBuilder::new();
        t___5649.append_safe("select ");
        t___5649.append_part_list(parts__1096.clone());
        let actual___1401: std::sync::Arc<String> = t___5649.accumulated().to_string();
        let mut t___5655: bool = Some(actual___1401.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___133 {
            actual___1401: std::sync::Arc<String>
        }
        impl ClosureGroup___133 {
            fn fn__5623(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___1401.clone()));
            }
        }
        let closure_group = ClosureGroup___133 {
            actual___1401: actual___1401.clone()
        };
        let fn__5623 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5623())
        };
        test___115.assert(t___5655, fn__5623.clone());
        test___115.soft_fail_to_hard()
    }
    use super::*;
}
