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
    pub fn new(field__352: impl temper_core::ToArcString, message__353: impl temper_core::ToArcString) -> ChangesetError {
        let field__352 = field__352.to_arc_string();
        let message__353 = message__353.to_arc_string();
        let field;
        let message;
        field = field__352.clone();
        message = message__353.clone();
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
    fn cast(& self, allowedFields__363: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__366: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__369: SafeIdentifier, min__370: i32, max__371: i32) -> Changeset;
    fn validate_int(& self, field__374: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__377: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__380: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__383: SafeIdentifier) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__388: i32) -> temper_core::Result<SqlFragment>;
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
    pub fn cast(& self, allowedFields__404: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__404 = allowedFields__404.to_list();
        let mb__406: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__113: ChangesetImpl, mb__406: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__5700(& self, f__407: SafeIdentifier) {
                let mut t___5698: std::sync::Arc<String>;
                let mut t___5695: std::sync::Arc<String> = f__407.sql_value();
                let val__408: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__113.0.params, t___5695.clone(), std::sync::Arc::new("".to_string()));
                if ! val__408.is_empty() {
                    t___5698 = f__407.sql_value();
                    temper_core::MapBuilder::set( & self.mb__406, t___5698.clone(), val__408.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__113: self.clone(), mb__406: mb__406.clone()
        };
        let fn__5700 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__407: SafeIdentifier | closure_group.fn__5700(f__407))
        };
        temper_core::listed::list_for_each( & allowedFields__404, & ( * fn__5700.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__406), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__410: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__410 = fields__410.to_list();
        let return__210: Changeset;
        let mut t___5693: temper_core::List<ChangesetError>;
        let mut t___3382: TableDef;
        let mut t___3383: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___3384: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__411: {
            if ! self.0.is_valid {
                return__210 = Changeset::new(self.clone());
                break 'fn__411;
            }
            let eb__412: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__413: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__114: ChangesetImpl, eb__412: temper_core::ListBuilder<ChangesetError>, valid__413: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__5689(& self, f__414: SafeIdentifier) {
                    let mut t___5687: ChangesetError;
                    let mut t___5684: std::sync::Arc<String> = f__414.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__114.0.changes, t___5684.clone()) {
                        t___5687 = ChangesetError::new(f__414.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__412, t___5687.clone(), None);
                        {
                            * self.valid__413.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__114: self.clone(), eb__412: eb__412.clone(), valid__413: valid__413.clone()
            };
            let fn__5689 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__414: SafeIdentifier | closure_group.fn__5689(f__414))
            };
            temper_core::listed::list_for_each( & fields__410, & ( * fn__5689.clone()));
            t___3382 = self.0.table_def.clone();
            t___3383 = self.0.params.clone();
            t___3384 = self.0.changes.clone();
            t___5693 = temper_core::ListedTrait::to_list( & eb__412);
            return__210 = Changeset::new(ChangesetImpl::new(t___3382.clone(), t___3383.clone(), t___3384.clone(), t___5693.clone(), temper_core::read_locked( & valid__413)));
        }
        return return__210.clone();
    }
    pub fn validate_length(& self, field__416: SafeIdentifier, min__417: i32, max__418: i32) -> Changeset {
        let return__211: Changeset;
        let mut t___5671: std::sync::Arc<String>;
        let mut t___5682: temper_core::List<ChangesetError>;
        let mut t___3365: bool;
        let mut t___3371: TableDef;
        let mut t___3372: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___3373: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__419: {
            if ! self.0.is_valid {
                return__211 = Changeset::new(self.clone());
                break 'fn__419;
            }
            t___5671 = field__416.sql_value();
            let val__420: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___5671.clone(), std::sync::Arc::new("".to_string()));
            let len__421: i32 = temper_core::string::count_between( & val__420, 0usize, val__420.len());
            if Some(len__421) < Some(min__417) {
                t___3365 = true;
            } else {
                t___3365 = Some(len__421) > Some(max__418);
            }
            if t___3365 {
                let msg__422: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__417, max__418));
                let eb__423: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__423, ChangesetError::new(field__416.sql_value(), msg__422.clone()), None);
                t___3371 = self.0.table_def.clone();
                t___3372 = self.0.params.clone();
                t___3373 = self.0.changes.clone();
                t___5682 = temper_core::ListedTrait::to_list( & eb__423);
                return__211 = Changeset::new(ChangesetImpl::new(t___3371.clone(), t___3372.clone(), t___3373.clone(), t___5682.clone(), false));
                break 'fn__419;
            }
            return__211 = Changeset::new(self.clone());
        }
        return return__211.clone();
    }
    pub fn validate_int(& self, field__425: SafeIdentifier) -> Changeset {
        let return__212: Changeset;
        let mut t___5662: std::sync::Arc<String>;
        let mut t___5669: temper_core::List<ChangesetError>;
        let mut t___3356: TableDef;
        let mut t___3357: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___3358: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__426: {
            if ! self.0.is_valid {
                return__212 = Changeset::new(self.clone());
                break 'fn__426;
            }
            t___5662 = field__425.sql_value();
            let val__427: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___5662.clone(), std::sync::Arc::new("".to_string()));
            if val__427.is_empty() {
                return__212 = Changeset::new(self.clone());
                break 'fn__426;
            }
            let parseOk__428: bool;
            'ok___5805: {
                'orelse___1212: {
                    match temper_core::string::to_int( & val__427, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1212
                    };
                    parseOk__428 = true;
                    break 'ok___5805;
                }
                parseOk__428 = false;
            }
            if ! parseOk__428 {
                let eb__429: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__429, ChangesetError::new(field__425.sql_value(), "must be an integer"), None);
                t___3356 = self.0.table_def.clone();
                t___3357 = self.0.params.clone();
                t___3358 = self.0.changes.clone();
                t___5669 = temper_core::ListedTrait::to_list( & eb__429);
                return__212 = Changeset::new(ChangesetImpl::new(t___3356.clone(), t___3357.clone(), t___3358.clone(), t___5669.clone(), false));
                break 'fn__426;
            }
            return__212 = Changeset::new(self.clone());
        }
        return return__212.clone();
    }
    pub fn validate_int64(& self, field__431: SafeIdentifier) -> Changeset {
        let return__213: Changeset;
        let mut t___5653: std::sync::Arc<String>;
        let mut t___5660: temper_core::List<ChangesetError>;
        let mut t___3343: TableDef;
        let mut t___3344: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___3345: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__432: {
            if ! self.0.is_valid {
                return__213 = Changeset::new(self.clone());
                break 'fn__432;
            }
            t___5653 = field__431.sql_value();
            let val__433: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___5653.clone(), std::sync::Arc::new("".to_string()));
            if val__433.is_empty() {
                return__213 = Changeset::new(self.clone());
                break 'fn__432;
            }
            let parseOk__434: bool;
            'ok___5807: {
                'orelse___1213: {
                    match temper_core::string::to_int64( & val__433, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1213
                    };
                    parseOk__434 = true;
                    break 'ok___5807;
                }
                parseOk__434 = false;
            }
            if ! parseOk__434 {
                let eb__435: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__435, ChangesetError::new(field__431.sql_value(), "must be a 64-bit integer"), None);
                t___3343 = self.0.table_def.clone();
                t___3344 = self.0.params.clone();
                t___3345 = self.0.changes.clone();
                t___5660 = temper_core::ListedTrait::to_list( & eb__435);
                return__213 = Changeset::new(ChangesetImpl::new(t___3343.clone(), t___3344.clone(), t___3345.clone(), t___5660.clone(), false));
                break 'fn__432;
            }
            return__213 = Changeset::new(self.clone());
        }
        return return__213.clone();
    }
    pub fn validate_float(& self, field__437: SafeIdentifier) -> Changeset {
        let return__214: Changeset;
        let mut t___5644: std::sync::Arc<String>;
        let mut t___5651: temper_core::List<ChangesetError>;
        let mut t___3330: TableDef;
        let mut t___3331: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___3332: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__438: {
            if ! self.0.is_valid {
                return__214 = Changeset::new(self.clone());
                break 'fn__438;
            }
            t___5644 = field__437.sql_value();
            let val__439: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___5644.clone(), std::sync::Arc::new("".to_string()));
            if val__439.is_empty() {
                return__214 = Changeset::new(self.clone());
                break 'fn__438;
            }
            let parseOk__440: bool;
            'ok___5809: {
                'orelse___1214: {
                    match temper_core::string::to_float64( & val__439) {
                        Ok(x) => x,
                        _ => break 'orelse___1214
                    };
                    parseOk__440 = true;
                    break 'ok___5809;
                }
                parseOk__440 = false;
            }
            if ! parseOk__440 {
                let eb__441: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__441, ChangesetError::new(field__437.sql_value(), "must be a number"), None);
                t___3330 = self.0.table_def.clone();
                t___3331 = self.0.params.clone();
                t___3332 = self.0.changes.clone();
                t___5651 = temper_core::ListedTrait::to_list( & eb__441);
                return__214 = Changeset::new(ChangesetImpl::new(t___3330.clone(), t___3331.clone(), t___3332.clone(), t___5651.clone(), false));
                break 'fn__438;
            }
            return__214 = Changeset::new(self.clone());
        }
        return return__214.clone();
    }
    pub fn validate_bool(& self, field__443: SafeIdentifier) -> Changeset {
        let return__215: Changeset;
        let mut t___5635: std::sync::Arc<String>;
        let mut t___5642: temper_core::List<ChangesetError>;
        let mut t___3305: bool;
        let mut t___3306: bool;
        let mut t___3308: bool;
        let mut t___3309: bool;
        let mut t___3311: bool;
        let mut t___3317: TableDef;
        let mut t___3318: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___3319: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__444: {
            if ! self.0.is_valid {
                return__215 = Changeset::new(self.clone());
                break 'fn__444;
            }
            t___5635 = field__443.sql_value();
            let val__445: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___5635.clone(), std::sync::Arc::new("".to_string()));
            if val__445.is_empty() {
                return__215 = Changeset::new(self.clone());
                break 'fn__444;
            }
            let isTrue__446: bool;
            if Some(val__445.as_str()) == Some("true") {
                isTrue__446 = true;
            } else {
                if Some(val__445.as_str()) == Some("1") {
                    t___3306 = true;
                } else {
                    if Some(val__445.as_str()) == Some("yes") {
                        t___3305 = true;
                    } else {
                        t___3305 = Some(val__445.as_str()) == Some("on");
                    }
                    t___3306 = t___3305;
                }
                isTrue__446 = t___3306;
            }
            let isFalse__447: bool;
            if Some(val__445.as_str()) == Some("false") {
                isFalse__447 = true;
            } else {
                if Some(val__445.as_str()) == Some("0") {
                    t___3309 = true;
                } else {
                    if Some(val__445.as_str()) == Some("no") {
                        t___3308 = true;
                    } else {
                        t___3308 = Some(val__445.as_str()) == Some("off");
                    }
                    t___3309 = t___3308;
                }
                isFalse__447 = t___3309;
            }
            if ! isTrue__446 {
                t___3311 = ! isFalse__447;
            } else {
                t___3311 = false;
            }
            if t___3311 {
                let eb__448: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__448, ChangesetError::new(field__443.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___3317 = self.0.table_def.clone();
                t___3318 = self.0.params.clone();
                t___3319 = self.0.changes.clone();
                t___5642 = temper_core::ListedTrait::to_list( & eb__448);
                return__215 = Changeset::new(ChangesetImpl::new(t___3317.clone(), t___3318.clone(), t___3319.clone(), t___5642.clone(), false));
                break 'fn__444;
            }
            return__215 = Changeset::new(self.clone());
        }
        return return__215.clone();
    }
    fn parse_bool_sql_part(& self, val__450: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__450 = val__450.to_arc_string();
        let return__216: SqlBoolean;
        let mut t___3294: bool;
        let mut t___3295: bool;
        let mut t___3296: bool;
        let mut t___3298: bool;
        let mut t___3299: bool;
        let mut t___3300: bool;
        'fn__451: {
            if Some(val__450.as_str()) == Some("true") {
                t___3296 = true;
            } else {
                if Some(val__450.as_str()) == Some("1") {
                    t___3295 = true;
                } else {
                    if Some(val__450.as_str()) == Some("yes") {
                        t___3294 = true;
                    } else {
                        t___3294 = Some(val__450.as_str()) == Some("on");
                    }
                    t___3295 = t___3294;
                }
                t___3296 = t___3295;
            }
            if t___3296 {
                return__216 = SqlBoolean::new(true);
                break 'fn__451;
            }
            if Some(val__450.as_str()) == Some("false") {
                t___3300 = true;
            } else {
                if Some(val__450.as_str()) == Some("0") {
                    t___3299 = true;
                } else {
                    if Some(val__450.as_str()) == Some("no") {
                        t___3298 = true;
                    } else {
                        t___3298 = Some(val__450.as_str()) == Some("off");
                    }
                    t___3299 = t___3298;
                }
                t___3300 = t___3299;
            }
            if t___3300 {
                return__216 = SqlBoolean::new(false);
                break 'fn__451;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__216.clone());
    }
    fn value_to_sql_part(& self, fieldDef__453: FieldDef, val__454: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__454 = val__454.to_arc_string();
        let return__217: SqlPart;
        let mut t___3281: i32;
        let mut t___3284: i64;
        let mut t___3287: f64;
        let mut t___3292: temper_std::temporal::Date;
        'fn__455: {
            let ft__456: FieldType = fieldDef__453.field_type();
            if temper_core::is::<StringField>(ft__456.clone()) {
                return__217 = SqlPart::new(SqlString::new(val__454.clone()));
                break 'fn__455;
            }
            if temper_core::is::<IntField>(ft__456.clone()) {
                t___3281 = temper_core::string::to_int( & val__454, None) ? ;
                return__217 = SqlPart::new(SqlInt32::new(t___3281));
                break 'fn__455;
            }
            if temper_core::is::<Int64Field>(ft__456.clone()) {
                t___3284 = temper_core::string::to_int64( & val__454, None) ? ;
                return__217 = SqlPart::new(SqlInt64::new(t___3284));
                break 'fn__455;
            }
            if temper_core::is::<FloatField>(ft__456.clone()) {
                t___3287 = temper_core::string::to_float64( & val__454) ? ;
                return__217 = SqlPart::new(SqlFloat64::new(t___3287));
                break 'fn__455;
            }
            if temper_core::is::<BoolField>(ft__456.clone()) {
                return__217 = SqlPart::new(self.parse_bool_sql_part(val__454.clone()) ? );
                break 'fn__455;
            }
            if temper_core::is::<DateField>(ft__456.clone()) {
                t___3292 = temper_std::temporal::Date::from_iso_string(val__454.clone()) ? ;
                return__217 = SqlPart::new(SqlDate::new(t___3292.clone()));
                break 'fn__455;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__217.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___5583: i32;
        let mut t___5588: std::sync::Arc<String>;
        let mut t___5589: bool;
        let mut t___5594: i32;
        let mut t___5596: std::sync::Arc<String>;
        let mut t___5600: std::sync::Arc<String>;
        let mut t___5615: i32;
        let mut t___3245: bool;
        let mut t___3253: FieldDef;
        let mut t___3258: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__459: i32 = 0;
        'loop___5870: loop {
            t___5583 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__459) < Some(t___5583)) {
                break;
            }
            let f__460: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__459);
            if ! f__460.nullable() {
                t___5588 = f__460.name().sql_value();
                t___5589 = temper_core::MappedTrait::has( & self.0.changes, t___5588.clone());
                t___3245 = ! t___5589;
            } else {
                t___3245 = false;
            }
            if t___3245 {
                return Err(temper_core::Error::new());
            }
            i__459 = i__459.wrapping_add(1);
        }
        let pairs__461: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__461)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__462: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__463: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__464: i32 = 0;
        'loop___5871: loop {
            t___5594 = temper_core::ListedTrait::len( & pairs__461);
            if ! (Some(i__464) < Some(t___5594)) {
                break;
            }
            let pair__465: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__461, i__464);
            t___5596 = pair__465.key();
            t___3253 = self.0.table_def.field(t___5596.clone()) ? ;
            let fd__466: FieldDef = t___3253.clone();
            temper_core::listed::add( & colNames__462, fd__466.name().sql_value(), None);
            t___5600 = pair__465.value();
            t___3258 = self.value_to_sql_part(fd__466.clone(), t___5600.clone()) ? ;
            temper_core::listed::add( & valParts__463, t___3258.clone(), None);
            i__464 = i__464.wrapping_add(1);
        }
        let b__467: SqlBuilder = SqlBuilder::new();
        b__467.append_safe("INSERT INTO ");
        b__467.append_safe(self.0.table_def.table_name().sql_value());
        b__467.append_safe(" (");
        let mut t___5608: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__462);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__5581(& self, c__468: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__468 = c__468.to_arc_string();
                return c__468.clone();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__5581 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__468: std::sync::Arc<String> | closure_group.fn__5581(c__468))
        };
        b__467.append_safe(temper_core::listed::join( & t___5608, std::sync::Arc::new(", ".to_string()), & ( * fn__5581.clone())));
        b__467.append_safe(") VALUES (");
        b__467.append_part(temper_core::ListedTrait::get( & valParts__463, 0));
        let mut j__469: i32 = 1;
        'loop___5872: loop {
            t___5615 = temper_core::ListedTrait::len( & valParts__463);
            if ! (Some(j__469) < Some(t___5615)) {
                break;
            }
            b__467.append_safe(", ");
            b__467.append_part(temper_core::ListedTrait::get( & valParts__463, j__469));
            j__469 = j__469.wrapping_add(1);
        }
        b__467.append_safe(")");
        return Ok(b__467.accumulated());
    }
    pub fn to_update_sql(& self, id__471: i32) -> temper_core::Result<SqlFragment> {
        let mut t___5568: i32;
        let mut t___5571: std::sync::Arc<String>;
        let mut t___5576: std::sync::Arc<String>;
        let mut t___3226: FieldDef;
        let mut t___3232: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__473: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__473)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__474: SqlBuilder = SqlBuilder::new();
        b__474.append_safe("UPDATE ");
        b__474.append_safe(self.0.table_def.table_name().sql_value());
        b__474.append_safe(" SET ");
        let mut i__475: i32 = 0;
        'loop___5873: loop {
            t___5568 = temper_core::ListedTrait::len( & pairs__473);
            if ! (Some(i__475) < Some(t___5568)) {
                break;
            }
            if Some(i__475) > Some(0) {
                b__474.append_safe(", ");
            }
            let pair__476: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__473, i__475);
            t___5571 = pair__476.key();
            t___3226 = self.0.table_def.field(t___5571.clone()) ? ;
            let fd__477: FieldDef = t___3226.clone();
            b__474.append_safe(fd__477.name().sql_value());
            b__474.append_safe(" = ");
            t___5576 = pair__476.value();
            t___3232 = self.value_to_sql_part(fd__477.clone(), t___5576.clone()) ? ;
            b__474.append_part(t___3232.clone());
            i__475 = i__475.wrapping_add(1);
        }
        b__474.append_safe(" WHERE id = ");
        b__474.append_int32(id__471);
        return Ok(b__474.accumulated());
    }
    pub fn new(_tableDef__479: TableDef, _params__480: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__481: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__482: impl temper_core::ToList<ChangesetError>, _isValid__483: bool) -> ChangesetImpl {
        let _errors__482 = _errors__482.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__479.clone();
        params = _params__480.clone();
        changes = _changes__481.clone();
        errors = _errors__482.clone();
        is_valid = _isValid__483;
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
    fn cast(& self, allowedFields__404: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__404)
    }
    fn validate_required(& self, fields__410: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__410)
    }
    fn validate_length(& self, field__416: SafeIdentifier, min__417: i32, max__418: i32) -> Changeset {
        self.validate_length(field__416, min__417, max__418)
    }
    fn validate_int(& self, field__425: SafeIdentifier) -> Changeset {
        self.validate_int(field__425)
    }
    fn validate_int64(& self, field__431: SafeIdentifier) -> Changeset {
        self.validate_int64(field__431)
    }
    fn validate_float(& self, field__437: SafeIdentifier) -> Changeset {
        self.validate_float(field__437)
    }
    fn validate_bool(& self, field__443: SafeIdentifier) -> Changeset {
        self.validate_bool(field__443)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__471: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__471)
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
    pub fn new(joinType__596: JoinType, table__597: SafeIdentifier, onCondition__598: SqlFragment) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__596.clone();
        table = table__597.clone();
        on_condition = onCondition__598.clone();
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
    pub fn new(field__602: SafeIdentifier, ascending__603: bool) -> OrderClause {
        let field;
        let ascending;
        field = field__602.clone();
        ascending = ascending__603;
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
struct QueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<SqlFragment>, selected_fields: temper_core::List<SafeIdentifier>, order_clauses: temper_core::List<OrderClause>, limit_val: Option<i32>, offset_val: Option<i32>, join_clauses: temper_core::List<JoinClause>
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<QueryStruct>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<SqlFragment>, pub selected_fields: temper_core::List<SafeIdentifier>, pub order_clauses: temper_core::List<OrderClause>, pub limit_val: Option<i32>, pub offset_val: Option<i32>, pub join_clauses: temper_core::List<JoinClause>
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.table_name, self.conditions, self.selected_fields, self.order_clauses, self.limit_val, self.offset_val, self.join_clauses)
    }
}
impl Query {
    pub fn r#where(& self, condition__612: SqlFragment) -> Query {
        let nb__614: temper_core::ListBuilder<SqlFragment> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__614, condition__612.clone(), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__614), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone());
    }
    pub fn select(& self, fields__616: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__616 = fields__616.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__616.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone());
    }
    pub fn order_by(& self, field__619: SafeIdentifier, ascending__620: bool) -> Query {
        let nb__622: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__622, OrderClause::new(field__619.clone(), ascending__620), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__622), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone());
    }
    pub fn limit(& self, n__624: i32) -> temper_core::Result<Query> {
        if Some(n__624) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__624), self.0.offset_val, self.0.join_clauses.clone()));
    }
    pub fn offset(& self, n__627: i32) -> temper_core::Result<Query> {
        if Some(n__627) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__627), self.0.join_clauses.clone()));
    }
    pub fn join(& self, joinType__630: JoinType, table__631: SafeIdentifier, onCondition__632: SqlFragment) -> Query {
        let nb__634: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__634, JoinClause::new(joinType__630.clone(), table__631.clone(), onCondition__632.clone()), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__634));
    }
    pub fn inner_join(& self, table__636: SafeIdentifier, onCondition__637: SqlFragment) -> Query {
        let mut t___5152: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___5152.clone()), table__636.clone(), onCondition__637.clone());
    }
    pub fn left_join(& self, table__640: SafeIdentifier, onCondition__641: SqlFragment) -> Query {
        let mut t___5150: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___5150.clone()), table__640.clone(), onCondition__641.clone());
    }
    pub fn right_join(& self, table__644: SafeIdentifier, onCondition__645: SqlFragment) -> Query {
        let mut t___5148: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___5148.clone()), table__644.clone(), onCondition__645.clone());
    }
    pub fn full_join(& self, table__648: SafeIdentifier, onCondition__649: SqlFragment) -> Query {
        let mut t___5146: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___5146.clone()), table__648.clone(), onCondition__649.clone());
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___5133: i32;
        let b__653: SqlBuilder = SqlBuilder::new();
        b__653.append_safe("SELECT ");
        if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
            b__653.append_safe("*");
        } else {
            #[derive(Clone)]
            struct ClosureGroup___4 {}
            impl ClosureGroup___4 {
                fn fn__5116(& self, f__654: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__654.sql_value();
                }
            }
            let closure_group = ClosureGroup___4 {};
            let fn__5116 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__654: SafeIdentifier | closure_group.fn__5116(f__654))
            };
            b__653.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__5116.clone())));
        }
        b__653.append_safe(" FROM ");
        b__653.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___5 {
            b__653: SqlBuilder
        }
        impl ClosureGroup___5 {
            fn fn__5115(& self, jc__655: JoinClause) {
                self.b__653.append_safe(" ");
                let mut t___5104: std::sync::Arc<String> = jc__655.join_type().keyword();
                self.b__653.append_safe(t___5104.clone());
                self.b__653.append_safe(" ");
                let mut t___5108: std::sync::Arc<String> = jc__655.table().sql_value();
                self.b__653.append_safe(t___5108.clone());
                self.b__653.append_safe(" ON ");
                let mut t___5111: SqlFragment = jc__655.on_condition();
                self.b__653.append_fragment(t___5111.clone());
            }
        }
        let closure_group = ClosureGroup___5 {
            b__653: b__653.clone()
        };
        let fn__5115 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__655: JoinClause | closure_group.fn__5115(jc__655))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__5115.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__653.append_safe(" WHERE ");
            b__653.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0));
            let mut i__656: i32 = 1;
            'loop___5884: loop {
                t___5133 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__656) < Some(t___5133)) {
                    break;
                }
                b__653.append_safe(" AND ");
                b__653.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__656));
                i__656 = i__656.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__653.append_safe(" ORDER BY ");
            let mut first__657: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___6 {
                first__657: std::sync::Arc<std::sync::RwLock<bool>>, b__653: SqlBuilder
            }
            impl ClosureGroup___6 {
                fn fn__5114(& self, oc__658: OrderClause) {
                    let mut t___2812: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__657) {
                        self.b__653.append_safe(", ");
                    }
                    {
                        * self.first__657.write().unwrap() = false;
                    }
                    let mut t___5098: std::sync::Arc<String> = oc__658.field().sql_value();
                    self.b__653.append_safe(t___5098.clone());
                    if oc__658.ascending() {
                        t___2812 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___2812 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__653.append_safe(t___2812.clone());
                }
            }
            let closure_group = ClosureGroup___6 {
                first__657: first__657.clone(), b__653: b__653.clone()
            };
            let fn__5114 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | oc__658: OrderClause | closure_group.fn__5114(oc__658))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__5114.clone()));
        }
        let lv__659: Option<i32> = self.0.limit_val;
        if ! lv__659.is_none() {
            let lv___1257: i32 = lv__659.unwrap();
            b__653.append_safe(" LIMIT ");
            b__653.append_int32(lv___1257);
        }
        let ov__660: Option<i32> = self.0.offset_val;
        if ! ov__660.is_none() {
            let ov___1258: i32 = ov__660.unwrap();
            b__653.append_safe(" OFFSET ");
            b__653.append_int32(ov___1258);
        }
        return b__653.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__662: i32) -> temper_core::Result<SqlFragment> {
        let return__260: SqlFragment;
        let mut t___2805: Query;
        if Some(defaultLimit__662) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__260 = self.to_sql();
        } else {
            t___2805 = self.limit(defaultLimit__662) ? ;
            return__260 = t___2805.to_sql();
        }
        return Ok(return__260.clone());
    }
    pub fn new(tableName__665: SafeIdentifier, conditions__666: impl temper_core::ToList<SqlFragment>, selectedFields__667: impl temper_core::ToList<SafeIdentifier>, orderClauses__668: impl temper_core::ToList<OrderClause>, limitVal__669: Option<i32>, offsetVal__670: Option<i32>, joinClauses__671: impl temper_core::ToList<JoinClause>) -> Query {
        let conditions__666 = conditions__666.to_list();
        let selectedFields__667 = selectedFields__667.to_list();
        let orderClauses__668 = orderClauses__668.to_list();
        let joinClauses__671 = joinClauses__671.to_list();
        let table_name;
        let conditions;
        let selected_fields;
        let order_clauses;
        let limit_val;
        let offset_val;
        let join_clauses;
        table_name = tableName__665.clone();
        conditions = conditions__666.clone();
        selected_fields = selectedFields__667.clone();
        order_clauses = orderClauses__668.clone();
        limit_val = limitVal__669;
        offset_val = offsetVal__670;
        join_clauses = joinClauses__671.clone();
        let selfish = Query(std::sync::Arc::new(QueryStruct {
                    table_name, conditions, selected_fields, order_clauses, limit_val, offset_val, join_clauses
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<SqlFragment> {
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
    pub fn new(_value__742: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__742 = _value__742.to_arc_string();
        let value;
        value = _value__742.clone();
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
    pub fn new(name__760: SafeIdentifier, fieldType__761: FieldType, nullable__762: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__760.clone();
        field_type = fieldType__761.clone();
        nullable = nullable__762;
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
    pub fn field(& self, name__766: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__766 = name__766.to_arc_string();
        let return__290: FieldDef;
        'fn__767: {
            let this__3570: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__3571: i32 = temper_core::ListedTrait::len( & this__3570);
            let mut i__3572: i32 = 0;
            'loop___5891: while Some(i__3572) < Some(n__3571) {
                let el__3573: FieldDef = temper_core::ListedTrait::get( & this__3570, i__3572);
                i__3572 = i__3572.wrapping_add(1);
                let f__768: FieldDef = el__3573.clone();
                if Some(f__768.name().sql_value().as_str()) == Some(name__766.as_str()) {
                    return__290 = f__768.clone();
                    break 'fn__767;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__290.clone());
    }
    pub fn new(tableName__770: SafeIdentifier, fields__771: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__771 = fields__771.to_list();
        let table_name;
        let fields;
        table_name = tableName__770.clone();
        fields = fields__771.clone();
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
    pub fn append_safe(& self, sqlSource__793: impl temper_core::ToArcString) {
        let sqlSource__793 = sqlSource__793.to_arc_string();
        let mut t___5758: SqlSource = SqlSource::new(sqlSource__793.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5758.clone()), None);
    }
    pub fn append_fragment(& self, fragment__796: SqlFragment) {
        let mut t___5756: temper_core::List<SqlPart> = fragment__796.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___5756.clone()), None);
    }
    pub fn append_part(& self, part__799: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__799.clone(), None);
    }
    pub fn append_part_list(& self, values__802: impl temper_core::ToList<SqlPart>) {
        let values__802 = values__802.to_list();
        #[derive(Clone)]
        struct ClosureGroup___7 {
            this__148: SqlBuilder
        }
        impl ClosureGroup___7 {
            fn fn__5752(& self, x__804: SqlPart) {
                self.this__148.append_part(x__804.clone());
            }
        }
        let closure_group = ClosureGroup___7 {
            this__148: self.clone()
        };
        let fn__5752 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__804: SqlPart | closure_group.fn__5752(x__804))
        };
        self.append_list(temper_core::ToListed::to_listed(values__802.clone()), fn__5752.clone());
    }
    pub fn append_boolean(& self, value__806: bool) {
        let mut t___5749: SqlBoolean = SqlBoolean::new(value__806);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5749.clone()), None);
    }
    pub fn append_boolean_list(& self, values__809: impl temper_core::ToListed<bool>) {
        let values__809 = values__809.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___8 {
            this__150: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__5746(& self, x__811: bool) {
                self.this__150.append_boolean(x__811);
            }
        }
        let closure_group = ClosureGroup___8 {
            this__150: self.clone()
        };
        let fn__5746 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__811: bool | closure_group.fn__5746(x__811))
        };
        self.append_list(values__809.clone(), fn__5746.clone());
    }
    pub fn append_date(& self, value__813: temper_std::temporal::Date) {
        let mut t___5743: SqlDate = SqlDate::new(value__813.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5743.clone()), None);
    }
    pub fn append_date_list(& self, values__816: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__816 = values__816.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___9 {
            this__152: SqlBuilder
        }
        impl ClosureGroup___9 {
            fn fn__5740(& self, x__818: temper_std::temporal::Date) {
                self.this__152.append_date(x__818.clone());
            }
        }
        let closure_group = ClosureGroup___9 {
            this__152: self.clone()
        };
        let fn__5740 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__818: temper_std::temporal::Date | closure_group.fn__5740(x__818))
        };
        self.append_list(values__816.clone(), fn__5740.clone());
    }
    pub fn append_float64(& self, value__820: f64) {
        let mut t___5737: SqlFloat64 = SqlFloat64::new(value__820);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5737.clone()), None);
    }
    pub fn append_float64_list(& self, values__823: impl temper_core::ToListed<f64>) {
        let values__823 = values__823.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__154: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__5734(& self, x__825: f64) {
                self.this__154.append_float64(x__825);
            }
        }
        let closure_group = ClosureGroup___10 {
            this__154: self.clone()
        };
        let fn__5734 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__825: f64 | closure_group.fn__5734(x__825))
        };
        self.append_list(values__823.clone(), fn__5734.clone());
    }
    pub fn append_int32(& self, value__827: i32) {
        let mut t___5731: SqlInt32 = SqlInt32::new(value__827);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5731.clone()), None);
    }
    pub fn append_int32_list(& self, values__830: impl temper_core::ToListed<i32>) {
        let values__830 = values__830.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__156: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__5728(& self, x__832: i32) {
                self.this__156.append_int32(x__832);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__156: self.clone()
        };
        let fn__5728 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__832: i32 | closure_group.fn__5728(x__832))
        };
        self.append_list(values__830.clone(), fn__5728.clone());
    }
    pub fn append_int64(& self, value__834: i64) {
        let mut t___5725: SqlInt64 = SqlInt64::new(value__834);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5725.clone()), None);
    }
    pub fn append_int64_list(& self, values__837: impl temper_core::ToListed<i64>) {
        let values__837 = values__837.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__158: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__5722(& self, x__839: i64) {
                self.this__158.append_int64(x__839);
            }
        }
        let closure_group = ClosureGroup___12 {
            this__158: self.clone()
        };
        let fn__5722 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__839: i64 | closure_group.fn__5722(x__839))
        };
        self.append_list(values__837.clone(), fn__5722.clone());
    }
    pub fn append_string(& self, value__841: impl temper_core::ToArcString) {
        let value__841 = value__841.to_arc_string();
        let mut t___5719: SqlString = SqlString::new(value__841.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5719.clone()), None);
    }
    pub fn append_string_list(& self, values__844: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__844 = values__844.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__160: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__5716(& self, x__846: impl temper_core::ToArcString) {
                let x__846 = x__846.to_arc_string();
                self.this__160.append_string(x__846.clone());
            }
        }
        let closure_group = ClosureGroup___13 {
            this__160: self.clone()
        };
        let fn__5716 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__846: std::sync::Arc<String> | closure_group.fn__5716(x__846))
        };
        self.append_list(values__844.clone(), fn__5716.clone());
    }
    fn append_list<T>(& self, values__848: impl temper_core::ToListed<T>, appendValue__849: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__848 = values__848.to_listed();
        let mut t___5711: i32;
        let mut t___5713: T;
        let mut i__851: i32 = 0;
        'loop___5892: loop {
            t___5711 = temper_core::ListedTrait::len( & ( * values__848));
            if ! (Some(i__851) < Some(t___5711)) {
                break;
            }
            if Some(i__851) > Some(0) {
                self.append_safe(", ");
            }
            t___5713 = temper_core::ListedTrait::get( & ( * values__848), i__851);
            appendValue__849(t___5713.clone());
            i__851 = i__851.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___5708: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___5708.clone();
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
        let mut t___5782: i32;
        let builder__863: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__864: i32 = 0;
        'loop___5893: loop {
            t___5782 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__864) < Some(t___5782)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__864).format_to(builder__863.clone());
            i__864 = i__864.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__863);
    }
    pub fn new(parts__866: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__866 = parts__866.to_list();
        let parts;
        parts = parts__866.clone();
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
    fn format_to(& self, builder__868: std::sync::Arc<std::sync::RwLock<String>>);
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
    pub fn format_to(& self, builder__872: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__872, self.0.source.clone());
    }
    pub fn new(source__875: impl temper_core::ToArcString) -> SqlSource {
        let source__875 = source__875.to_arc_string();
        let source;
        source = source__875.clone();
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
    fn format_to(& self, builder__872: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__872)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__878: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___3437: std::sync::Arc<String>;
        if self.0.value {
            t___3437 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___3437 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__878, t___3437.clone());
    }
    pub fn new(value__881: bool) -> SqlBoolean {
        let value;
        value = value__881;
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
    fn format_to(& self, builder__878: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__878)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__884: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__884, "'");
        let mut t___5763: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            builder__884: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___14 {
            fn fn__5761(& self, c__886: i32) {
                if Some(c__886) == Some(39) {
                    temper_core::string::builder::append( & self.builder__884, "''");
                } else {
                    'ok___5821: {
                        'orelse___1211: {
                            match temper_core::string::builder::append_code_point( & self.builder__884, c__886) {
                                Ok(x) => x,
                                _ => break 'orelse___1211
                            };
                            break 'ok___5821;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___14 {
            builder__884: builder__884.clone()
        };
        let fn__5761 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__886: i32 | closure_group.fn__5761(c__886))
        };
        temper_core::string::for_each( & t___5763, & ( * fn__5761.clone()));
        temper_core::string::builder::append( & builder__884, "'");
    }
    pub fn new(value__888: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__888.clone();
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
    fn format_to(& self, builder__884: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__884)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__891: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___3426: bool;
        let mut t___3427: bool;
        let s__893: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__893.as_str()) == Some("NaN") {
            t___3427 = true;
        } else {
            if Some(s__893.as_str()) == Some("Infinity") {
                t___3426 = true;
            } else {
                t___3426 = Some(s__893.as_str()) == Some("-Infinity");
            }
            t___3427 = t___3426;
        }
        if t___3427 {
            temper_core::string::builder::append( & builder__891, "NULL");
        } else {
            temper_core::string::builder::append( & builder__891, s__893.clone());
        }
    }
    pub fn new(value__895: f64) -> SqlFloat64 {
        let value;
        value = value__895;
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
    fn format_to(& self, builder__891: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__891)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__898: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___5772: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__898, t___5772.clone());
    }
    pub fn new(value__901: i32) -> SqlInt32 {
        let value;
        value = value__901;
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
    fn format_to(& self, builder__898: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__898)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__904: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___5770: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__904, t___5770.clone());
    }
    pub fn new(value__907: i64) -> SqlInt64 {
        let value;
        value = value__907;
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
    fn format_to(& self, builder__904: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__904)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__910: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__910, "'");
        #[derive(Clone)]
        struct ClosureGroup___15 {
            builder__910: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___15 {
            fn fn__5775(& self, c__912: i32) {
                if Some(c__912) == Some(39) {
                    temper_core::string::builder::append( & self.builder__910, "''");
                } else {
                    'ok___5826: {
                        'orelse___1210: {
                            match temper_core::string::builder::append_code_point( & self.builder__910, c__912) {
                                Ok(x) => x,
                                _ => break 'orelse___1210
                            };
                            break 'ok___5826;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___15 {
            builder__910: builder__910.clone()
        };
        let fn__5775 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__912: i32 | closure_group.fn__5775(c__912))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__5775.clone()));
        temper_core::string::builder::append( & builder__910, "'");
    }
    pub fn new(value__914: impl temper_core::ToArcString) -> SqlString {
        let value__914 = value__914.to_arc_string();
        let value;
        value = value__914.clone();
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
    fn format_to(& self, builder__910: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__910)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__484: TableDef, params__485: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___5558: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__484.clone(), params__485.clone(), t___5558.clone(), [], true));
}
fn isIdentStart__345(c__743: i32) -> bool {
    let return__270: bool;
    let mut t___3200: bool;
    let mut t___3201: bool;
    if Some(c__743) >= Some(97) {
        t___3200 = Some(c__743) <= Some(122);
    } else {
        t___3200 = false;
    }
    if t___3200 {
        return__270 = true;
    } else {
        if Some(c__743) >= Some(65) {
            t___3201 = Some(c__743) <= Some(90);
        } else {
            t___3201 = false;
        }
        if t___3201 {
            return__270 = true;
        } else {
            return__270 = Some(c__743) == Some(95);
        }
    }
    return return__270;
}
fn isIdentPart__346(c__745: i32) -> bool {
    let return__271: bool;
    if isIdentStart__345(c__745) {
        return__271 = true;
    } else {
        if Some(c__745) >= Some(48) {
            return__271 = Some(c__745) <= Some(57);
        } else {
            return__271 = false;
        }
    }
    return return__271;
}
pub fn safe_identifier(name__747: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__747 = name__747.to_arc_string();
    let mut t___5556: usize;
    if name__747.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__749: usize = 0usize;
    if ! isIdentStart__345(temper_core::string::get( & name__747, idx__749)) {
        return Err(temper_core::Error::new());
    }
    let mut t___5553: usize = temper_core::string::next( & name__747, idx__749);
    idx__749 = t___5553;
    'loop___5894: loop {
        if ! temper_core::string::has_index( & name__747, idx__749) {
            break;
        }
        if ! isIdentPart__346(temper_core::string::get( & name__747, idx__749)) {
            return Err(temper_core::Error::new());
        }
        t___5556 = temper_core::string::next( & name__747, idx__749);
        idx__749 = t___5556;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__747.clone())));
}
fn csid__342(name__487: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__487 = name__487.to_arc_string();
    let return__221: SafeIdentifier;
    let mut t___3188: SafeIdentifier;
    'ok___5831: {
        'orelse___1215: {
            t___3188 = match safe_identifier(name__487.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1215
            };
            return__221 = t___3188.clone();
            break 'ok___5831;
        }
        return__221 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__221.clone();
}
fn userTable__343() -> TableDef {
    return TableDef::new(csid__342("users"), [FieldDef::new(csid__342("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__342("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__342("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__342("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__342("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__574: TableDef, id__575: i32) -> SqlFragment {
    let b__577: SqlBuilder = SqlBuilder::new();
    b__577.append_safe("DELETE FROM ");
    b__577.append_safe(tableDef__574.table_name().sql_value());
    b__577.append_safe(" WHERE id = ");
    b__577.append_int32(id__575);
    return b__577.accumulated();
}
pub fn from(tableName__672: SafeIdentifier) -> Query {
    return Query::new(tableName__672.clone(), [], [], [], None, None, []);
}
pub fn col(table__674: SafeIdentifier, column__675: SafeIdentifier) -> SqlFragment {
    let b__677: SqlBuilder = SqlBuilder::new();
    b__677.append_safe(table__674.sql_value());
    b__677.append_safe(".");
    b__677.append_safe(column__675.sql_value());
    return b__677.accumulated();
}
fn sid__344(name__678: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__678 = name__678.to_arc_string();
    let return__263: SafeIdentifier;
    let mut t___2781: SafeIdentifier;
    'ok___5842: {
        'orelse___1223: {
            t___2781 = match safe_identifier(name__678.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1223
            };
            return__263 = t___2781.clone();
            break 'ok___5842;
        }
        return__263 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__263.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__1020() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___22 = temper_std::testing::Test::new();
        let params__491: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___5514: TableDef = userTable__343();
        let mut t___5515: SafeIdentifier = csid__342("name");
        let mut t___5516: SafeIdentifier = csid__342("email");
        let cs__492: Changeset = changeset(t___5514.clone(), params__491.clone()).cast(std::sync::Arc::new(vec![t___5515.clone(), t___5516.clone()]));
        let mut t___5519: bool = temper_core::MappedTrait::has( & cs__492.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___16 {}
        impl ClosureGroup___16 {
            fn fn__5509(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___16 {};
        let fn__5509 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5509())
        };
        test___22.assert(t___5519, fn__5509.clone());
        let mut t___5523: bool = temper_core::MappedTrait::has( & cs__492.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___17 {}
        impl ClosureGroup___17 {
            fn fn__5508(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___17 {};
        let fn__5508 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5508())
        };
        test___22.assert(t___5523, fn__5508.clone());
        let mut t___5529: bool = ! temper_core::MappedTrait::has( & cs__492.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___18 {}
        impl ClosureGroup___18 {
            fn fn__5507(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___18 {};
        let fn__5507 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5507())
        };
        test___22.assert(t___5529, fn__5507.clone());
        let mut t___5531: bool = cs__492.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__5506(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__5506 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5506())
        };
        test___22.assert(t___5531, fn__5506.clone());
        test___22.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__1021() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___23 = temper_std::testing::Test::new();
        let params__494: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___5492: TableDef = userTable__343();
        let mut t___5493: SafeIdentifier = csid__342("name");
        let cs__495: Changeset = changeset(t___5492.clone(), params__494.clone()).cast(std::sync::Arc::new(vec![t___5493.clone()])).cast(std::sync::Arc::new(vec![csid__342("email")]));
        let mut t___5500: bool = ! temper_core::MappedTrait::has( & cs__495.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__5488(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__5488 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5488())
        };
        test___23.assert(t___5500, fn__5488.clone());
        let mut t___5503: bool = temper_core::MappedTrait::has( & cs__495.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__5487(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__5487 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5487())
        };
        test___23.assert(t___5503, fn__5487.clone());
        test___23.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__1022() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let params__497: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___5474: TableDef = userTable__343();
        let mut t___5475: SafeIdentifier = csid__342("name");
        let mut t___5476: SafeIdentifier = csid__342("email");
        let cs__498: Changeset = changeset(t___5474.clone(), params__497.clone()).cast(std::sync::Arc::new(vec![t___5475.clone(), t___5476.clone()]));
        let mut t___5481: bool = ! temper_core::MappedTrait::has( & cs__498.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__5470(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__5470 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5470())
        };
        test___24.assert(t___5481, fn__5470.clone());
        let mut t___5484: bool = temper_core::MappedTrait::has( & cs__498.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__5469(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__5469 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5469())
        };
        test___24.assert(t___5484, fn__5469.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__1023() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let params__500: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___5456: TableDef = userTable__343();
        let mut t___5457: SafeIdentifier = csid__342("name");
        let cs__501: Changeset = changeset(t___5456.clone(), params__500.clone()).cast(std::sync::Arc::new(vec![t___5457.clone()])).validate_required(std::sync::Arc::new(vec![csid__342("name")]));
        let mut t___5461: bool = cs__501.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__5453(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__5453 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5453())
        };
        test___25.assert(t___5461, fn__5453.clone());
        let mut t___5467: bool = Some(temper_core::ListedTrait::len( & cs__501.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__5452(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__5452 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5452())
        };
        test___25.assert(t___5467, fn__5452.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__1024() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let params__503: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___5432: TableDef = userTable__343();
        let mut t___5433: SafeIdentifier = csid__342("name");
        let cs__504: Changeset = changeset(t___5432.clone(), params__503.clone()).cast(std::sync::Arc::new(vec![t___5433.clone()])).validate_required(std::sync::Arc::new(vec![csid__342("name")]));
        let mut t___5439: bool = ! cs__504.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__5430(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__5430 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5430())
        };
        test___26.assert(t___5439, fn__5430.clone());
        let mut t___5444: bool = Some(temper_core::ListedTrait::len( & cs__504.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__5429(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__5429 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5429())
        };
        test___26.assert(t___5444, fn__5429.clone());
        let mut t___5450: bool = Some(temper_core::ListedTrait::get( & cs__504.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__5428(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__5428 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5428())
        };
        test___26.assert(t___5450, fn__5428.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__1025() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let params__506: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___5420: TableDef = userTable__343();
        let mut t___5421: SafeIdentifier = csid__342("name");
        let cs__507: Changeset = changeset(t___5420.clone(), params__506.clone()).cast(std::sync::Arc::new(vec![t___5421.clone()])).validate_length(csid__342("name"), 2, 50);
        let mut t___5425: bool = cs__507.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__5417(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__5417 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5417())
        };
        test___27.assert(t___5425, fn__5417.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__1026() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let params__509: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___5408: TableDef = userTable__343();
        let mut t___5409: SafeIdentifier = csid__342("name");
        let cs__510: Changeset = changeset(t___5408.clone(), params__509.clone()).cast(std::sync::Arc::new(vec![t___5409.clone()])).validate_length(csid__342("name"), 2, 50);
        let mut t___5415: bool = ! cs__510.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__5405(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__5405 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5405())
        };
        test___28.assert(t___5415, fn__5405.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__1027() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let params__512: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___5396: TableDef = userTable__343();
        let mut t___5397: SafeIdentifier = csid__342("name");
        let cs__513: Changeset = changeset(t___5396.clone(), params__512.clone()).cast(std::sync::Arc::new(vec![t___5397.clone()])).validate_length(csid__342("name"), 2, 10);
        let mut t___5403: bool = ! cs__513.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__5393(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__5393 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5393())
        };
        test___29.assert(t___5403, fn__5393.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__1028() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let params__515: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___5385: TableDef = userTable__343();
        let mut t___5386: SafeIdentifier = csid__342("age");
        let cs__516: Changeset = changeset(t___5385.clone(), params__515.clone()).cast(std::sync::Arc::new(vec![t___5386.clone()])).validate_int(csid__342("age"));
        let mut t___5390: bool = cs__516.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__5382(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__5382 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5382())
        };
        test___30.assert(t___5390, fn__5382.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__1029() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__518: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___5373: TableDef = userTable__343();
        let mut t___5374: SafeIdentifier = csid__342("age");
        let cs__519: Changeset = changeset(t___5373.clone(), params__518.clone()).cast(std::sync::Arc::new(vec![t___5374.clone()])).validate_int(csid__342("age"));
        let mut t___5380: bool = ! cs__519.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__5370(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__5370 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5370())
        };
        test___31.assert(t___5380, fn__5370.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__1030() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__521: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___5362: TableDef = userTable__343();
        let mut t___5363: SafeIdentifier = csid__342("score");
        let cs__522: Changeset = changeset(t___5362.clone(), params__521.clone()).cast(std::sync::Arc::new(vec![t___5363.clone()])).validate_float(csid__342("score"));
        let mut t___5367: bool = cs__522.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__5359(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__5359 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5359())
        };
        test___32.assert(t___5367, fn__5359.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__1031() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__524: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___5351: TableDef = userTable__343();
        let mut t___5352: SafeIdentifier = csid__342("age");
        let cs__525: Changeset = changeset(t___5351.clone(), params__524.clone()).cast(std::sync::Arc::new(vec![t___5352.clone()])).validate_int64(csid__342("age"));
        let mut t___5356: bool = cs__525.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__5348(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__5348 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5348())
        };
        test___33.assert(t___5356, fn__5348.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__1032() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__527: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___5339: TableDef = userTable__343();
        let mut t___5340: SafeIdentifier = csid__342("age");
        let cs__528: Changeset = changeset(t___5339.clone(), params__527.clone()).cast(std::sync::Arc::new(vec![t___5340.clone()])).validate_int64(csid__342("age"));
        let mut t___5346: bool = ! cs__528.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__5336(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__5336 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5336())
        };
        test___34.assert(t___5346, fn__5336.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__1033() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___37 {
            test___35: temper_std::testing::Test
        }
        impl ClosureGroup___37 {
            fn fn__5333(& self, v__530: impl temper_core::ToArcString) {
                let v__530 = v__530.to_arc_string();
                let params__531: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__530.clone())]);
                let mut t___5325: TableDef = userTable__343();
                let mut t___5326: SafeIdentifier = csid__342("active");
                let cs__532: Changeset = changeset(t___5325.clone(), params__531.clone()).cast(std::sync::Arc::new(vec![t___5326.clone()])).validate_bool(csid__342("active"));
                let mut t___5330: bool = cs__532.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___38 {
                    v__530: std::sync::Arc<String>
                }
                impl ClosureGroup___38 {
                    fn fn__5322(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__530.clone()));
                    }
                }
                let closure_group = ClosureGroup___38 {
                    v__530: v__530.clone()
                };
                let fn__5322 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__5322())
                };
                self.test___35.assert(t___5330, fn__5322.clone());
            }
        }
        let closure_group = ClosureGroup___37 {
            test___35: test___35.clone()
        };
        let fn__5333 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__530: std::sync::Arc<String> | closure_group.fn__5333(v__530))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__5333.clone()));
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__1034() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___39 {
            test___36: temper_std::testing::Test
        }
        impl ClosureGroup___39 {
            fn fn__5319(& self, v__534: impl temper_core::ToArcString) {
                let v__534 = v__534.to_arc_string();
                let params__535: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__534.clone())]);
                let mut t___5311: TableDef = userTable__343();
                let mut t___5312: SafeIdentifier = csid__342("active");
                let cs__536: Changeset = changeset(t___5311.clone(), params__535.clone()).cast(std::sync::Arc::new(vec![t___5312.clone()])).validate_bool(csid__342("active"));
                let mut t___5316: bool = cs__536.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___40 {
                    v__534: std::sync::Arc<String>
                }
                impl ClosureGroup___40 {
                    fn fn__5308(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__534.clone()));
                    }
                }
                let closure_group = ClosureGroup___40 {
                    v__534: v__534.clone()
                };
                let fn__5308 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__5308())
                };
                self.test___36.assert(t___5316, fn__5308.clone());
            }
        }
        let closure_group = ClosureGroup___39 {
            test___36: test___36.clone()
        };
        let fn__5319 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__534: std::sync::Arc<String> | closure_group.fn__5319(v__534))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__5319.clone()));
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__1035() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___41 {
            test___37: temper_std::testing::Test
        }
        impl ClosureGroup___41 {
            fn fn__5305(& self, v__538: impl temper_core::ToArcString) {
                let v__538 = v__538.to_arc_string();
                let params__539: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__538.clone())]);
                let mut t___5296: TableDef = userTable__343();
                let mut t___5297: SafeIdentifier = csid__342("active");
                let cs__540: Changeset = changeset(t___5296.clone(), params__539.clone()).cast(std::sync::Arc::new(vec![t___5297.clone()])).validate_bool(csid__342("active"));
                let mut t___5303: bool = ! cs__540.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___42 {
                    v__538: std::sync::Arc<String>
                }
                impl ClosureGroup___42 {
                    fn fn__5293(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__538.clone()));
                    }
                }
                let closure_group = ClosureGroup___42 {
                    v__538: v__538.clone()
                };
                let fn__5293 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__5293())
                };
                self.test___37.assert(t___5303, fn__5293.clone());
            }
        }
        let closure_group = ClosureGroup___41 {
            test___37: test___37.clone()
        };
        let fn__5305 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__538: std::sync::Arc<String> | closure_group.fn__5305(v__538))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__5305.clone()));
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__1036() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        let mut t___2989: SqlFragment;
        let params__542: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___5281: TableDef = userTable__343();
        let mut t___5282: SafeIdentifier = csid__342("name");
        let mut t___5283: SafeIdentifier = csid__342("email");
        let cs__543: Changeset = changeset(t___5281.clone(), params__542.clone()).cast(std::sync::Arc::new(vec![t___5282.clone(), t___5283.clone()])).validate_required(std::sync::Arc::new(vec![csid__342("name"), csid__342("email")]));
        let sqlFrag__544: SqlFragment;
        'ok___5833: {
            'orelse___1216: {
                t___2989 = match cs__543.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1216
                };
                sqlFrag__544 = t___2989.clone();
                break 'ok___5833;
            }
            sqlFrag__544 = panic!();
        }
        let s__545: std::sync::Arc<String> = sqlFrag__544.to_string();
        let mut t___5290: bool = temper_core::string::index_of( & s__545, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___43 {
            s__545: std::sync::Arc<String>
        }
        impl ClosureGroup___43 {
            fn fn__5277(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__545.clone()));
            }
        }
        let closure_group = ClosureGroup___43 {
            s__545: s__545.clone()
        };
        let fn__5277 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5277())
        };
        test___38.assert(t___5290, fn__5277.clone());
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__1037() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        let mut t___2968: SqlFragment;
        let params__547: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___5261: TableDef = userTable__343();
        let mut t___5262: SafeIdentifier = csid__342("name");
        let mut t___5263: SafeIdentifier = csid__342("email");
        let cs__548: Changeset = changeset(t___5261.clone(), params__547.clone()).cast(std::sync::Arc::new(vec![t___5262.clone(), t___5263.clone()])).validate_required(std::sync::Arc::new(vec![csid__342("name"), csid__342("email")]));
        let sqlFrag__549: SqlFragment;
        'ok___5836: {
            'orelse___1217: {
                t___2968 = match cs__548.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1217
                };
                sqlFrag__549 = t___2968.clone();
                break 'ok___5836;
            }
            sqlFrag__549 = panic!();
        }
        let s__550: std::sync::Arc<String> = sqlFrag__549.to_string();
        let mut t___5270: bool = temper_core::string::index_of( & s__550, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            s__550: std::sync::Arc<String>
        }
        impl ClosureGroup___44 {
            fn fn__5257(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__550.clone()));
            }
        }
        let closure_group = ClosureGroup___44 {
            s__550: s__550.clone()
        };
        let fn__5257 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5257())
        };
        test___39.assert(t___5270, fn__5257.clone());
        let mut t___5274: bool = temper_core::string::index_of( & s__550, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___45 {
            s__550: std::sync::Arc<String>
        }
        impl ClosureGroup___45 {
            fn fn__5256(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__550.clone()));
            }
        }
        let closure_group = ClosureGroup___45 {
            s__550: s__550.clone()
        };
        let fn__5256 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5256())
        };
        test___39.assert(t___5274, fn__5256.clone());
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__1038() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let mut t___2951: SqlFragment;
        let params__552: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___5243: TableDef = userTable__343();
        let mut t___5244: SafeIdentifier = csid__342("name");
        let mut t___5245: SafeIdentifier = csid__342("email");
        let mut t___5246: SafeIdentifier = csid__342("age");
        let cs__553: Changeset = changeset(t___5243.clone(), params__552.clone()).cast(std::sync::Arc::new(vec![t___5244.clone(), t___5245.clone(), t___5246.clone()])).validate_required(std::sync::Arc::new(vec![csid__342("name"), csid__342("email")]));
        let sqlFrag__554: SqlFragment;
        'ok___5837: {
            'orelse___1218: {
                t___2951 = match cs__553.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1218
                };
                sqlFrag__554 = t___2951.clone();
                break 'ok___5837;
            }
            sqlFrag__554 = panic!();
        }
        let s__555: std::sync::Arc<String> = sqlFrag__554.to_string();
        let mut t___5253: bool = temper_core::string::index_of( & s__555, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            s__555: std::sync::Arc<String>
        }
        impl ClosureGroup___46 {
            fn fn__5238(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__555.clone()));
            }
        }
        let closure_group = ClosureGroup___46 {
            s__555: s__555.clone()
        };
        let fn__5238 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5238())
        };
        test___40.assert(t___5253, fn__5238.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__1039() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let params__557: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___5231: TableDef = userTable__343();
        let mut t___5232: SafeIdentifier = csid__342("name");
        let cs__558: Changeset = changeset(t___5231.clone(), params__557.clone()).cast(std::sync::Arc::new(vec![t___5232.clone()])).validate_required(std::sync::Arc::new(vec![csid__342("name")]));
        let didBubble__559: bool;
        'ok___5838: {
            'orelse___1219: {
                match cs__558.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1219
                };
                didBubble__559 = false;
                break 'ok___5838;
            }
            didBubble__559 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___47 {}
        impl ClosureGroup___47 {
            fn fn__5229(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___47 {};
        let fn__5229 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5229())
        };
        test___41.assert(didBubble__559, fn__5229.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__1040() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let strictTable__561: TableDef = TableDef::new(csid__342("posts"), [FieldDef::new(csid__342("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__342("body"), FieldType::new(StringField::new()), true)]);
        let params__562: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___5222: SafeIdentifier = csid__342("body");
        let cs__563: Changeset = changeset(strictTable__561.clone(), params__562.clone()).cast(std::sync::Arc::new(vec![t___5222.clone()]));
        let mut t___5224: bool = cs__563.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___48 {}
        impl ClosureGroup___48 {
            fn fn__5211(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___48 {};
        let fn__5211 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5211())
        };
        test___42.assert(t___5224, fn__5211.clone());
        let didBubble__564: bool;
        'ok___5839: {
            'orelse___1220: {
                match cs__563.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1220
                };
                didBubble__564 = false;
                break 'ok___5839;
            }
            didBubble__564 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___49 {}
        impl ClosureGroup___49 {
            fn fn__5210(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___49 {};
        let fn__5210 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5210())
        };
        test___42.assert(didBubble__564, fn__5210.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__1041() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let mut t___2911: SqlFragment;
        let params__566: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___5201: TableDef = userTable__343();
        let mut t___5202: SafeIdentifier = csid__342("name");
        let cs__567: Changeset = changeset(t___5201.clone(), params__566.clone()).cast(std::sync::Arc::new(vec![t___5202.clone()])).validate_required(std::sync::Arc::new(vec![csid__342("name")]));
        let sqlFrag__568: SqlFragment;
        'ok___5840: {
            'orelse___1221: {
                t___2911 = match cs__567.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___1221
                };
                sqlFrag__568 = t___2911.clone();
                break 'ok___5840;
            }
            sqlFrag__568 = panic!();
        }
        let s__569: std::sync::Arc<String> = sqlFrag__568.to_string();
        let mut t___5208: bool = Some(s__569.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___50 {
            s__569: std::sync::Arc<String>
        }
        impl ClosureGroup___50 {
            fn fn__5198(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__569.clone()));
            }
        }
        let closure_group = ClosureGroup___50 {
            s__569: s__569.clone()
        };
        let fn__5198 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5198())
        };
        test___43.assert(t___5208, fn__5198.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__1042() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let params__571: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___5191: TableDef = userTable__343();
        let mut t___5192: SafeIdentifier = csid__342("name");
        let cs__572: Changeset = changeset(t___5191.clone(), params__571.clone()).cast(std::sync::Arc::new(vec![t___5192.clone()])).validate_required(std::sync::Arc::new(vec![csid__342("name")]));
        let didBubble__573: bool;
        'ok___5841: {
            'orelse___1222: {
                match cs__572.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___1222
                };
                didBubble__573 = false;
                break 'ok___5841;
            }
            didBubble__573 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___51 {}
        impl ClosureGroup___51 {
            fn fn__5189(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___51 {};
        let fn__5189 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5189())
        };
        test___44.assert(didBubble__573, fn__5189.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__1079() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let q__681: Query = from(sid__344("users"));
        let mut t___5084: bool = Some(q__681.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__5079(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__5079 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5079())
        };
        test___45.assert(t___5084, fn__5079.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__1080() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let mut t___5070: SafeIdentifier = sid__344("users");
        let mut t___5071: SafeIdentifier = sid__344("id");
        let mut t___5072: SafeIdentifier = sid__344("name");
        let q__683: Query = from(t___5070.clone()).select([t___5071.clone(), t___5072.clone()]);
        let mut t___5077: bool = Some(q__683.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___53 {}
        impl ClosureGroup___53 {
            fn fn__5069(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___53 {};
        let fn__5069 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5069())
        };
        test___46.assert(t___5077, fn__5069.clone());
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__1081() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let mut t___5058: SafeIdentifier = sid__344("users");
        let mut t___5059: SqlBuilder = SqlBuilder::new();
        t___5059.append_safe("age > ");
        t___5059.append_int32(18);
        let mut t___5062: SqlFragment = t___5059.accumulated();
        let q__685: Query = from(t___5058.clone()).r#where(t___5062.clone());
        let mut t___5067: bool = Some(q__685.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__5057(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__5057 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5057())
        };
        test___47.assert(t___5067, fn__5057.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__1083() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___5046: SafeIdentifier = sid__344("users");
        let mut t___5047: SqlBuilder = SqlBuilder::new();
        t___5047.append_safe("active = ");
        t___5047.append_boolean(true);
        let mut t___5050: SqlFragment = t___5047.accumulated();
        let q__687: Query = from(t___5046.clone()).r#where(t___5050.clone());
        let mut t___5055: bool = Some(q__687.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__5045(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__5045 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5045())
        };
        test___48.assert(t___5055, fn__5045.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__1085() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___5029: SafeIdentifier = sid__344("users");
        let mut t___5030: SqlBuilder = SqlBuilder::new();
        t___5030.append_safe("age > ");
        t___5030.append_int32(18);
        let mut t___5033: SqlFragment = t___5030.accumulated();
        let mut t___5034: Query = from(t___5029.clone()).r#where(t___5033.clone());
        let mut t___5035: SqlBuilder = SqlBuilder::new();
        t___5035.append_safe("active = ");
        t___5035.append_boolean(true);
        let q__689: Query = t___5034.r#where(t___5035.accumulated());
        let mut t___5043: bool = Some(q__689.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__5028(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__5028 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5028())
        };
        test___49.assert(t___5043, fn__5028.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__1088() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___5020: SafeIdentifier = sid__344("users");
        let mut t___5021: SafeIdentifier = sid__344("name");
        let q__691: Query = from(t___5020.clone()).order_by(t___5021.clone(), true);
        let mut t___5026: bool = Some(q__691.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__5019(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__5019 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5019())
        };
        test___50.assert(t___5026, fn__5019.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__1089() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let mut t___5011: SafeIdentifier = sid__344("users");
        let mut t___5012: SafeIdentifier = sid__344("created_at");
        let q__693: Query = from(t___5011.clone()).order_by(t___5012.clone(), false);
        let mut t___5017: bool = Some(q__693.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__5010(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__5010 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5010())
        };
        test___51.assert(t___5017, fn__5010.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__1090() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___2715: Query;
        let mut t___2716: Query;
        let q__695: Query;
        'ok___5843: {
            'orelse___1224: {
                t___2715 = match from(sid__344("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1224
                };
                t___2716 = match t___2715.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1224
                };
                q__695 = t___2716.clone();
                break 'ok___5843;
            }
            q__695 = panic!();
        }
        let mut t___5008: bool = Some(q__695.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__5003(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__5003 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5003())
        };
        test___52.assert(t___5008, fn__5003.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__1091() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let didBubble__697: bool;
        'ok___5844: {
            'orelse___1225: {
                match from(sid__344("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1225
                };
                didBubble__697 = false;
                break 'ok___5844;
            }
            didBubble__697 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__4999(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__4999 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4999())
        };
        test___53.assert(didBubble__697, fn__4999.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__1092() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let didBubble__699: bool;
        'ok___5845: {
            'orelse___1226: {
                match from(sid__344("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1226
                };
                didBubble__699 = false;
                break 'ok___5845;
            }
            didBubble__699 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__4995(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__4995 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4995())
        };
        test___54.assert(didBubble__699, fn__4995.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__1093() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let mut t___4973: SafeIdentifier;
        let mut t___4974: SafeIdentifier;
        let mut t___4975: SafeIdentifier;
        let mut t___4976: SafeIdentifier;
        let mut t___4977: Query;
        let mut t___4978: SqlBuilder;
        let mut t___4982: Query;
        let mut t___4983: SqlBuilder;
        let mut t___2701: Query;
        let mut t___2702: Query;
        let minAge__701: i32 = 21;
        let q__702: Query;
        'ok___5846: {
            'orelse___1227: {
                t___4973 = sid__344("users");
                t___4974 = sid__344("id");
                t___4975 = sid__344("name");
                t___4976 = sid__344("email");
                t___4977 = from(t___4973.clone()).select([t___4974.clone(), t___4975.clone(), t___4976.clone()]);
                t___4978 = SqlBuilder::new();
                t___4978.append_safe("age >= ");
                t___4978.append_int32(21);
                t___4982 = t___4977.r#where(t___4978.accumulated());
                t___4983 = SqlBuilder::new();
                t___4983.append_safe("active = ");
                t___4983.append_boolean(true);
                t___2701 = match t___4982.r#where(t___4983.accumulated()).order_by(sid__344("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___1227
                };
                t___2702 = match t___2701.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___1227
                };
                q__702 = t___2702.clone();
                break 'ok___5846;
            }
            q__702 = panic!();
        }
        let mut t___4993: bool = Some(q__702.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__4972(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__4972 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4972())
        };
        test___55.assert(t___4993, fn__4972.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__1096() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let mut t___2678: SqlFragment;
        let mut t___2679: SqlFragment;
        let q__704: Query = from(sid__344("users"));
        'ok___5847: {
            'orelse___1228: {
                t___2678 = match q__704.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1228
                };
                t___2679 = t___2678.clone();
                break 'ok___5847;
            }
            t___2679 = panic!();
        }
        let s__705: std::sync::Arc<String> = t___2679.to_string();
        let mut t___4970: bool = Some(s__705.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___63 {
            s__705: std::sync::Arc<String>
        }
        impl ClosureGroup___63 {
            fn fn__4966(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__705.clone()));
            }
        }
        let closure_group = ClosureGroup___63 {
            s__705: s__705.clone()
        };
        let fn__4966 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4966())
        };
        test___56.assert(t___4970, fn__4966.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__1097() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___2670: Query;
        let mut t___2673: SqlFragment;
        let mut t___2674: SqlFragment;
        let q__707: Query;
        'ok___5848: {
            'orelse___1229: {
                t___2670 = match from(sid__344("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___1229
                };
                q__707 = t___2670.clone();
                break 'ok___5848;
            }
            q__707 = panic!();
        }
        'ok___5849: {
            'orelse___1230: {
                t___2673 = match q__707.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1230
                };
                t___2674 = t___2673.clone();
                break 'ok___5849;
            }
            t___2674 = panic!();
        }
        let s__708: std::sync::Arc<String> = t___2674.to_string();
        let mut t___4964: bool = Some(s__708.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___64 {
            s__708: std::sync::Arc<String>
        }
        impl ClosureGroup___64 {
            fn fn__4960(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__708.clone()));
            }
        }
        let closure_group = ClosureGroup___64 {
            s__708: s__708.clone()
        };
        let fn__4960 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4960())
        };
        test___57.assert(t___4964, fn__4960.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__1098() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let didBubble__710: bool;
        'ok___5850: {
            'orelse___1231: {
                match from(sid__344("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1231
                };
                didBubble__710 = false;
                break 'ok___5850;
            }
            didBubble__710 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__4956(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__4956 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4956())
        };
        test___58.assert(didBubble__710, fn__4956.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__1099() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let evil__712: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___4940: SafeIdentifier = sid__344("users");
        let mut t___4941: SqlBuilder = SqlBuilder::new();
        t___4941.append_safe("name = ");
        t___4941.append_string("'; DROP TABLE users; --");
        let mut t___4944: SqlFragment = t___4941.accumulated();
        let q__713: Query = from(t___4940.clone()).r#where(t___4944.clone());
        let s__714: std::sync::Arc<String> = q__713.to_sql().to_string();
        let mut t___4949: bool = temper_core::string::index_of( & s__714, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___66 {
            s__714: std::sync::Arc<String>
        }
        impl ClosureGroup___66 {
            fn fn__4939(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__714.clone()));
            }
        }
        let closure_group = ClosureGroup___66 {
            s__714: s__714.clone()
        };
        let fn__4939 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4939())
        };
        test___59.assert(t___4949, fn__4939.clone());
        let mut t___4953: bool = temper_core::string::index_of( & s__714, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___67 {
            s__714: std::sync::Arc<String>
        }
        impl ClosureGroup___67 {
            fn fn__4938(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__714.clone()));
            }
        }
        let closure_group = ClosureGroup___67 {
            s__714: s__714.clone()
        };
        let fn__4938 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4938())
        };
        test___59.assert(t___4953, fn__4938.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__1101() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let attack__716: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__717: bool;
        'ok___5851: {
            'orelse___1232: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___1232
                };
                didBubble__717 = false;
                break 'ok___5851;
            }
            didBubble__717 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__4935(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__4935 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4935())
        };
        test___60.assert(didBubble__717, fn__4935.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__1102() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let mut t___4924: SafeIdentifier = sid__344("users");
        let mut t___4925: SafeIdentifier = sid__344("orders");
        let mut t___4926: SqlBuilder = SqlBuilder::new();
        t___4926.append_safe("users.id = orders.user_id");
        let mut t___4928: SqlFragment = t___4926.accumulated();
        let q__719: Query = from(t___4924.clone()).inner_join(t___4925.clone(), t___4928.clone());
        let mut t___4933: bool = Some(q__719.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___69 {}
        impl ClosureGroup___69 {
            fn fn__4923(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___69 {};
        let fn__4923 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4923())
        };
        test___61.assert(t___4933, fn__4923.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__1104() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let mut t___4912: SafeIdentifier = sid__344("users");
        let mut t___4913: SafeIdentifier = sid__344("profiles");
        let mut t___4914: SqlBuilder = SqlBuilder::new();
        t___4914.append_safe("users.id = profiles.user_id");
        let mut t___4916: SqlFragment = t___4914.accumulated();
        let q__721: Query = from(t___4912.clone()).left_join(t___4913.clone(), t___4916.clone());
        let mut t___4921: bool = Some(q__721.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___70 {}
        impl ClosureGroup___70 {
            fn fn__4911(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___70 {};
        let fn__4911 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4911())
        };
        test___62.assert(t___4921, fn__4911.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__1106() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let mut t___4900: SafeIdentifier = sid__344("orders");
        let mut t___4901: SafeIdentifier = sid__344("users");
        let mut t___4902: SqlBuilder = SqlBuilder::new();
        t___4902.append_safe("orders.user_id = users.id");
        let mut t___4904: SqlFragment = t___4902.accumulated();
        let q__723: Query = from(t___4900.clone()).right_join(t___4901.clone(), t___4904.clone());
        let mut t___4909: bool = Some(q__723.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__4899(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__4899 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4899())
        };
        test___63.assert(t___4909, fn__4899.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__1108() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let mut t___4888: SafeIdentifier = sid__344("users");
        let mut t___4889: SafeIdentifier = sid__344("orders");
        let mut t___4890: SqlBuilder = SqlBuilder::new();
        t___4890.append_safe("users.id = orders.user_id");
        let mut t___4892: SqlFragment = t___4890.accumulated();
        let q__725: Query = from(t___4888.clone()).full_join(t___4889.clone(), t___4892.clone());
        let mut t___4897: bool = Some(q__725.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__4887(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__4887 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4887())
        };
        test___64.assert(t___4897, fn__4887.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__1110() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let mut t___4871: SafeIdentifier = sid__344("users");
        let mut t___4872: SafeIdentifier = sid__344("orders");
        let mut t___4873: SqlBuilder = SqlBuilder::new();
        t___4873.append_safe("users.id = orders.user_id");
        let mut t___4875: SqlFragment = t___4873.accumulated();
        let mut t___4876: Query = from(t___4871.clone()).inner_join(t___4872.clone(), t___4875.clone());
        let mut t___4877: SafeIdentifier = sid__344("profiles");
        let mut t___4878: SqlBuilder = SqlBuilder::new();
        t___4878.append_safe("users.id = profiles.user_id");
        let q__727: Query = t___4876.left_join(t___4877.clone(), t___4878.accumulated());
        let mut t___4885: bool = Some(q__727.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__4870(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__4870 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4870())
        };
        test___65.assert(t___4885, fn__4870.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__1113() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let mut t___4852: SafeIdentifier;
        let mut t___4853: SafeIdentifier;
        let mut t___4854: SqlBuilder;
        let mut t___4856: SqlFragment;
        let mut t___4857: Query;
        let mut t___4858: SqlBuilder;
        let mut t___2585: Query;
        let q__729: Query;
        'ok___5852: {
            'orelse___1233: {
                t___4852 = sid__344("users");
                t___4853 = sid__344("orders");
                t___4854 = SqlBuilder::new();
                t___4854.append_safe("users.id = orders.user_id");
                t___4856 = t___4854.accumulated();
                t___4857 = from(t___4852.clone()).inner_join(t___4853.clone(), t___4856.clone());
                t___4858 = SqlBuilder::new();
                t___4858.append_safe("orders.total > ");
                t___4858.append_int32(100);
                t___2585 = match t___4857.r#where(t___4858.accumulated()).order_by(sid__344("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1233
                };
                q__729 = t___2585.clone();
                break 'ok___5852;
            }
            q__729 = panic!();
        }
        let mut t___4868: bool = Some(q__729.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__4851(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__4851 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4851())
        };
        test___66.assert(t___4868, fn__4851.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__1116() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let c__731: SqlFragment = col(sid__344("users"), sid__344("id"));
        let mut t___4849: bool = Some(c__731.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__4843(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__4843 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4843())
        };
        test___67.assert(t___4849, fn__4843.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__1117() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let onCond__733: SqlFragment = col(sid__344("users"), sid__344("id"));
        let b__734: SqlBuilder = SqlBuilder::new();
        b__734.append_fragment(onCond__733.clone());
        b__734.append_safe(" = ");
        b__734.append_fragment(col(sid__344("orders"), sid__344("user_id")));
        let mut t___4834: SafeIdentifier = sid__344("users");
        let mut t___4835: SafeIdentifier = sid__344("orders");
        let mut t___4836: SqlFragment = b__734.accumulated();
        let q__735: Query = from(t___4834.clone()).inner_join(t___4835.clone(), t___4836.clone());
        let mut t___4841: bool = Some(q__735.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__4823(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__4823 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4823())
        };
        test___68.assert(t___4841, fn__4823.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__1118() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let mut t___2544: SafeIdentifier;
        let id__773: SafeIdentifier;
        'ok___5853: {
            'orelse___1234: {
                t___2544 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___1234
                };
                id__773 = t___2544.clone();
                break 'ok___5853;
            }
            id__773 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4821: bool = Some(id__773.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__4818(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__4818 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4818())
        };
        test___75.assert(t___4821, fn__4818.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__1119() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        let didBubble__775: bool;
        'ok___5854: {
            'orelse___1235: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___1235
                };
                didBubble__775 = false;
                break 'ok___5854;
            }
            didBubble__775 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__4815(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__4815 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4815())
        };
        test___76.assert(didBubble__775, fn__4815.clone());
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__1120() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let didBubble__777: bool;
        'ok___5855: {
            'orelse___1236: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___1236
                };
                didBubble__777 = false;
                break 'ok___5855;
            }
            didBubble__777 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__4812(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__4812 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4812())
        };
        test___77.assert(didBubble__777, fn__4812.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__1121() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let cases__779: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___80 {
            test___78: temper_std::testing::Test
        }
        impl ClosureGroup___80 {
            fn fn__4809(& self, c__780: impl temper_core::ToArcString) {
                let c__780 = c__780.to_arc_string();
                let didBubble__781: bool;
                'ok___5856: {
                    'orelse___1237: {
                        match safe_identifier(c__780.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___1237
                        };
                        didBubble__781 = false;
                        break 'ok___5856;
                    }
                    didBubble__781 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___81 {
                    c__780: std::sync::Arc<String>
                }
                impl ClosureGroup___81 {
                    fn fn__4806(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__780.clone()));
                    }
                }
                let closure_group = ClosureGroup___81 {
                    c__780: c__780.clone()
                };
                let fn__4806 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__4806())
                };
                self.test___78.assert(didBubble__781, fn__4806.clone());
            }
        }
        let closure_group = ClosureGroup___80 {
            test___78: test___78.clone()
        };
        let fn__4809 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__780: std::sync::Arc<String> | closure_group.fn__4809(c__780))
        };
        temper_core::listed::list_for_each( & cases__779, & ( * fn__4809.clone()));
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__1122() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let mut t___2521: SafeIdentifier;
        let mut t___2522: SafeIdentifier;
        let mut t___2523: SafeIdentifier;
        let mut t___2524: SafeIdentifier;
        let mut t___2527: SafeIdentifier;
        let mut t___2528: SafeIdentifier;
        let mut t___2532: FieldDef;
        'ok___5857: {
            'orelse___1238: {
                t___2521 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1238
                };
                t___2522 = t___2521.clone();
                break 'ok___5857;
            }
            t___2522 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___5858: {
            'orelse___1239: {
                t___2523 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1239
                };
                t___2524 = t___2523.clone();
                break 'ok___5858;
            }
            t___2524 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4796: StringField = StringField::new();
        let mut t___4797: FieldDef = FieldDef::new(t___2524.clone(), FieldType::new(t___4796.clone()), false);
        'ok___5859: {
            'orelse___1240: {
                t___2527 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1240
                };
                t___2528 = t___2527.clone();
                break 'ok___5859;
            }
            t___2528 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4798: IntField = IntField::new();
        let mut t___4799: FieldDef = FieldDef::new(t___2528.clone(), FieldType::new(t___4798.clone()), false);
        let td__783: TableDef = TableDef::new(t___2522.clone(), [t___4797.clone(), t___4799.clone()]);
        let f__784: FieldDef;
        'ok___5860: {
            'orelse___1241: {
                t___2532 = match td__783.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1241
                };
                f__784 = t___2532.clone();
                break 'ok___5860;
            }
            f__784 = panic!();
        }
        let mut t___4804: bool = Some(f__784.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__4795(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__4795 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4795())
        };
        test___79.assert(t___4804, fn__4795.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__1123() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let mut t___2512: SafeIdentifier;
        let mut t___2513: SafeIdentifier;
        let mut t___2514: SafeIdentifier;
        let mut t___2515: SafeIdentifier;
        'ok___5861: {
            'orelse___1242: {
                t___2512 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1242
                };
                t___2513 = t___2512.clone();
                break 'ok___5861;
            }
            t___2513 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___5862: {
            'orelse___1243: {
                t___2514 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1243
                };
                t___2515 = t___2514.clone();
                break 'ok___5862;
            }
            t___2515 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4790: StringField = StringField::new();
        let mut t___4791: FieldDef = FieldDef::new(t___2515.clone(), FieldType::new(t___4790.clone()), false);
        let td__786: TableDef = TableDef::new(t___2513.clone(), [t___4791.clone()]);
        let didBubble__787: bool;
        'ok___5863: {
            'orelse___1244: {
                match td__786.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___1244
                };
                didBubble__787 = false;
                break 'ok___5863;
            }
            didBubble__787 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__4789(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__4789 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4789())
        };
        test___80.assert(didBubble__787, fn__4789.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__1124() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let mut t___2500: SafeIdentifier;
        let mut t___2501: SafeIdentifier;
        let mut t___2504: SafeIdentifier;
        let mut t___2505: SafeIdentifier;
        'ok___5864: {
            'orelse___1245: {
                t___2500 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___1245
                };
                t___2501 = t___2500.clone();
                break 'ok___5864;
            }
            t___2501 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4778: StringField = StringField::new();
        let required__789: FieldDef = FieldDef::new(t___2501.clone(), FieldType::new(t___4778.clone()), false);
        'ok___5865: {
            'orelse___1246: {
                t___2504 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___1246
                };
                t___2505 = t___2504.clone();
                break 'ok___5865;
            }
            t___2505 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4780: StringField = StringField::new();
        let optional__790: FieldDef = FieldDef::new(t___2505.clone(), FieldType::new(t___4780.clone()), true);
        let mut t___4784: bool = ! required__789.nullable();
        #[derive(Clone)]
        struct ClosureGroup___84 {}
        impl ClosureGroup___84 {
            fn fn__4777(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___84 {};
        let fn__4777 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4777())
        };
        test___81.assert(t___4784, fn__4777.clone());
        let mut t___4786: bool = optional__790.nullable();
        #[derive(Clone)]
        struct ClosureGroup___85 {}
        impl ClosureGroup___85 {
            fn fn__4776(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___85 {};
        let fn__4776 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4776())
        };
        test___81.assert(t___4786, fn__4776.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__1125() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn build__916(& self, name__918: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__918 = name__918.to_arc_string();
                let mut t___4758: SqlBuilder = SqlBuilder::new();
                t___4758.append_safe("select * from hi where name = ");
                t___4758.append_string(name__918.clone());
                return t___4758.accumulated().to_string();
            }
            fn buildWrong__917(& self, name__920: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__920 = name__920.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__920.clone()));
            }
        }
        let closure_group = ClosureGroup___86 {};
        let build__916 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__918: std::sync::Arc<String> | closure_group.build__916(name__918))
        };
        let buildWrong__917 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__920: std::sync::Arc<String> | closure_group.buildWrong__917(name__920))
        };
        let actual___1127: std::sync::Arc<String> = build__916(std::sync::Arc::new("world".to_string()));
        let mut t___4768: bool = Some(actual___1127.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___87 {
            actual___1127: std::sync::Arc<String>
        }
        impl ClosureGroup___87 {
            fn fn__4765(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___1127.clone()));
            }
        }
        let closure_group = ClosureGroup___87 {
            actual___1127: actual___1127.clone()
        };
        let fn__4765 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4765())
        };
        test___85.assert(t___4768, fn__4765.clone());
        let bobbyTables__922: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___1129: std::sync::Arc<String> = build__916(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___4772: bool = Some(actual___1129.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___88 {
            actual___1129: std::sync::Arc<String>
        }
        impl ClosureGroup___88 {
            fn fn__4764(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___1129.clone()));
            }
        }
        let closure_group = ClosureGroup___88 {
            actual___1129: actual___1129.clone()
        };
        let fn__4764 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4764())
        };
        test___85.assert(t___4772, fn__4764.clone());
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__4763(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__4763 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4763())
        };
        test___85.assert(true, fn__4763.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__1133() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let mut t___4726: SqlBuilder = SqlBuilder::new();
        t___4726.append_safe("v = ");
        t___4726.append_string("");
        let actual___1134: std::sync::Arc<String> = t___4726.accumulated().to_string();
        let mut t___4732: bool = Some(actual___1134.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___90 {
            actual___1134: std::sync::Arc<String>
        }
        impl ClosureGroup___90 {
            fn fn__4725(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___1134.clone()));
            }
        }
        let closure_group = ClosureGroup___90 {
            actual___1134: actual___1134.clone()
        };
        let fn__4725 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4725())
        };
        test___86.assert(t___4732, fn__4725.clone());
        let mut t___4734: SqlBuilder = SqlBuilder::new();
        t___4734.append_safe("v = ");
        t___4734.append_string("a''b");
        let actual___1137: std::sync::Arc<String> = t___4734.accumulated().to_string();
        let mut t___4740: bool = Some(actual___1137.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___91 {
            actual___1137: std::sync::Arc<String>
        }
        impl ClosureGroup___91 {
            fn fn__4724(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___1137.clone()));
            }
        }
        let closure_group = ClosureGroup___91 {
            actual___1137: actual___1137.clone()
        };
        let fn__4724 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4724())
        };
        test___86.assert(t___4740, fn__4724.clone());
        let mut t___4742: SqlBuilder = SqlBuilder::new();
        t___4742.append_safe("v = ");
        t___4742.append_string("Hello 世界");
        let actual___1140: std::sync::Arc<String> = t___4742.accumulated().to_string();
        let mut t___4748: bool = Some(actual___1140.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___92 {
            actual___1140: std::sync::Arc<String>
        }
        impl ClosureGroup___92 {
            fn fn__4723(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___1140.clone()));
            }
        }
        let closure_group = ClosureGroup___92 {
            actual___1140: actual___1140.clone()
        };
        let fn__4723 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4723())
        };
        test___86.assert(t___4748, fn__4723.clone());
        let mut t___4750: SqlBuilder = SqlBuilder::new();
        t___4750.append_safe("v = ");
        t___4750.append_string("Line1\x0aLine2");
        let actual___1143: std::sync::Arc<String> = t___4750.accumulated().to_string();
        let mut t___4756: bool = Some(actual___1143.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___93 {
            actual___1143: std::sync::Arc<String>
        }
        impl ClosureGroup___93 {
            fn fn__4722(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___1143.clone()));
            }
        }
        let closure_group = ClosureGroup___93 {
            actual___1143: actual___1143.clone()
        };
        let fn__4722 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4722())
        };
        test___86.assert(t___4756, fn__4722.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__1146() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let mut t___2445: temper_std::temporal::Date;
        let mut t___4697: SqlBuilder = SqlBuilder::new();
        t___4697.append_safe("select ");
        t___4697.append_int32(42);
        t___4697.append_safe(", ");
        t___4697.append_int64(43);
        t___4697.append_safe(", ");
        t___4697.append_float64(19.99f64);
        t___4697.append_safe(", ");
        t___4697.append_boolean(true);
        t___4697.append_safe(", ");
        t___4697.append_boolean(false);
        let actual___1147: std::sync::Arc<String> = t___4697.accumulated().to_string();
        let mut t___4711: bool = Some(actual___1147.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___94 {
            actual___1147: std::sync::Arc<String>
        }
        impl ClosureGroup___94 {
            fn fn__4696(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___1147.clone()));
            }
        }
        let closure_group = ClosureGroup___94 {
            actual___1147: actual___1147.clone()
        };
        let fn__4696 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4696())
        };
        test___87.assert(t___4711, fn__4696.clone());
        let date__925: temper_std::temporal::Date;
        'ok___5866: {
            'orelse___1247: {
                t___2445 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1247
                };
                date__925 = t___2445.clone();
                break 'ok___5866;
            }
            date__925 = panic!();
        }
        let mut t___4713: SqlBuilder = SqlBuilder::new();
        t___4713.append_safe("insert into t values (");
        t___4713.append_date(date__925.clone());
        t___4713.append_safe(")");
        let actual___1150: std::sync::Arc<String> = t___4713.accumulated().to_string();
        let mut t___4720: bool = Some(actual___1150.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___95 {
            actual___1150: std::sync::Arc<String>
        }
        impl ClosureGroup___95 {
            fn fn__4695(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___1150.clone()));
            }
        }
        let closure_group = ClosureGroup___95 {
            actual___1150: actual___1150.clone()
        };
        let fn__4695 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4695())
        };
        test___87.assert(t___4720, fn__4695.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn lists__1153() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let mut t___2417: temper_std::temporal::Date;
        let mut t___2418: temper_std::temporal::Date;
        let mut t___2419: temper_std::temporal::Date;
        let mut t___2420: temper_std::temporal::Date;
        let mut t___4641: SqlBuilder = SqlBuilder::new();
        t___4641.append_safe("v IN (");
        t___4641.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___4641.append_safe(")");
        let actual___1154: std::sync::Arc<String> = t___4641.accumulated().to_string();
        let mut t___4648: bool = Some(actual___1154.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___96 {
            actual___1154: std::sync::Arc<String>
        }
        impl ClosureGroup___96 {
            fn fn__4640(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___1154.clone()));
            }
        }
        let closure_group = ClosureGroup___96 {
            actual___1154: actual___1154.clone()
        };
        let fn__4640 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4640())
        };
        test___88.assert(t___4648, fn__4640.clone());
        let mut t___4650: SqlBuilder = SqlBuilder::new();
        t___4650.append_safe("v IN (");
        t___4650.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___4650.append_safe(")");
        let actual___1157: std::sync::Arc<String> = t___4650.accumulated().to_string();
        let mut t___4657: bool = Some(actual___1157.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___97 {
            actual___1157: std::sync::Arc<String>
        }
        impl ClosureGroup___97 {
            fn fn__4639(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___1157.clone()));
            }
        }
        let closure_group = ClosureGroup___97 {
            actual___1157: actual___1157.clone()
        };
        let fn__4639 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4639())
        };
        test___88.assert(t___4657, fn__4639.clone());
        let mut t___4659: SqlBuilder = SqlBuilder::new();
        t___4659.append_safe("v IN (");
        t___4659.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___4659.append_safe(")");
        let actual___1160: std::sync::Arc<String> = t___4659.accumulated().to_string();
        let mut t___4666: bool = Some(actual___1160.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___98 {
            actual___1160: std::sync::Arc<String>
        }
        impl ClosureGroup___98 {
            fn fn__4638(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___1160.clone()));
            }
        }
        let closure_group = ClosureGroup___98 {
            actual___1160: actual___1160.clone()
        };
        let fn__4638 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4638())
        };
        test___88.assert(t___4666, fn__4638.clone());
        let mut t___4668: SqlBuilder = SqlBuilder::new();
        t___4668.append_safe("v IN (");
        t___4668.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___4668.append_safe(")");
        let actual___1163: std::sync::Arc<String> = t___4668.accumulated().to_string();
        let mut t___4675: bool = Some(actual___1163.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___99 {
            actual___1163: std::sync::Arc<String>
        }
        impl ClosureGroup___99 {
            fn fn__4637(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___1163.clone()));
            }
        }
        let closure_group = ClosureGroup___99 {
            actual___1163: actual___1163.clone()
        };
        let fn__4637 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4637())
        };
        test___88.assert(t___4675, fn__4637.clone());
        let mut t___4677: SqlBuilder = SqlBuilder::new();
        t___4677.append_safe("v IN (");
        t___4677.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___4677.append_safe(")");
        let actual___1166: std::sync::Arc<String> = t___4677.accumulated().to_string();
        let mut t___4684: bool = Some(actual___1166.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___100 {
            actual___1166: std::sync::Arc<String>
        }
        impl ClosureGroup___100 {
            fn fn__4636(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___1166.clone()));
            }
        }
        let closure_group = ClosureGroup___100 {
            actual___1166: actual___1166.clone()
        };
        let fn__4636 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4636())
        };
        test___88.assert(t___4684, fn__4636.clone());
        'ok___5867: {
            'orelse___1248: {
                t___2417 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___1248
                };
                t___2418 = t___2417.clone();
                break 'ok___5867;
            }
            t___2418 = panic!();
        }
        'ok___5868: {
            'orelse___1249: {
                t___2419 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1249
                };
                t___2420 = t___2419.clone();
                break 'ok___5868;
            }
            t___2420 = panic!();
        }
        let dates__927: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___2418.clone(), t___2420.clone()]);
        let mut t___4686: SqlBuilder = SqlBuilder::new();
        t___4686.append_safe("v IN (");
        t___4686.append_date_list(temper_core::ToListed::to_listed(dates__927.clone()));
        t___4686.append_safe(")");
        let actual___1169: std::sync::Arc<String> = t___4686.accumulated().to_string();
        let mut t___4693: bool = Some(actual___1169.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___101 {
            actual___1169: std::sync::Arc<String>
        }
        impl ClosureGroup___101 {
            fn fn__4635(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___1169.clone()));
            }
        }
        let closure_group = ClosureGroup___101 {
            actual___1169: actual___1169.clone()
        };
        let fn__4635 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4635())
        };
        test___88.assert(t___4693, fn__4635.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__1172() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let nan__929: f64;
        nan__929 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___4627: SqlBuilder = SqlBuilder::new();
        t___4627.append_safe("v = ");
        t___4627.append_float64(nan__929);
        let actual___1173: std::sync::Arc<String> = t___4627.accumulated().to_string();
        let mut t___4633: bool = Some(actual___1173.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___102 {
            actual___1173: std::sync::Arc<String>
        }
        impl ClosureGroup___102 {
            fn fn__4626(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___1173.clone()));
            }
        }
        let closure_group = ClosureGroup___102 {
            actual___1173: actual___1173.clone()
        };
        let fn__4626 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4626())
        };
        test___89.assert(t___4633, fn__4626.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__1176() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___90 = temper_std::testing::Test::new();
        let inf__931: f64;
        inf__931 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___4618: SqlBuilder = SqlBuilder::new();
        t___4618.append_safe("v = ");
        t___4618.append_float64(inf__931);
        let actual___1177: std::sync::Arc<String> = t___4618.accumulated().to_string();
        let mut t___4624: bool = Some(actual___1177.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___103 {
            actual___1177: std::sync::Arc<String>
        }
        impl ClosureGroup___103 {
            fn fn__4617(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___1177.clone()));
            }
        }
        let closure_group = ClosureGroup___103 {
            actual___1177: actual___1177.clone()
        };
        let fn__4617 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4617())
        };
        test___90.assert(t___4624, fn__4617.clone());
        test___90.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__1180() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___91 = temper_std::testing::Test::new();
        let ninf__933: f64;
        ninf__933 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___4609: SqlBuilder = SqlBuilder::new();
        t___4609.append_safe("v = ");
        t___4609.append_float64(ninf__933);
        let actual___1181: std::sync::Arc<String> = t___4609.accumulated().to_string();
        let mut t___4615: bool = Some(actual___1181.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___104 {
            actual___1181: std::sync::Arc<String>
        }
        impl ClosureGroup___104 {
            fn fn__4608(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___1181.clone()));
            }
        }
        let closure_group = ClosureGroup___104 {
            actual___1181: actual___1181.clone()
        };
        let fn__4608 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4608())
        };
        test___91.assert(t___4615, fn__4608.clone());
        test___91.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__1184() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___92 = temper_std::testing::Test::new();
        let mut t___4584: SqlBuilder = SqlBuilder::new();
        t___4584.append_safe("v = ");
        t___4584.append_float64(3.14f64);
        let actual___1185: std::sync::Arc<String> = t___4584.accumulated().to_string();
        let mut t___4590: bool = Some(actual___1185.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___105 {
            actual___1185: std::sync::Arc<String>
        }
        impl ClosureGroup___105 {
            fn fn__4583(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___1185.clone()));
            }
        }
        let closure_group = ClosureGroup___105 {
            actual___1185: actual___1185.clone()
        };
        let fn__4583 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4583())
        };
        test___92.assert(t___4590, fn__4583.clone());
        let mut t___4592: SqlBuilder = SqlBuilder::new();
        t___4592.append_safe("v = ");
        t___4592.append_float64(0.0f64);
        let actual___1188: std::sync::Arc<String> = t___4592.accumulated().to_string();
        let mut t___4598: bool = Some(actual___1188.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___106 {
            actual___1188: std::sync::Arc<String>
        }
        impl ClosureGroup___106 {
            fn fn__4582(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___1188.clone()));
            }
        }
        let closure_group = ClosureGroup___106 {
            actual___1188: actual___1188.clone()
        };
        let fn__4582 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4582())
        };
        test___92.assert(t___4598, fn__4582.clone());
        let mut t___4600: SqlBuilder = SqlBuilder::new();
        t___4600.append_safe("v = ");
        t___4600.append_float64(-42.5f64);
        let actual___1191: std::sync::Arc<String> = t___4600.accumulated().to_string();
        let mut t___4606: bool = Some(actual___1191.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___107 {
            actual___1191: std::sync::Arc<String>
        }
        impl ClosureGroup___107 {
            fn fn__4581(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___1191.clone()));
            }
        }
        let closure_group = ClosureGroup___107 {
            actual___1191: actual___1191.clone()
        };
        let fn__4581 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4581())
        };
        test___92.assert(t___4606, fn__4581.clone());
        test___92.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__1194() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___93 = temper_std::testing::Test::new();
        let mut t___2313: temper_std::temporal::Date;
        let d__936: temper_std::temporal::Date;
        'ok___5869: {
            'orelse___1250: {
                t___2313 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___1250
                };
                d__936 = t___2313.clone();
                break 'ok___5869;
            }
            d__936 = panic!();
        }
        let mut t___4573: SqlBuilder = SqlBuilder::new();
        t___4573.append_safe("v = ");
        t___4573.append_date(d__936.clone());
        let actual___1195: std::sync::Arc<String> = t___4573.accumulated().to_string();
        let mut t___4579: bool = Some(actual___1195.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___108 {
            actual___1195: std::sync::Arc<String>
        }
        impl ClosureGroup___108 {
            fn fn__4572(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___1195.clone()));
            }
        }
        let closure_group = ClosureGroup___108 {
            actual___1195: actual___1195.clone()
        };
        let fn__4572 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4572())
        };
        test___93.assert(t___4579, fn__4572.clone());
        test___93.soft_fail_to_hard()
    }
    #[test]
    fn nesting__1198() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___94 = temper_std::testing::Test::new();
        let name__938: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___4541: SqlBuilder = SqlBuilder::new();
        t___4541.append_safe("where p.last_name = ");
        t___4541.append_string("Someone");
        let condition__939: SqlFragment = t___4541.accumulated();
        let mut t___4545: SqlBuilder = SqlBuilder::new();
        t___4545.append_safe("select p.id from person p ");
        t___4545.append_fragment(condition__939.clone());
        let actual___1200: std::sync::Arc<String> = t___4545.accumulated().to_string();
        let mut t___4551: bool = Some(actual___1200.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___109 {
            actual___1200: std::sync::Arc<String>
        }
        impl ClosureGroup___109 {
            fn fn__4540(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1200.clone()));
            }
        }
        let closure_group = ClosureGroup___109 {
            actual___1200: actual___1200.clone()
        };
        let fn__4540 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4540())
        };
        test___94.assert(t___4551, fn__4540.clone());
        let mut t___4553: SqlBuilder = SqlBuilder::new();
        t___4553.append_safe("select p.id from person p ");
        t___4553.append_part(SqlPart::new(condition__939.to_source()));
        let actual___1203: std::sync::Arc<String> = t___4553.accumulated().to_string();
        let mut t___4560: bool = Some(actual___1203.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___110 {
            actual___1203: std::sync::Arc<String>
        }
        impl ClosureGroup___110 {
            fn fn__4539(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1203.clone()));
            }
        }
        let closure_group = ClosureGroup___110 {
            actual___1203: actual___1203.clone()
        };
        let fn__4539 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4539())
        };
        test___94.assert(t___4560, fn__4539.clone());
        let parts__940: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___4564: SqlBuilder = SqlBuilder::new();
        t___4564.append_safe("select ");
        t___4564.append_part_list(parts__940.clone());
        let actual___1206: std::sync::Arc<String> = t___4564.accumulated().to_string();
        let mut t___4570: bool = Some(actual___1206.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___111 {
            actual___1206: std::sync::Arc<String>
        }
        impl ClosureGroup___111 {
            fn fn__4538(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___1206.clone()));
            }
        }
        let closure_group = ClosureGroup___111 {
            actual___1206: actual___1206.clone()
        };
        let fn__4538 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4538())
        };
        test___94.assert(t___4570, fn__4538.clone());
        test___94.soft_fail_to_hard()
    }
    use super::*;
}
