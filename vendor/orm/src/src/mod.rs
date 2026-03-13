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
    pub fn new(field__469: impl temper_core::ToArcString, message__470: impl temper_core::ToArcString) -> ChangesetError {
        let field__469 = field__469.to_arc_string();
        let message__470 = message__470.to_arc_string();
        let field;
        let message;
        field = field__469.clone();
        message = message__470.clone();
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
    fn cast(& self, allowedFields__480: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__483: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__486: SafeIdentifier, min__487: i32, max__488: i32) -> Changeset;
    fn validate_int(& self, field__491: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__494: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__497: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__500: SafeIdentifier) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__505: i32) -> temper_core::Result<SqlFragment>;
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
    pub fn cast(& self, allowedFields__521: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__521 = allowedFields__521.to_list();
        let mb__523: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__166: ChangesetImpl, mb__523: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__9583(& self, f__524: SafeIdentifier) {
                let mut t___9581: std::sync::Arc<String>;
                let mut t___9578: std::sync::Arc<String> = f__524.sql_value();
                let val__525: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__166.0.params, t___9578.clone(), std::sync::Arc::new("".to_string()));
                if ! val__525.is_empty() {
                    t___9581 = f__524.sql_value();
                    temper_core::MapBuilder::set( & self.mb__523, t___9581.clone(), val__525.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__166: self.clone(), mb__523: mb__523.clone()
        };
        let fn__9583 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__524: SafeIdentifier | closure_group.fn__9583(f__524))
        };
        temper_core::listed::list_for_each( & allowedFields__521, & ( * fn__9583.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__523), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__527: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__527 = fields__527.to_list();
        let return__284: Changeset;
        let mut t___9576: temper_core::List<ChangesetError>;
        let mut t___5480: TableDef;
        let mut t___5481: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___5482: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__528: {
            if ! self.0.is_valid {
                return__284 = Changeset::new(self.clone());
                break 'fn__528;
            }
            let eb__529: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__530: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__167: ChangesetImpl, eb__529: temper_core::ListBuilder<ChangesetError>, valid__530: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__9572(& self, f__531: SafeIdentifier) {
                    let mut t___9570: ChangesetError;
                    let mut t___9567: std::sync::Arc<String> = f__531.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__167.0.changes, t___9567.clone()) {
                        t___9570 = ChangesetError::new(f__531.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__529, t___9570.clone(), None);
                        {
                            * self.valid__530.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__167: self.clone(), eb__529: eb__529.clone(), valid__530: valid__530.clone()
            };
            let fn__9572 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__531: SafeIdentifier | closure_group.fn__9572(f__531))
            };
            temper_core::listed::list_for_each( & fields__527, & ( * fn__9572.clone()));
            t___5480 = self.0.table_def.clone();
            t___5481 = self.0.params.clone();
            t___5482 = self.0.changes.clone();
            t___9576 = temper_core::ListedTrait::to_list( & eb__529);
            return__284 = Changeset::new(ChangesetImpl::new(t___5480.clone(), t___5481.clone(), t___5482.clone(), t___9576.clone(), temper_core::read_locked( & valid__530)));
        }
        return return__284.clone();
    }
    pub fn validate_length(& self, field__533: SafeIdentifier, min__534: i32, max__535: i32) -> Changeset {
        let return__285: Changeset;
        let mut t___9554: std::sync::Arc<String>;
        let mut t___9565: temper_core::List<ChangesetError>;
        let mut t___5463: bool;
        let mut t___5469: TableDef;
        let mut t___5470: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___5471: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__536: {
            if ! self.0.is_valid {
                return__285 = Changeset::new(self.clone());
                break 'fn__536;
            }
            t___9554 = field__533.sql_value();
            let val__537: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___9554.clone(), std::sync::Arc::new("".to_string()));
            let len__538: i32 = temper_core::string::count_between( & val__537, 0usize, val__537.len());
            if Some(len__538) < Some(min__534) {
                t___5463 = true;
            } else {
                t___5463 = Some(len__538) > Some(max__535);
            }
            if t___5463 {
                let msg__539: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__534, max__535));
                let eb__540: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__540, ChangesetError::new(field__533.sql_value(), msg__539.clone()), None);
                t___5469 = self.0.table_def.clone();
                t___5470 = self.0.params.clone();
                t___5471 = self.0.changes.clone();
                t___9565 = temper_core::ListedTrait::to_list( & eb__540);
                return__285 = Changeset::new(ChangesetImpl::new(t___5469.clone(), t___5470.clone(), t___5471.clone(), t___9565.clone(), false));
                break 'fn__536;
            }
            return__285 = Changeset::new(self.clone());
        }
        return return__285.clone();
    }
    pub fn validate_int(& self, field__542: SafeIdentifier) -> Changeset {
        let return__286: Changeset;
        let mut t___9545: std::sync::Arc<String>;
        let mut t___9552: temper_core::List<ChangesetError>;
        let mut t___5454: TableDef;
        let mut t___5455: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___5456: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__543: {
            if ! self.0.is_valid {
                return__286 = Changeset::new(self.clone());
                break 'fn__543;
            }
            t___9545 = field__542.sql_value();
            let val__544: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___9545.clone(), std::sync::Arc::new("".to_string()));
            if val__544.is_empty() {
                return__286 = Changeset::new(self.clone());
                break 'fn__543;
            }
            let parseOk__545: bool;
            'ok___9688: {
                'orelse___1696: {
                    match temper_core::string::to_int( & val__544, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1696
                    };
                    parseOk__545 = true;
                    break 'ok___9688;
                }
                parseOk__545 = false;
            }
            if ! parseOk__545 {
                let eb__546: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__546, ChangesetError::new(field__542.sql_value(), "must be an integer"), None);
                t___5454 = self.0.table_def.clone();
                t___5455 = self.0.params.clone();
                t___5456 = self.0.changes.clone();
                t___9552 = temper_core::ListedTrait::to_list( & eb__546);
                return__286 = Changeset::new(ChangesetImpl::new(t___5454.clone(), t___5455.clone(), t___5456.clone(), t___9552.clone(), false));
                break 'fn__543;
            }
            return__286 = Changeset::new(self.clone());
        }
        return return__286.clone();
    }
    pub fn validate_int64(& self, field__548: SafeIdentifier) -> Changeset {
        let return__287: Changeset;
        let mut t___9536: std::sync::Arc<String>;
        let mut t___9543: temper_core::List<ChangesetError>;
        let mut t___5441: TableDef;
        let mut t___5442: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___5443: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__549: {
            if ! self.0.is_valid {
                return__287 = Changeset::new(self.clone());
                break 'fn__549;
            }
            t___9536 = field__548.sql_value();
            let val__550: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___9536.clone(), std::sync::Arc::new("".to_string()));
            if val__550.is_empty() {
                return__287 = Changeset::new(self.clone());
                break 'fn__549;
            }
            let parseOk__551: bool;
            'ok___9690: {
                'orelse___1697: {
                    match temper_core::string::to_int64( & val__550, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1697
                    };
                    parseOk__551 = true;
                    break 'ok___9690;
                }
                parseOk__551 = false;
            }
            if ! parseOk__551 {
                let eb__552: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__552, ChangesetError::new(field__548.sql_value(), "must be a 64-bit integer"), None);
                t___5441 = self.0.table_def.clone();
                t___5442 = self.0.params.clone();
                t___5443 = self.0.changes.clone();
                t___9543 = temper_core::ListedTrait::to_list( & eb__552);
                return__287 = Changeset::new(ChangesetImpl::new(t___5441.clone(), t___5442.clone(), t___5443.clone(), t___9543.clone(), false));
                break 'fn__549;
            }
            return__287 = Changeset::new(self.clone());
        }
        return return__287.clone();
    }
    pub fn validate_float(& self, field__554: SafeIdentifier) -> Changeset {
        let return__288: Changeset;
        let mut t___9527: std::sync::Arc<String>;
        let mut t___9534: temper_core::List<ChangesetError>;
        let mut t___5428: TableDef;
        let mut t___5429: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___5430: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__555: {
            if ! self.0.is_valid {
                return__288 = Changeset::new(self.clone());
                break 'fn__555;
            }
            t___9527 = field__554.sql_value();
            let val__556: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___9527.clone(), std::sync::Arc::new("".to_string()));
            if val__556.is_empty() {
                return__288 = Changeset::new(self.clone());
                break 'fn__555;
            }
            let parseOk__557: bool;
            'ok___9692: {
                'orelse___1698: {
                    match temper_core::string::to_float64( & val__556) {
                        Ok(x) => x,
                        _ => break 'orelse___1698
                    };
                    parseOk__557 = true;
                    break 'ok___9692;
                }
                parseOk__557 = false;
            }
            if ! parseOk__557 {
                let eb__558: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__558, ChangesetError::new(field__554.sql_value(), "must be a number"), None);
                t___5428 = self.0.table_def.clone();
                t___5429 = self.0.params.clone();
                t___5430 = self.0.changes.clone();
                t___9534 = temper_core::ListedTrait::to_list( & eb__558);
                return__288 = Changeset::new(ChangesetImpl::new(t___5428.clone(), t___5429.clone(), t___5430.clone(), t___9534.clone(), false));
                break 'fn__555;
            }
            return__288 = Changeset::new(self.clone());
        }
        return return__288.clone();
    }
    pub fn validate_bool(& self, field__560: SafeIdentifier) -> Changeset {
        let return__289: Changeset;
        let mut t___9518: std::sync::Arc<String>;
        let mut t___9525: temper_core::List<ChangesetError>;
        let mut t___5403: bool;
        let mut t___5404: bool;
        let mut t___5406: bool;
        let mut t___5407: bool;
        let mut t___5409: bool;
        let mut t___5415: TableDef;
        let mut t___5416: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___5417: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__561: {
            if ! self.0.is_valid {
                return__289 = Changeset::new(self.clone());
                break 'fn__561;
            }
            t___9518 = field__560.sql_value();
            let val__562: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___9518.clone(), std::sync::Arc::new("".to_string()));
            if val__562.is_empty() {
                return__289 = Changeset::new(self.clone());
                break 'fn__561;
            }
            let isTrue__563: bool;
            if Some(val__562.as_str()) == Some("true") {
                isTrue__563 = true;
            } else {
                if Some(val__562.as_str()) == Some("1") {
                    t___5404 = true;
                } else {
                    if Some(val__562.as_str()) == Some("yes") {
                        t___5403 = true;
                    } else {
                        t___5403 = Some(val__562.as_str()) == Some("on");
                    }
                    t___5404 = t___5403;
                }
                isTrue__563 = t___5404;
            }
            let isFalse__564: bool;
            if Some(val__562.as_str()) == Some("false") {
                isFalse__564 = true;
            } else {
                if Some(val__562.as_str()) == Some("0") {
                    t___5407 = true;
                } else {
                    if Some(val__562.as_str()) == Some("no") {
                        t___5406 = true;
                    } else {
                        t___5406 = Some(val__562.as_str()) == Some("off");
                    }
                    t___5407 = t___5406;
                }
                isFalse__564 = t___5407;
            }
            if ! isTrue__563 {
                t___5409 = ! isFalse__564;
            } else {
                t___5409 = false;
            }
            if t___5409 {
                let eb__565: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__565, ChangesetError::new(field__560.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___5415 = self.0.table_def.clone();
                t___5416 = self.0.params.clone();
                t___5417 = self.0.changes.clone();
                t___9525 = temper_core::ListedTrait::to_list( & eb__565);
                return__289 = Changeset::new(ChangesetImpl::new(t___5415.clone(), t___5416.clone(), t___5417.clone(), t___9525.clone(), false));
                break 'fn__561;
            }
            return__289 = Changeset::new(self.clone());
        }
        return return__289.clone();
    }
    fn parse_bool_sql_part(& self, val__567: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__567 = val__567.to_arc_string();
        let return__290: SqlBoolean;
        let mut t___5392: bool;
        let mut t___5393: bool;
        let mut t___5394: bool;
        let mut t___5396: bool;
        let mut t___5397: bool;
        let mut t___5398: bool;
        'fn__568: {
            if Some(val__567.as_str()) == Some("true") {
                t___5394 = true;
            } else {
                if Some(val__567.as_str()) == Some("1") {
                    t___5393 = true;
                } else {
                    if Some(val__567.as_str()) == Some("yes") {
                        t___5392 = true;
                    } else {
                        t___5392 = Some(val__567.as_str()) == Some("on");
                    }
                    t___5393 = t___5392;
                }
                t___5394 = t___5393;
            }
            if t___5394 {
                return__290 = SqlBoolean::new(true);
                break 'fn__568;
            }
            if Some(val__567.as_str()) == Some("false") {
                t___5398 = true;
            } else {
                if Some(val__567.as_str()) == Some("0") {
                    t___5397 = true;
                } else {
                    if Some(val__567.as_str()) == Some("no") {
                        t___5396 = true;
                    } else {
                        t___5396 = Some(val__567.as_str()) == Some("off");
                    }
                    t___5397 = t___5396;
                }
                t___5398 = t___5397;
            }
            if t___5398 {
                return__290 = SqlBoolean::new(false);
                break 'fn__568;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__290.clone());
    }
    fn value_to_sql_part(& self, fieldDef__570: FieldDef, val__571: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__571 = val__571.to_arc_string();
        let return__291: SqlPart;
        let mut t___5379: i32;
        let mut t___5382: i64;
        let mut t___5385: f64;
        let mut t___5390: temper_std::temporal::Date;
        'fn__572: {
            let ft__573: FieldType = fieldDef__570.field_type();
            if temper_core::is::<StringField>(ft__573.clone()) {
                return__291 = SqlPart::new(SqlString::new(val__571.clone()));
                break 'fn__572;
            }
            if temper_core::is::<IntField>(ft__573.clone()) {
                t___5379 = temper_core::string::to_int( & val__571, None) ? ;
                return__291 = SqlPart::new(SqlInt32::new(t___5379));
                break 'fn__572;
            }
            if temper_core::is::<Int64Field>(ft__573.clone()) {
                t___5382 = temper_core::string::to_int64( & val__571, None) ? ;
                return__291 = SqlPart::new(SqlInt64::new(t___5382));
                break 'fn__572;
            }
            if temper_core::is::<FloatField>(ft__573.clone()) {
                t___5385 = temper_core::string::to_float64( & val__571) ? ;
                return__291 = SqlPart::new(SqlFloat64::new(t___5385));
                break 'fn__572;
            }
            if temper_core::is::<BoolField>(ft__573.clone()) {
                return__291 = SqlPart::new(self.parse_bool_sql_part(val__571.clone()) ? );
                break 'fn__572;
            }
            if temper_core::is::<DateField>(ft__573.clone()) {
                t___5390 = temper_std::temporal::Date::from_iso_string(val__571.clone()) ? ;
                return__291 = SqlPart::new(SqlDate::new(t___5390.clone()));
                break 'fn__572;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__291.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___9466: i32;
        let mut t___9471: std::sync::Arc<String>;
        let mut t___9472: bool;
        let mut t___9477: i32;
        let mut t___9479: std::sync::Arc<String>;
        let mut t___9483: std::sync::Arc<String>;
        let mut t___9498: i32;
        let mut t___5343: bool;
        let mut t___5351: FieldDef;
        let mut t___5356: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__576: i32 = 0;
        'loop___9754: loop {
            t___9466 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__576) < Some(t___9466)) {
                break;
            }
            let f__577: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__576);
            if ! f__577.nullable() {
                t___9471 = f__577.name().sql_value();
                t___9472 = temper_core::MappedTrait::has( & self.0.changes, t___9471.clone());
                t___5343 = ! t___9472;
            } else {
                t___5343 = false;
            }
            if t___5343 {
                return Err(temper_core::Error::new());
            }
            i__576 = i__576.wrapping_add(1);
        }
        let pairs__578: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__578)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__579: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__580: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__581: i32 = 0;
        'loop___9755: loop {
            t___9477 = temper_core::ListedTrait::len( & pairs__578);
            if ! (Some(i__581) < Some(t___9477)) {
                break;
            }
            let pair__582: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__578, i__581);
            t___9479 = pair__582.key();
            t___5351 = self.0.table_def.field(t___9479.clone()) ? ;
            let fd__583: FieldDef = t___5351.clone();
            temper_core::listed::add( & colNames__579, fd__583.name().sql_value(), None);
            t___9483 = pair__582.value();
            t___5356 = self.value_to_sql_part(fd__583.clone(), t___9483.clone()) ? ;
            temper_core::listed::add( & valParts__580, t___5356.clone(), None);
            i__581 = i__581.wrapping_add(1);
        }
        let b__584: SqlBuilder = SqlBuilder::new();
        b__584.append_safe("INSERT INTO ");
        b__584.append_safe(self.0.table_def.table_name().sql_value());
        b__584.append_safe(" (");
        let mut t___9491: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__579);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__9464(& self, c__585: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__585 = c__585.to_arc_string();
                return c__585.clone();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__9464 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__585: std::sync::Arc<String> | closure_group.fn__9464(c__585))
        };
        b__584.append_safe(temper_core::listed::join( & t___9491, std::sync::Arc::new(", ".to_string()), & ( * fn__9464.clone())));
        b__584.append_safe(") VALUES (");
        b__584.append_part(temper_core::ListedTrait::get( & valParts__580, 0));
        let mut j__586: i32 = 1;
        'loop___9756: loop {
            t___9498 = temper_core::ListedTrait::len( & valParts__580);
            if ! (Some(j__586) < Some(t___9498)) {
                break;
            }
            b__584.append_safe(", ");
            b__584.append_part(temper_core::ListedTrait::get( & valParts__580, j__586));
            j__586 = j__586.wrapping_add(1);
        }
        b__584.append_safe(")");
        return Ok(b__584.accumulated());
    }
    pub fn to_update_sql(& self, id__588: i32) -> temper_core::Result<SqlFragment> {
        let mut t___9451: i32;
        let mut t___9454: std::sync::Arc<String>;
        let mut t___9459: std::sync::Arc<String>;
        let mut t___5324: FieldDef;
        let mut t___5330: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__590: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__590)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__591: SqlBuilder = SqlBuilder::new();
        b__591.append_safe("UPDATE ");
        b__591.append_safe(self.0.table_def.table_name().sql_value());
        b__591.append_safe(" SET ");
        let mut i__592: i32 = 0;
        'loop___9757: loop {
            t___9451 = temper_core::ListedTrait::len( & pairs__590);
            if ! (Some(i__592) < Some(t___9451)) {
                break;
            }
            if Some(i__592) > Some(0) {
                b__591.append_safe(", ");
            }
            let pair__593: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__590, i__592);
            t___9454 = pair__593.key();
            t___5324 = self.0.table_def.field(t___9454.clone()) ? ;
            let fd__594: FieldDef = t___5324.clone();
            b__591.append_safe(fd__594.name().sql_value());
            b__591.append_safe(" = ");
            t___9459 = pair__593.value();
            t___5330 = self.value_to_sql_part(fd__594.clone(), t___9459.clone()) ? ;
            b__591.append_part(t___5330.clone());
            i__592 = i__592.wrapping_add(1);
        }
        b__591.append_safe(" WHERE id = ");
        b__591.append_int32(id__588);
        return Ok(b__591.accumulated());
    }
    pub fn new(_tableDef__596: TableDef, _params__597: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__598: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__599: impl temper_core::ToList<ChangesetError>, _isValid__600: bool) -> ChangesetImpl {
        let _errors__599 = _errors__599.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__596.clone();
        params = _params__597.clone();
        changes = _changes__598.clone();
        errors = _errors__599.clone();
        is_valid = _isValid__600;
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
    fn cast(& self, allowedFields__521: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__521)
    }
    fn validate_required(& self, fields__527: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__527)
    }
    fn validate_length(& self, field__533: SafeIdentifier, min__534: i32, max__535: i32) -> Changeset {
        self.validate_length(field__533, min__534, max__535)
    }
    fn validate_int(& self, field__542: SafeIdentifier) -> Changeset {
        self.validate_int(field__542)
    }
    fn validate_int64(& self, field__548: SafeIdentifier) -> Changeset {
        self.validate_int64(field__548)
    }
    fn validate_float(& self, field__554: SafeIdentifier) -> Changeset {
        self.validate_float(field__554)
    }
    fn validate_bool(& self, field__560: SafeIdentifier) -> Changeset {
        self.validate_bool(field__560)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__588: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__588)
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
    pub fn new(joinType__713: JoinType, table__714: SafeIdentifier, onCondition__715: SqlFragment) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__713.clone();
        table = table__714.clone();
        on_condition = onCondition__715.clone();
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
    pub fn new(field__719: SafeIdentifier, ascending__720: bool) -> OrderClause {
        let field;
        let ascending;
        field = field__719.clone();
        ascending = ascending__720;
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
    pub fn new(_condition__731: SqlFragment) -> AndCondition {
        let condition;
        condition = _condition__731.clone();
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
    pub fn new(_condition__738: SqlFragment) -> OrCondition {
        let condition;
        condition = _condition__738.clone();
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
    table_name: SafeIdentifier, conditions: temper_core::List<WhereClause>, selected_fields: temper_core::List<SafeIdentifier>, order_clauses: temper_core::List<OrderClause>, limit_val: Option<i32>, offset_val: Option<i32>, join_clauses: temper_core::List<JoinClause>, group_by_fields: temper_core::List<SafeIdentifier>, having_conditions: temper_core::List<WhereClause>, is_distinct: bool, select_exprs: temper_core::List<SqlFragment>
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<QueryStruct>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<WhereClause>, pub selected_fields: temper_core::List<SafeIdentifier>, pub order_clauses: temper_core::List<OrderClause>, pub limit_val: Option<i32>, pub offset_val: Option<i32>, pub join_clauses: temper_core::List<JoinClause>, pub group_by_fields: temper_core::List<SafeIdentifier>, pub having_conditions: temper_core::List<WhereClause>, pub is_distinct: bool, pub select_exprs: temper_core::List<SqlFragment>
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.table_name, self.conditions, self.selected_fields, self.order_clauses, self.limit_val, self.offset_val, self.join_clauses, self.group_by_fields, self.having_conditions, self.is_distinct, self.select_exprs)
    }
}
impl Query {
    pub fn r#where(& self, condition__751: SqlFragment) -> Query {
        let nb__753: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__753, WhereClause::new(AndCondition::new(condition__751.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__753), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn or_where(& self, condition__755: SqlFragment) -> Query {
        let nb__757: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__757, WhereClause::new(OrCondition::new(condition__755.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__757), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn where_null(& self, field__759: SafeIdentifier) -> Query {
        let b__761: SqlBuilder = SqlBuilder::new();
        b__761.append_safe(field__759.sql_value());
        b__761.append_safe(" IS NULL");
        let mut t___9050: SqlFragment = b__761.accumulated();
        return self.r#where(t___9050.clone());
    }
    pub fn where_not_null(& self, field__763: SafeIdentifier) -> Query {
        let b__765: SqlBuilder = SqlBuilder::new();
        b__765.append_safe(field__763.sql_value());
        b__765.append_safe(" IS NOT NULL");
        let mut t___9044: SqlFragment = b__765.accumulated();
        return self.r#where(t___9044.clone());
    }
    pub fn where_in(& self, field__767: SafeIdentifier, values__768: impl temper_core::ToList<SqlPart>) -> Query {
        let values__768 = values__768.to_list();
        let return__343: Query;
        let mut t___9025: SqlFragment;
        let mut t___9033: i32;
        let mut t___9038: SqlFragment;
        'fn__769: {
            if temper_core::ListedTrait::is_empty( & values__768) {
                let b__770: SqlBuilder = SqlBuilder::new();
                b__770.append_safe("1 = 0");
                t___9025 = b__770.accumulated();
                return__343 = self.r#where(t___9025.clone());
                break 'fn__769;
            }
            let b__771: SqlBuilder = SqlBuilder::new();
            b__771.append_safe(field__767.sql_value());
            b__771.append_safe(" IN (");
            b__771.append_part(temper_core::ListedTrait::get( & values__768, 0));
            let mut i__772: i32 = 1;
            'loop___9762: loop {
                t___9033 = temper_core::ListedTrait::len( & values__768);
                if ! (Some(i__772) < Some(t___9033)) {
                    break;
                }
                b__771.append_safe(", ");
                b__771.append_part(temper_core::ListedTrait::get( & values__768, i__772));
                i__772 = i__772.wrapping_add(1);
            }
            b__771.append_safe(")");
            t___9038 = b__771.accumulated();
            return__343 = self.r#where(t___9038.clone());
        }
        return return__343.clone();
    }
    pub fn where_in_subquery(& self, field__774: SafeIdentifier, sub__775: Query) -> Query {
        let b__777: SqlBuilder = SqlBuilder::new();
        b__777.append_safe(field__774.sql_value());
        b__777.append_safe(" IN (");
        b__777.append_fragment(sub__775.to_sql());
        b__777.append_safe(")");
        let mut t___9020: SqlFragment = b__777.accumulated();
        return self.r#where(t___9020.clone());
    }
    pub fn where_not(& self, condition__779: SqlFragment) -> Query {
        let b__781: SqlBuilder = SqlBuilder::new();
        b__781.append_safe("NOT (");
        b__781.append_fragment(condition__779.clone());
        b__781.append_safe(")");
        let mut t___9011: SqlFragment = b__781.accumulated();
        return self.r#where(t___9011.clone());
    }
    pub fn where_between(& self, field__783: SafeIdentifier, low__784: SqlPart, high__785: SqlPart) -> Query {
        let b__787: SqlBuilder = SqlBuilder::new();
        b__787.append_safe(field__783.sql_value());
        b__787.append_safe(" BETWEEN ");
        b__787.append_part(low__784.clone());
        b__787.append_safe(" AND ");
        b__787.append_part(high__785.clone());
        let mut t___9005: SqlFragment = b__787.accumulated();
        return self.r#where(t___9005.clone());
    }
    pub fn where_like(& self, field__789: SafeIdentifier, pattern__790: impl temper_core::ToArcString) -> Query {
        let pattern__790 = pattern__790.to_arc_string();
        let b__792: SqlBuilder = SqlBuilder::new();
        b__792.append_safe(field__789.sql_value());
        b__792.append_safe(" LIKE ");
        b__792.append_string(pattern__790.clone());
        let mut t___8996: SqlFragment = b__792.accumulated();
        return self.r#where(t___8996.clone());
    }
    pub fn where_i_like(& self, field__794: SafeIdentifier, pattern__795: impl temper_core::ToArcString) -> Query {
        let pattern__795 = pattern__795.to_arc_string();
        let b__797: SqlBuilder = SqlBuilder::new();
        b__797.append_safe(field__794.sql_value());
        b__797.append_safe(" ILIKE ");
        b__797.append_string(pattern__795.clone());
        let mut t___8989: SqlFragment = b__797.accumulated();
        return self.r#where(t___8989.clone());
    }
    pub fn select(& self, fields__799: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__799 = fields__799.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__799.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn select_expr(& self, exprs__802: impl temper_core::ToList<SqlFragment>) -> Query {
        let exprs__802 = exprs__802.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, exprs__802.clone());
    }
    pub fn order_by(& self, field__805: SafeIdentifier, ascending__806: bool) -> Query {
        let nb__808: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__808, OrderClause::new(field__805.clone(), ascending__806), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__808), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn limit(& self, n__810: i32) -> temper_core::Result<Query> {
        if Some(n__810) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__810), self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone()));
    }
    pub fn offset(& self, n__813: i32) -> temper_core::Result<Query> {
        if Some(n__813) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__813), self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone()));
    }
    pub fn join(& self, joinType__816: JoinType, table__817: SafeIdentifier, onCondition__818: SqlFragment) -> Query {
        let nb__820: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__820, JoinClause::new(joinType__816.clone(), table__817.clone(), onCondition__818.clone()), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__820), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn inner_join(& self, table__822: SafeIdentifier, onCondition__823: SqlFragment) -> Query {
        let mut t___8959: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___8959.clone()), table__822.clone(), onCondition__823.clone());
    }
    pub fn left_join(& self, table__826: SafeIdentifier, onCondition__827: SqlFragment) -> Query {
        let mut t___8957: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___8957.clone()), table__826.clone(), onCondition__827.clone());
    }
    pub fn right_join(& self, table__830: SafeIdentifier, onCondition__831: SqlFragment) -> Query {
        let mut t___8955: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___8955.clone()), table__830.clone(), onCondition__831.clone());
    }
    pub fn full_join(& self, table__834: SafeIdentifier, onCondition__835: SqlFragment) -> Query {
        let mut t___8953: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___8953.clone()), table__834.clone(), onCondition__835.clone());
    }
    pub fn group_by(& self, field__838: SafeIdentifier) -> Query {
        let nb__840: temper_core::ListBuilder<SafeIdentifier> = temper_core::ListedTrait::to_list_builder( & self.0.group_by_fields);
        temper_core::listed::add( & nb__840, field__838.clone(), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), temper_core::ListedTrait::to_list( & nb__840), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn having(& self, condition__842: SqlFragment) -> Query {
        let nb__844: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__844, WhereClause::new(AndCondition::new(condition__842.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__844), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn or_having(& self, condition__846: SqlFragment) -> Query {
        let nb__848: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__848, WhereClause::new(OrCondition::new(condition__846.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__848), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn distinct(& self) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), true, self.0.select_exprs.clone());
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___8859: i32;
        let mut t___8878: i32;
        let mut t___8897: i32;
        let b__853: SqlBuilder = SqlBuilder::new();
        if self.0.is_distinct {
            b__853.append_safe("SELECT DISTINCT ");
        } else {
            b__853.append_safe("SELECT ");
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.select_exprs) {
            b__853.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, 0));
            let mut i__854: i32 = 1;
            'loop___9781: loop {
                t___8859 = temper_core::ListedTrait::len( & self.0.select_exprs);
                if ! (Some(i__854) < Some(t___8859)) {
                    break;
                }
                b__853.append_safe(", ");
                b__853.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, i__854));
                i__854 = i__854.wrapping_add(1);
            }
        } else {
            if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
                b__853.append_safe("*");
            } else {
                #[derive(Clone)]
                struct ClosureGroup___4 {}
                impl ClosureGroup___4 {
                    fn fn__8852(& self, f__855: SafeIdentifier) -> std::sync::Arc<String> {
                        return f__855.sql_value();
                    }
                }
                let closure_group = ClosureGroup___4 {};
                let fn__8852 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | f__855: SafeIdentifier | closure_group.fn__8852(f__855))
                };
                b__853.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__8852.clone())));
            }
        }
        b__853.append_safe(" FROM ");
        b__853.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___5 {
            b__853: SqlBuilder
        }
        impl ClosureGroup___5 {
            fn fn__8851(& self, jc__856: JoinClause) {
                self.b__853.append_safe(" ");
                let mut t___8839: std::sync::Arc<String> = jc__856.join_type().keyword();
                self.b__853.append_safe(t___8839.clone());
                self.b__853.append_safe(" ");
                let mut t___8843: std::sync::Arc<String> = jc__856.table().sql_value();
                self.b__853.append_safe(t___8843.clone());
                self.b__853.append_safe(" ON ");
                let mut t___8846: SqlFragment = jc__856.on_condition();
                self.b__853.append_fragment(t___8846.clone());
            }
        }
        let closure_group = ClosureGroup___5 {
            b__853: b__853.clone()
        };
        let fn__8851 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__856: JoinClause | closure_group.fn__8851(jc__856))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__8851.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__853.append_safe(" WHERE ");
            b__853.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__857: i32 = 1;
            'loop___9782: loop {
                t___8878 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__857) < Some(t___8878)) {
                    break;
                }
                b__853.append_safe(" ");
                b__853.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__857).keyword());
                b__853.append_safe(" ");
                b__853.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__857).condition());
                i__857 = i__857.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__853.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___6 {}
            impl ClosureGroup___6 {
                fn fn__8850(& self, f__858: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__858.sql_value();
                }
            }
            let closure_group = ClosureGroup___6 {};
            let fn__8850 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__858: SafeIdentifier | closure_group.fn__8850(f__858))
            };
            b__853.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__8850.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__853.append_safe(" HAVING ");
            b__853.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__859: i32 = 1;
            'loop___9783: loop {
                t___8897 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__859) < Some(t___8897)) {
                    break;
                }
                b__853.append_safe(" ");
                b__853.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__859).keyword());
                b__853.append_safe(" ");
                b__853.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__859).condition());
                i__859 = i__859.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__853.append_safe(" ORDER BY ");
            let mut first__860: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___7 {
                first__860: std::sync::Arc<std::sync::RwLock<bool>>, b__853: SqlBuilder
            }
            impl ClosureGroup___7 {
                fn fn__8849(& self, oc__861: OrderClause) {
                    let mut t___4769: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__860) {
                        self.b__853.append_safe(", ");
                    }
                    {
                        * self.first__860.write().unwrap() = false;
                    }
                    let mut t___8832: std::sync::Arc<String> = oc__861.field().sql_value();
                    self.b__853.append_safe(t___8832.clone());
                    if oc__861.ascending() {
                        t___4769 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___4769 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__853.append_safe(t___4769.clone());
                }
            }
            let closure_group = ClosureGroup___7 {
                first__860: first__860.clone(), b__853: b__853.clone()
            };
            let fn__8849 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | oc__861: OrderClause | closure_group.fn__8849(oc__861))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__8849.clone()));
        }
        let lv__862: Option<i32> = self.0.limit_val;
        if ! lv__862.is_none() {
            let lv___1742: i32 = lv__862.unwrap();
            b__853.append_safe(" LIMIT ");
            b__853.append_int32(lv___1742);
        }
        let ov__863: Option<i32> = self.0.offset_val;
        if ! ov__863.is_none() {
            let ov___1743: i32 = ov__863.unwrap();
            b__853.append_safe(" OFFSET ");
            b__853.append_int32(ov___1743);
        }
        return b__853.accumulated();
    }
    pub fn count_sql(& self) -> SqlFragment {
        let mut t___8801: i32;
        let mut t___8820: i32;
        let b__866: SqlBuilder = SqlBuilder::new();
        b__866.append_safe("SELECT COUNT(*) FROM ");
        b__866.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___8 {
            b__866: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__8789(& self, jc__867: JoinClause) {
                self.b__866.append_safe(" ");
                let mut t___8779: std::sync::Arc<String> = jc__867.join_type().keyword();
                self.b__866.append_safe(t___8779.clone());
                self.b__866.append_safe(" ");
                let mut t___8783: std::sync::Arc<String> = jc__867.table().sql_value();
                self.b__866.append_safe(t___8783.clone());
                self.b__866.append_safe(" ON ");
                let mut t___8786: SqlFragment = jc__867.on_condition();
                self.b__866.append_fragment(t___8786.clone());
            }
        }
        let closure_group = ClosureGroup___8 {
            b__866: b__866.clone()
        };
        let fn__8789 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__867: JoinClause | closure_group.fn__8789(jc__867))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__8789.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__866.append_safe(" WHERE ");
            b__866.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__868: i32 = 1;
            'loop___9786: loop {
                t___8801 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__868) < Some(t___8801)) {
                    break;
                }
                b__866.append_safe(" ");
                b__866.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__868).keyword());
                b__866.append_safe(" ");
                b__866.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__868).condition());
                i__868 = i__868.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__866.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___9 {}
            impl ClosureGroup___9 {
                fn fn__8788(& self, f__869: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__869.sql_value();
                }
            }
            let closure_group = ClosureGroup___9 {};
            let fn__8788 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__869: SafeIdentifier | closure_group.fn__8788(f__869))
            };
            b__866.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__8788.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__866.append_safe(" HAVING ");
            b__866.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__870: i32 = 1;
            'loop___9787: loop {
                t___8820 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__870) < Some(t___8820)) {
                    break;
                }
                b__866.append_safe(" ");
                b__866.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__870).keyword());
                b__866.append_safe(" ");
                b__866.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__870).condition());
                i__870 = i__870.wrapping_add(1);
            }
        }
        return b__866.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__872: i32) -> temper_core::Result<SqlFragment> {
        let return__365: SqlFragment;
        let mut t___4718: Query;
        if Some(defaultLimit__872) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__365 = self.to_sql();
        } else {
            t___4718 = self.limit(defaultLimit__872) ? ;
            return__365 = t___4718.to_sql();
        }
        return Ok(return__365.clone());
    }
    pub fn new(tableName__875: SafeIdentifier, conditions__876: impl temper_core::ToList<WhereClause>, selectedFields__877: impl temper_core::ToList<SafeIdentifier>, orderClauses__878: impl temper_core::ToList<OrderClause>, limitVal__879: Option<i32>, offsetVal__880: Option<i32>, joinClauses__881: impl temper_core::ToList<JoinClause>, groupByFields__882: impl temper_core::ToList<SafeIdentifier>, havingConditions__883: impl temper_core::ToList<WhereClause>, isDistinct__884: bool, selectExprs__885: impl temper_core::ToList<SqlFragment>) -> Query {
        let conditions__876 = conditions__876.to_list();
        let selectedFields__877 = selectedFields__877.to_list();
        let orderClauses__878 = orderClauses__878.to_list();
        let joinClauses__881 = joinClauses__881.to_list();
        let groupByFields__882 = groupByFields__882.to_list();
        let havingConditions__883 = havingConditions__883.to_list();
        let selectExprs__885 = selectExprs__885.to_list();
        let table_name;
        let conditions;
        let selected_fields;
        let order_clauses;
        let limit_val;
        let offset_val;
        let join_clauses;
        let group_by_fields;
        let having_conditions;
        let is_distinct;
        let select_exprs;
        table_name = tableName__875.clone();
        conditions = conditions__876.clone();
        selected_fields = selectedFields__877.clone();
        order_clauses = orderClauses__878.clone();
        limit_val = limitVal__879;
        offset_val = offsetVal__880;
        join_clauses = joinClauses__881.clone();
        group_by_fields = groupByFields__882.clone();
        having_conditions = havingConditions__883.clone();
        is_distinct = isDistinct__884;
        select_exprs = selectExprs__885.clone();
        let selfish = Query(std::sync::Arc::new(QueryStruct {
                    table_name, conditions, selected_fields, order_clauses, limit_val, offset_val, join_clauses, group_by_fields, having_conditions, is_distinct, select_exprs
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
    pub fn group_by_fields(& self) -> temper_core::List<SafeIdentifier> {
        return self.0.group_by_fields.clone();
    }
    pub fn having_conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.having_conditions.clone();
    }
    pub fn is_distinct(& self) -> bool {
        return self.0.is_distinct;
    }
    pub fn select_exprs(& self) -> temper_core::List<SqlFragment> {
        return self.0.select_exprs.clone();
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
    pub fn new(_value__1120: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__1120 = _value__1120.to_arc_string();
        let value;
        value = _value__1120.clone();
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
    pub fn new(name__1138: SafeIdentifier, fieldType__1139: FieldType, nullable__1140: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__1138.clone();
        field_type = fieldType__1139.clone();
        nullable = nullable__1140;
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
    pub fn field(& self, name__1144: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__1144 = name__1144.to_arc_string();
        let return__407: FieldDef;
        'fn__1145: {
            let this__5723: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__5724: i32 = temper_core::ListedTrait::len( & this__5723);
            let mut i__5725: i32 = 0;
            'loop___9792: while Some(i__5725) < Some(n__5724) {
                let el__5726: FieldDef = temper_core::ListedTrait::get( & this__5723, i__5725);
                i__5725 = i__5725.wrapping_add(1);
                let f__1146: FieldDef = el__5726.clone();
                if Some(f__1146.name().sql_value().as_str()) == Some(name__1144.as_str()) {
                    return__407 = f__1146.clone();
                    break 'fn__1145;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__407.clone());
    }
    pub fn new(tableName__1148: SafeIdentifier, fields__1149: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__1149 = fields__1149.to_list();
        let table_name;
        let fields;
        table_name = tableName__1148.clone();
        fields = fields__1149.clone();
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
    pub fn append_safe(& self, sqlSource__1171: impl temper_core::ToArcString) {
        let sqlSource__1171 = sqlSource__1171.to_arc_string();
        let mut t___9641: SqlSource = SqlSource::new(sqlSource__1171.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___9641.clone()), None);
    }
    pub fn append_fragment(& self, fragment__1174: SqlFragment) {
        let mut t___9639: temper_core::List<SqlPart> = fragment__1174.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___9639.clone()), None);
    }
    pub fn append_part(& self, part__1177: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__1177.clone(), None);
    }
    pub fn append_part_list(& self, values__1180: impl temper_core::ToList<SqlPart>) {
        let values__1180 = values__1180.to_list();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__222: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__9635(& self, x__1182: SqlPart) {
                self.this__222.append_part(x__1182.clone());
            }
        }
        let closure_group = ClosureGroup___10 {
            this__222: self.clone()
        };
        let fn__9635 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1182: SqlPart | closure_group.fn__9635(x__1182))
        };
        self.append_list(temper_core::ToListed::to_listed(values__1180.clone()), fn__9635.clone());
    }
    pub fn append_boolean(& self, value__1184: bool) {
        let mut t___9632: SqlBoolean = SqlBoolean::new(value__1184);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___9632.clone()), None);
    }
    pub fn append_boolean_list(& self, values__1187: impl temper_core::ToListed<bool>) {
        let values__1187 = values__1187.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__224: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__9629(& self, x__1189: bool) {
                self.this__224.append_boolean(x__1189);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__224: self.clone()
        };
        let fn__9629 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1189: bool | closure_group.fn__9629(x__1189))
        };
        self.append_list(values__1187.clone(), fn__9629.clone());
    }
    pub fn append_date(& self, value__1191: temper_std::temporal::Date) {
        let mut t___9626: SqlDate = SqlDate::new(value__1191.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___9626.clone()), None);
    }
    pub fn append_date_list(& self, values__1194: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__1194 = values__1194.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__226: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__9623(& self, x__1196: temper_std::temporal::Date) {
                self.this__226.append_date(x__1196.clone());
            }
        }
        let closure_group = ClosureGroup___12 {
            this__226: self.clone()
        };
        let fn__9623 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1196: temper_std::temporal::Date | closure_group.fn__9623(x__1196))
        };
        self.append_list(values__1194.clone(), fn__9623.clone());
    }
    pub fn append_float64(& self, value__1198: f64) {
        let mut t___9620: SqlFloat64 = SqlFloat64::new(value__1198);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___9620.clone()), None);
    }
    pub fn append_float64_list(& self, values__1201: impl temper_core::ToListed<f64>) {
        let values__1201 = values__1201.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__228: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__9617(& self, x__1203: f64) {
                self.this__228.append_float64(x__1203);
            }
        }
        let closure_group = ClosureGroup___13 {
            this__228: self.clone()
        };
        let fn__9617 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1203: f64 | closure_group.fn__9617(x__1203))
        };
        self.append_list(values__1201.clone(), fn__9617.clone());
    }
    pub fn append_int32(& self, value__1205: i32) {
        let mut t___9614: SqlInt32 = SqlInt32::new(value__1205);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___9614.clone()), None);
    }
    pub fn append_int32_list(& self, values__1208: impl temper_core::ToListed<i32>) {
        let values__1208 = values__1208.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            this__230: SqlBuilder
        }
        impl ClosureGroup___14 {
            fn fn__9611(& self, x__1210: i32) {
                self.this__230.append_int32(x__1210);
            }
        }
        let closure_group = ClosureGroup___14 {
            this__230: self.clone()
        };
        let fn__9611 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1210: i32 | closure_group.fn__9611(x__1210))
        };
        self.append_list(values__1208.clone(), fn__9611.clone());
    }
    pub fn append_int64(& self, value__1212: i64) {
        let mut t___9608: SqlInt64 = SqlInt64::new(value__1212);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___9608.clone()), None);
    }
    pub fn append_int64_list(& self, values__1215: impl temper_core::ToListed<i64>) {
        let values__1215 = values__1215.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___15 {
            this__232: SqlBuilder
        }
        impl ClosureGroup___15 {
            fn fn__9605(& self, x__1217: i64) {
                self.this__232.append_int64(x__1217);
            }
        }
        let closure_group = ClosureGroup___15 {
            this__232: self.clone()
        };
        let fn__9605 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1217: i64 | closure_group.fn__9605(x__1217))
        };
        self.append_list(values__1215.clone(), fn__9605.clone());
    }
    pub fn append_string(& self, value__1219: impl temper_core::ToArcString) {
        let value__1219 = value__1219.to_arc_string();
        let mut t___9602: SqlString = SqlString::new(value__1219.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___9602.clone()), None);
    }
    pub fn append_string_list(& self, values__1222: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__1222 = values__1222.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___16 {
            this__234: SqlBuilder
        }
        impl ClosureGroup___16 {
            fn fn__9599(& self, x__1224: impl temper_core::ToArcString) {
                let x__1224 = x__1224.to_arc_string();
                self.this__234.append_string(x__1224.clone());
            }
        }
        let closure_group = ClosureGroup___16 {
            this__234: self.clone()
        };
        let fn__9599 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1224: std::sync::Arc<String> | closure_group.fn__9599(x__1224))
        };
        self.append_list(values__1222.clone(), fn__9599.clone());
    }
    fn append_list<T>(& self, values__1226: impl temper_core::ToListed<T>, appendValue__1227: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__1226 = values__1226.to_listed();
        let mut t___9594: i32;
        let mut t___9596: T;
        let mut i__1229: i32 = 0;
        'loop___9793: loop {
            t___9594 = temper_core::ListedTrait::len( & ( * values__1226));
            if ! (Some(i__1229) < Some(t___9594)) {
                break;
            }
            if Some(i__1229) > Some(0) {
                self.append_safe(", ");
            }
            t___9596 = temper_core::ListedTrait::get( & ( * values__1226), i__1229);
            appendValue__1227(t___9596.clone());
            i__1229 = i__1229.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___9591: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___9591.clone();
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
        let mut t___9665: i32;
        let builder__1241: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__1242: i32 = 0;
        'loop___9794: loop {
            t___9665 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__1242) < Some(t___9665)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__1242).format_to(builder__1241.clone());
            i__1242 = i__1242.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__1241);
    }
    pub fn new(parts__1244: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__1244 = parts__1244.to_list();
        let parts;
        parts = parts__1244.clone();
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
    fn format_to(& self, builder__1246: std::sync::Arc<std::sync::RwLock<String>>);
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
    pub fn format_to(& self, builder__1250: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1250, self.0.source.clone());
    }
    pub fn new(source__1253: impl temper_core::ToArcString) -> SqlSource {
        let source__1253 = source__1253.to_arc_string();
        let source;
        source = source__1253.clone();
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
    fn format_to(& self, builder__1250: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1250)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__1256: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___5535: std::sync::Arc<String>;
        if self.0.value {
            t___5535 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___5535 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__1256, t___5535.clone());
    }
    pub fn new(value__1259: bool) -> SqlBoolean {
        let value;
        value = value__1259;
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
    fn format_to(& self, builder__1256: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1256)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__1262: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1262, "'");
        let mut t___9646: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___17 {
            builder__1262: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___17 {
            fn fn__9644(& self, c__1264: i32) {
                if Some(c__1264) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1262, "''");
                } else {
                    'ok___9704: {
                        'orelse___1695: {
                            match temper_core::string::builder::append_code_point( & self.builder__1262, c__1264) {
                                Ok(x) => x,
                                _ => break 'orelse___1695
                            };
                            break 'ok___9704;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___17 {
            builder__1262: builder__1262.clone()
        };
        let fn__9644 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1264: i32 | closure_group.fn__9644(c__1264))
        };
        temper_core::string::for_each( & t___9646, & ( * fn__9644.clone()));
        temper_core::string::builder::append( & builder__1262, "'");
    }
    pub fn new(value__1266: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__1266.clone();
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
    fn format_to(& self, builder__1262: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1262)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__1269: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___5524: bool;
        let mut t___5525: bool;
        let s__1271: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__1271.as_str()) == Some("NaN") {
            t___5525 = true;
        } else {
            if Some(s__1271.as_str()) == Some("Infinity") {
                t___5524 = true;
            } else {
                t___5524 = Some(s__1271.as_str()) == Some("-Infinity");
            }
            t___5525 = t___5524;
        }
        if t___5525 {
            temper_core::string::builder::append( & builder__1269, "NULL");
        } else {
            temper_core::string::builder::append( & builder__1269, s__1271.clone());
        }
    }
    pub fn new(value__1273: f64) -> SqlFloat64 {
        let value;
        value = value__1273;
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
    fn format_to(& self, builder__1269: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1269)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__1276: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___9655: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1276, t___9655.clone());
    }
    pub fn new(value__1279: i32) -> SqlInt32 {
        let value;
        value = value__1279;
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
    fn format_to(& self, builder__1276: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1276)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__1282: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___9653: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1282, t___9653.clone());
    }
    pub fn new(value__1285: i64) -> SqlInt64 {
        let value;
        value = value__1285;
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
    fn format_to(& self, builder__1282: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1282)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__1288: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1288, "'");
        #[derive(Clone)]
        struct ClosureGroup___18 {
            builder__1288: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___18 {
            fn fn__9658(& self, c__1290: i32) {
                if Some(c__1290) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1288, "''");
                } else {
                    'ok___9709: {
                        'orelse___1694: {
                            match temper_core::string::builder::append_code_point( & self.builder__1288, c__1290) {
                                Ok(x) => x,
                                _ => break 'orelse___1694
                            };
                            break 'ok___9709;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___18 {
            builder__1288: builder__1288.clone()
        };
        let fn__9658 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1290: i32 | closure_group.fn__9658(c__1290))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__9658.clone()));
        temper_core::string::builder::append( & builder__1288, "'");
    }
    pub fn new(value__1292: impl temper_core::ToArcString) -> SqlString {
        let value__1292 = value__1292.to_arc_string();
        let value;
        value = value__1292.clone();
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
    fn format_to(& self, builder__1288: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1288)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__601: TableDef, params__602: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___9441: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__601.clone(), params__602.clone(), t___9441.clone(), [], true));
}
fn isIdentStart__462(c__1121: i32) -> bool {
    let return__387: bool;
    let mut t___5298: bool;
    let mut t___5299: bool;
    if Some(c__1121) >= Some(97) {
        t___5298 = Some(c__1121) <= Some(122);
    } else {
        t___5298 = false;
    }
    if t___5298 {
        return__387 = true;
    } else {
        if Some(c__1121) >= Some(65) {
            t___5299 = Some(c__1121) <= Some(90);
        } else {
            t___5299 = false;
        }
        if t___5299 {
            return__387 = true;
        } else {
            return__387 = Some(c__1121) == Some(95);
        }
    }
    return return__387;
}
fn isIdentPart__463(c__1123: i32) -> bool {
    let return__388: bool;
    if isIdentStart__462(c__1123) {
        return__388 = true;
    } else {
        if Some(c__1123) >= Some(48) {
            return__388 = Some(c__1123) <= Some(57);
        } else {
            return__388 = false;
        }
    }
    return return__388;
}
pub fn safe_identifier(name__1125: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__1125 = name__1125.to_arc_string();
    let mut t___9439: usize;
    if name__1125.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__1127: usize = 0usize;
    if ! isIdentStart__462(temper_core::string::get( & name__1125, idx__1127)) {
        return Err(temper_core::Error::new());
    }
    let mut t___9436: usize = temper_core::string::next( & name__1125, idx__1127);
    idx__1127 = t___9436;
    'loop___9795: loop {
        if ! temper_core::string::has_index( & name__1125, idx__1127) {
            break;
        }
        if ! isIdentPart__463(temper_core::string::get( & name__1125, idx__1127)) {
            return Err(temper_core::Error::new());
        }
        t___9439 = temper_core::string::next( & name__1125, idx__1127);
        idx__1127 = t___9439;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__1125.clone())));
}
fn csid__459(name__604: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__604 = name__604.to_arc_string();
    let return__295: SafeIdentifier;
    let mut t___5286: SafeIdentifier;
    'ok___9714: {
        'orelse___1699: {
            t___5286 = match safe_identifier(name__604.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1699
            };
            return__295 = t___5286.clone();
            break 'ok___9714;
        }
        return__295 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__295.clone();
}
fn userTable__460() -> TableDef {
    return TableDef::new(csid__459("users"), [FieldDef::new(csid__459("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__459("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__459("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__459("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__459("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__691: TableDef, id__692: i32) -> SqlFragment {
    let b__694: SqlBuilder = SqlBuilder::new();
    b__694.append_safe("DELETE FROM ");
    b__694.append_safe(tableDef__691.table_name().sql_value());
    b__694.append_safe(" WHERE id = ");
    b__694.append_int32(id__692);
    return b__694.accumulated();
}
pub fn from(tableName__886: SafeIdentifier) -> Query {
    return Query::new(tableName__886.clone(), [], [], [], None, None, [], [], [], false, []);
}
pub fn col(table__888: SafeIdentifier, column__889: SafeIdentifier) -> SqlFragment {
    let b__891: SqlBuilder = SqlBuilder::new();
    b__891.append_safe(table__888.sql_value());
    b__891.append_safe(".");
    b__891.append_safe(column__889.sql_value());
    return b__891.accumulated();
}
pub fn count_all() -> SqlFragment {
    let b__893: SqlBuilder = SqlBuilder::new();
    b__893.append_safe("COUNT(*)");
    return b__893.accumulated();
}
pub fn count_col(field__894: SafeIdentifier) -> SqlFragment {
    let b__896: SqlBuilder = SqlBuilder::new();
    b__896.append_safe("COUNT(");
    b__896.append_safe(field__894.sql_value());
    b__896.append_safe(")");
    return b__896.accumulated();
}
pub fn sum_col(field__897: SafeIdentifier) -> SqlFragment {
    let b__899: SqlBuilder = SqlBuilder::new();
    b__899.append_safe("SUM(");
    b__899.append_safe(field__897.sql_value());
    b__899.append_safe(")");
    return b__899.accumulated();
}
pub fn avg_col(field__900: SafeIdentifier) -> SqlFragment {
    let b__902: SqlBuilder = SqlBuilder::new();
    b__902.append_safe("AVG(");
    b__902.append_safe(field__900.sql_value());
    b__902.append_safe(")");
    return b__902.accumulated();
}
pub fn min_col(field__903: SafeIdentifier) -> SqlFragment {
    let b__905: SqlBuilder = SqlBuilder::new();
    b__905.append_safe("MIN(");
    b__905.append_safe(field__903.sql_value());
    b__905.append_safe(")");
    return b__905.accumulated();
}
pub fn max_col(field__906: SafeIdentifier) -> SqlFragment {
    let b__908: SqlBuilder = SqlBuilder::new();
    b__908.append_safe("MAX(");
    b__908.append_safe(field__906.sql_value());
    b__908.append_safe(")");
    return b__908.accumulated();
}
pub fn union_sql(a__909: Query, b__910: Query) -> SqlFragment {
    let sb__912: SqlBuilder = SqlBuilder::new();
    sb__912.append_safe("(");
    sb__912.append_fragment(a__909.to_sql());
    sb__912.append_safe(") UNION (");
    sb__912.append_fragment(b__910.to_sql());
    sb__912.append_safe(")");
    return sb__912.accumulated();
}
pub fn union_all_sql(a__913: Query, b__914: Query) -> SqlFragment {
    let sb__916: SqlBuilder = SqlBuilder::new();
    sb__916.append_safe("(");
    sb__916.append_fragment(a__913.to_sql());
    sb__916.append_safe(") UNION ALL (");
    sb__916.append_fragment(b__914.to_sql());
    sb__916.append_safe(")");
    return sb__916.accumulated();
}
pub fn intersect_sql(a__917: Query, b__918: Query) -> SqlFragment {
    let sb__920: SqlBuilder = SqlBuilder::new();
    sb__920.append_safe("(");
    sb__920.append_fragment(a__917.to_sql());
    sb__920.append_safe(") INTERSECT (");
    sb__920.append_fragment(b__918.to_sql());
    sb__920.append_safe(")");
    return sb__920.accumulated();
}
pub fn except_sql(a__921: Query, b__922: Query) -> SqlFragment {
    let sb__924: SqlBuilder = SqlBuilder::new();
    sb__924.append_safe("(");
    sb__924.append_fragment(a__921.to_sql());
    sb__924.append_safe(") EXCEPT (");
    sb__924.append_fragment(b__922.to_sql());
    sb__924.append_safe(")");
    return sb__924.accumulated();
}
pub fn subquery(q__925: Query, alias__926: SafeIdentifier) -> SqlFragment {
    let b__928: SqlBuilder = SqlBuilder::new();
    b__928.append_safe("(");
    b__928.append_fragment(q__925.to_sql());
    b__928.append_safe(") AS ");
    b__928.append_safe(alias__926.sql_value());
    return b__928.accumulated();
}
pub fn exists_sql(q__929: Query) -> SqlFragment {
    let b__931: SqlBuilder = SqlBuilder::new();
    b__931.append_safe("EXISTS (");
    b__931.append_fragment(q__929.to_sql());
    b__931.append_safe(")");
    return b__931.accumulated();
}
fn sid__461(name__932: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__932 = name__932.to_arc_string();
    let return__380: SafeIdentifier;
    let mut t___4603: SafeIdentifier;
    'ok___9725: {
        'orelse___1707: {
            t___4603 = match safe_identifier(name__932.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1707
            };
            return__380 = t___4603.clone();
            break 'ok___9725;
        }
        return__380 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__380.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__1404() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let params__608: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___9397: TableDef = userTable__460();
        let mut t___9398: SafeIdentifier = csid__459("name");
        let mut t___9399: SafeIdentifier = csid__459("email");
        let cs__609: Changeset = changeset(t___9397.clone(), params__608.clone()).cast(std::sync::Arc::new(vec![t___9398.clone(), t___9399.clone()]));
        let mut t___9402: bool = temper_core::MappedTrait::has( & cs__609.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__9392(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__9392 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9392())
        };
        test___24.assert(t___9402, fn__9392.clone());
        let mut t___9406: bool = temper_core::MappedTrait::has( & cs__609.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__9391(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__9391 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9391())
        };
        test___24.assert(t___9406, fn__9391.clone());
        let mut t___9412: bool = ! temper_core::MappedTrait::has( & cs__609.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__9390(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__9390 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9390())
        };
        test___24.assert(t___9412, fn__9390.clone());
        let mut t___9414: bool = cs__609.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__9389(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__9389 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9389())
        };
        test___24.assert(t___9414, fn__9389.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__1405() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let params__611: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___9375: TableDef = userTable__460();
        let mut t___9376: SafeIdentifier = csid__459("name");
        let cs__612: Changeset = changeset(t___9375.clone(), params__611.clone()).cast(std::sync::Arc::new(vec![t___9376.clone()])).cast(std::sync::Arc::new(vec![csid__459("email")]));
        let mut t___9383: bool = ! temper_core::MappedTrait::has( & cs__612.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__9371(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__9371 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9371())
        };
        test___25.assert(t___9383, fn__9371.clone());
        let mut t___9386: bool = temper_core::MappedTrait::has( & cs__612.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__9370(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__9370 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9370())
        };
        test___25.assert(t___9386, fn__9370.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__1406() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let params__614: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___9357: TableDef = userTable__460();
        let mut t___9358: SafeIdentifier = csid__459("name");
        let mut t___9359: SafeIdentifier = csid__459("email");
        let cs__615: Changeset = changeset(t___9357.clone(), params__614.clone()).cast(std::sync::Arc::new(vec![t___9358.clone(), t___9359.clone()]));
        let mut t___9364: bool = ! temper_core::MappedTrait::has( & cs__615.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__9353(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__9353 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9353())
        };
        test___26.assert(t___9364, fn__9353.clone());
        let mut t___9367: bool = temper_core::MappedTrait::has( & cs__615.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__9352(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__9352 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9352())
        };
        test___26.assert(t___9367, fn__9352.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__1407() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let params__617: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___9339: TableDef = userTable__460();
        let mut t___9340: SafeIdentifier = csid__459("name");
        let cs__618: Changeset = changeset(t___9339.clone(), params__617.clone()).cast(std::sync::Arc::new(vec![t___9340.clone()])).validate_required(std::sync::Arc::new(vec![csid__459("name")]));
        let mut t___9344: bool = cs__618.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__9336(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__9336 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9336())
        };
        test___27.assert(t___9344, fn__9336.clone());
        let mut t___9350: bool = Some(temper_core::ListedTrait::len( & cs__618.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__9335(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__9335 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9335())
        };
        test___27.assert(t___9350, fn__9335.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__1408() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let params__620: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___9315: TableDef = userTable__460();
        let mut t___9316: SafeIdentifier = csid__459("name");
        let cs__621: Changeset = changeset(t___9315.clone(), params__620.clone()).cast(std::sync::Arc::new(vec![t___9316.clone()])).validate_required(std::sync::Arc::new(vec![csid__459("name")]));
        let mut t___9322: bool = ! cs__621.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__9313(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__9313 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9313())
        };
        test___28.assert(t___9322, fn__9313.clone());
        let mut t___9327: bool = Some(temper_core::ListedTrait::len( & cs__621.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__9312(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__9312 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9312())
        };
        test___28.assert(t___9327, fn__9312.clone());
        let mut t___9333: bool = Some(temper_core::ListedTrait::get( & cs__621.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__9311(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__9311 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9311())
        };
        test___28.assert(t___9333, fn__9311.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__1409() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let params__623: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___9303: TableDef = userTable__460();
        let mut t___9304: SafeIdentifier = csid__459("name");
        let cs__624: Changeset = changeset(t___9303.clone(), params__623.clone()).cast(std::sync::Arc::new(vec![t___9304.clone()])).validate_length(csid__459("name"), 2, 50);
        let mut t___9308: bool = cs__624.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__9300(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__9300 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9300())
        };
        test___29.assert(t___9308, fn__9300.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__1410() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let params__626: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___9291: TableDef = userTable__460();
        let mut t___9292: SafeIdentifier = csid__459("name");
        let cs__627: Changeset = changeset(t___9291.clone(), params__626.clone()).cast(std::sync::Arc::new(vec![t___9292.clone()])).validate_length(csid__459("name"), 2, 50);
        let mut t___9298: bool = ! cs__627.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__9288(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__9288 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9288())
        };
        test___30.assert(t___9298, fn__9288.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__1411() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__629: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___9279: TableDef = userTable__460();
        let mut t___9280: SafeIdentifier = csid__459("name");
        let cs__630: Changeset = changeset(t___9279.clone(), params__629.clone()).cast(std::sync::Arc::new(vec![t___9280.clone()])).validate_length(csid__459("name"), 2, 10);
        let mut t___9286: bool = ! cs__630.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__9276(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__9276 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9276())
        };
        test___31.assert(t___9286, fn__9276.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__1412() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__632: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___9268: TableDef = userTable__460();
        let mut t___9269: SafeIdentifier = csid__459("age");
        let cs__633: Changeset = changeset(t___9268.clone(), params__632.clone()).cast(std::sync::Arc::new(vec![t___9269.clone()])).validate_int(csid__459("age"));
        let mut t___9273: bool = cs__633.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__9265(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__9265 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9265())
        };
        test___32.assert(t___9273, fn__9265.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__1413() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__635: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___9256: TableDef = userTable__460();
        let mut t___9257: SafeIdentifier = csid__459("age");
        let cs__636: Changeset = changeset(t___9256.clone(), params__635.clone()).cast(std::sync::Arc::new(vec![t___9257.clone()])).validate_int(csid__459("age"));
        let mut t___9263: bool = ! cs__636.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__9253(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__9253 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9253())
        };
        test___33.assert(t___9263, fn__9253.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__1414() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__638: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___9245: TableDef = userTable__460();
        let mut t___9246: SafeIdentifier = csid__459("score");
        let cs__639: Changeset = changeset(t___9245.clone(), params__638.clone()).cast(std::sync::Arc::new(vec![t___9246.clone()])).validate_float(csid__459("score"));
        let mut t___9250: bool = cs__639.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__9242(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__9242 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9242())
        };
        test___34.assert(t___9250, fn__9242.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__1415() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let params__641: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___9234: TableDef = userTable__460();
        let mut t___9235: SafeIdentifier = csid__459("age");
        let cs__642: Changeset = changeset(t___9234.clone(), params__641.clone()).cast(std::sync::Arc::new(vec![t___9235.clone()])).validate_int64(csid__459("age"));
        let mut t___9239: bool = cs__642.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___38 {}
        impl ClosureGroup___38 {
            fn fn__9231(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___38 {};
        let fn__9231 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9231())
        };
        test___35.assert(t___9239, fn__9231.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__1416() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let params__644: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___9222: TableDef = userTable__460();
        let mut t___9223: SafeIdentifier = csid__459("age");
        let cs__645: Changeset = changeset(t___9222.clone(), params__644.clone()).cast(std::sync::Arc::new(vec![t___9223.clone()])).validate_int64(csid__459("age"));
        let mut t___9229: bool = ! cs__645.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__9219(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__9219 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9219())
        };
        test___36.assert(t___9229, fn__9219.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__1417() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___40 {
            test___37: temper_std::testing::Test
        }
        impl ClosureGroup___40 {
            fn fn__9216(& self, v__647: impl temper_core::ToArcString) {
                let v__647 = v__647.to_arc_string();
                let params__648: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__647.clone())]);
                let mut t___9208: TableDef = userTable__460();
                let mut t___9209: SafeIdentifier = csid__459("active");
                let cs__649: Changeset = changeset(t___9208.clone(), params__648.clone()).cast(std::sync::Arc::new(vec![t___9209.clone()])).validate_bool(csid__459("active"));
                let mut t___9213: bool = cs__649.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___41 {
                    v__647: std::sync::Arc<String>
                }
                impl ClosureGroup___41 {
                    fn fn__9205(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__647.clone()));
                    }
                }
                let closure_group = ClosureGroup___41 {
                    v__647: v__647.clone()
                };
                let fn__9205 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__9205())
                };
                self.test___37.assert(t___9213, fn__9205.clone());
            }
        }
        let closure_group = ClosureGroup___40 {
            test___37: test___37.clone()
        };
        let fn__9216 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__647: std::sync::Arc<String> | closure_group.fn__9216(v__647))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__9216.clone()));
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__1418() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            test___38: temper_std::testing::Test
        }
        impl ClosureGroup___42 {
            fn fn__9202(& self, v__651: impl temper_core::ToArcString) {
                let v__651 = v__651.to_arc_string();
                let params__652: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__651.clone())]);
                let mut t___9194: TableDef = userTable__460();
                let mut t___9195: SafeIdentifier = csid__459("active");
                let cs__653: Changeset = changeset(t___9194.clone(), params__652.clone()).cast(std::sync::Arc::new(vec![t___9195.clone()])).validate_bool(csid__459("active"));
                let mut t___9199: bool = cs__653.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___43 {
                    v__651: std::sync::Arc<String>
                }
                impl ClosureGroup___43 {
                    fn fn__9191(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__651.clone()));
                    }
                }
                let closure_group = ClosureGroup___43 {
                    v__651: v__651.clone()
                };
                let fn__9191 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__9191())
                };
                self.test___38.assert(t___9199, fn__9191.clone());
            }
        }
        let closure_group = ClosureGroup___42 {
            test___38: test___38.clone()
        };
        let fn__9202 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__651: std::sync::Arc<String> | closure_group.fn__9202(v__651))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__9202.clone()));
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__1419() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            test___39: temper_std::testing::Test
        }
        impl ClosureGroup___44 {
            fn fn__9188(& self, v__655: impl temper_core::ToArcString) {
                let v__655 = v__655.to_arc_string();
                let params__656: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__655.clone())]);
                let mut t___9179: TableDef = userTable__460();
                let mut t___9180: SafeIdentifier = csid__459("active");
                let cs__657: Changeset = changeset(t___9179.clone(), params__656.clone()).cast(std::sync::Arc::new(vec![t___9180.clone()])).validate_bool(csid__459("active"));
                let mut t___9186: bool = ! cs__657.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___45 {
                    v__655: std::sync::Arc<String>
                }
                impl ClosureGroup___45 {
                    fn fn__9176(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__655.clone()));
                    }
                }
                let closure_group = ClosureGroup___45 {
                    v__655: v__655.clone()
                };
                let fn__9176 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__9176())
                };
                self.test___39.assert(t___9186, fn__9176.clone());
            }
        }
        let closure_group = ClosureGroup___44 {
            test___39: test___39.clone()
        };
        let fn__9188 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__655: std::sync::Arc<String> | closure_group.fn__9188(v__655))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__9188.clone()));
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__1420() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let mut t___5087: SqlFragment;
        let params__659: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___9164: TableDef = userTable__460();
        let mut t___9165: SafeIdentifier = csid__459("name");
        let mut t___9166: SafeIdentifier = csid__459("email");
        let cs__660: Changeset = changeset(t___9164.clone(), params__659.clone()).cast(std::sync::Arc::new(vec![t___9165.clone(), t___9166.clone()])).validate_required(std::sync::Arc::new(vec![csid__459("name"), csid__459("email")]));
        let sqlFrag__661: SqlFragment;
        'ok___9716: {
            'orelse___1700: {
                t___5087 = match cs__660.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1700
                };
                sqlFrag__661 = t___5087.clone();
                break 'ok___9716;
            }
            sqlFrag__661 = panic!();
        }
        let s__662: std::sync::Arc<String> = sqlFrag__661.to_string();
        let mut t___9173: bool = temper_core::string::index_of( & s__662, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            s__662: std::sync::Arc<String>
        }
        impl ClosureGroup___46 {
            fn fn__9160(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__662.clone()));
            }
        }
        let closure_group = ClosureGroup___46 {
            s__662: s__662.clone()
        };
        let fn__9160 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9160())
        };
        test___40.assert(t___9173, fn__9160.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__1421() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let mut t___5066: SqlFragment;
        let params__664: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___9144: TableDef = userTable__460();
        let mut t___9145: SafeIdentifier = csid__459("name");
        let mut t___9146: SafeIdentifier = csid__459("email");
        let cs__665: Changeset = changeset(t___9144.clone(), params__664.clone()).cast(std::sync::Arc::new(vec![t___9145.clone(), t___9146.clone()])).validate_required(std::sync::Arc::new(vec![csid__459("name"), csid__459("email")]));
        let sqlFrag__666: SqlFragment;
        'ok___9719: {
            'orelse___1701: {
                t___5066 = match cs__665.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1701
                };
                sqlFrag__666 = t___5066.clone();
                break 'ok___9719;
            }
            sqlFrag__666 = panic!();
        }
        let s__667: std::sync::Arc<String> = sqlFrag__666.to_string();
        let mut t___9153: bool = temper_core::string::index_of( & s__667, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___47 {
            s__667: std::sync::Arc<String>
        }
        impl ClosureGroup___47 {
            fn fn__9140(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__667.clone()));
            }
        }
        let closure_group = ClosureGroup___47 {
            s__667: s__667.clone()
        };
        let fn__9140 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9140())
        };
        test___41.assert(t___9153, fn__9140.clone());
        let mut t___9157: bool = temper_core::string::index_of( & s__667, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___48 {
            s__667: std::sync::Arc<String>
        }
        impl ClosureGroup___48 {
            fn fn__9139(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__667.clone()));
            }
        }
        let closure_group = ClosureGroup___48 {
            s__667: s__667.clone()
        };
        let fn__9139 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9139())
        };
        test___41.assert(t___9157, fn__9139.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__1422() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let mut t___5049: SqlFragment;
        let params__669: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___9126: TableDef = userTable__460();
        let mut t___9127: SafeIdentifier = csid__459("name");
        let mut t___9128: SafeIdentifier = csid__459("email");
        let mut t___9129: SafeIdentifier = csid__459("age");
        let cs__670: Changeset = changeset(t___9126.clone(), params__669.clone()).cast(std::sync::Arc::new(vec![t___9127.clone(), t___9128.clone(), t___9129.clone()])).validate_required(std::sync::Arc::new(vec![csid__459("name"), csid__459("email")]));
        let sqlFrag__671: SqlFragment;
        'ok___9720: {
            'orelse___1702: {
                t___5049 = match cs__670.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1702
                };
                sqlFrag__671 = t___5049.clone();
                break 'ok___9720;
            }
            sqlFrag__671 = panic!();
        }
        let s__672: std::sync::Arc<String> = sqlFrag__671.to_string();
        let mut t___9136: bool = temper_core::string::index_of( & s__672, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___49 {
            s__672: std::sync::Arc<String>
        }
        impl ClosureGroup___49 {
            fn fn__9121(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__672.clone()));
            }
        }
        let closure_group = ClosureGroup___49 {
            s__672: s__672.clone()
        };
        let fn__9121 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9121())
        };
        test___42.assert(t___9136, fn__9121.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__1423() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let params__674: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___9114: TableDef = userTable__460();
        let mut t___9115: SafeIdentifier = csid__459("name");
        let cs__675: Changeset = changeset(t___9114.clone(), params__674.clone()).cast(std::sync::Arc::new(vec![t___9115.clone()])).validate_required(std::sync::Arc::new(vec![csid__459("name")]));
        let didBubble__676: bool;
        'ok___9721: {
            'orelse___1703: {
                match cs__675.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1703
                };
                didBubble__676 = false;
                break 'ok___9721;
            }
            didBubble__676 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___50 {}
        impl ClosureGroup___50 {
            fn fn__9112(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___50 {};
        let fn__9112 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9112())
        };
        test___43.assert(didBubble__676, fn__9112.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__1424() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let strictTable__678: TableDef = TableDef::new(csid__459("posts"), [FieldDef::new(csid__459("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__459("body"), FieldType::new(StringField::new()), true)]);
        let params__679: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___9105: SafeIdentifier = csid__459("body");
        let cs__680: Changeset = changeset(strictTable__678.clone(), params__679.clone()).cast(std::sync::Arc::new(vec![t___9105.clone()]));
        let mut t___9107: bool = cs__680.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___51 {}
        impl ClosureGroup___51 {
            fn fn__9094(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___51 {};
        let fn__9094 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9094())
        };
        test___44.assert(t___9107, fn__9094.clone());
        let didBubble__681: bool;
        'ok___9722: {
            'orelse___1704: {
                match cs__680.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1704
                };
                didBubble__681 = false;
                break 'ok___9722;
            }
            didBubble__681 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__9093(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__9093 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9093())
        };
        test___44.assert(didBubble__681, fn__9093.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__1425() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let mut t___5009: SqlFragment;
        let params__683: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___9084: TableDef = userTable__460();
        let mut t___9085: SafeIdentifier = csid__459("name");
        let cs__684: Changeset = changeset(t___9084.clone(), params__683.clone()).cast(std::sync::Arc::new(vec![t___9085.clone()])).validate_required(std::sync::Arc::new(vec![csid__459("name")]));
        let sqlFrag__685: SqlFragment;
        'ok___9723: {
            'orelse___1705: {
                t___5009 = match cs__684.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___1705
                };
                sqlFrag__685 = t___5009.clone();
                break 'ok___9723;
            }
            sqlFrag__685 = panic!();
        }
        let s__686: std::sync::Arc<String> = sqlFrag__685.to_string();
        let mut t___9091: bool = Some(s__686.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___53 {
            s__686: std::sync::Arc<String>
        }
        impl ClosureGroup___53 {
            fn fn__9081(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__686.clone()));
            }
        }
        let closure_group = ClosureGroup___53 {
            s__686: s__686.clone()
        };
        let fn__9081 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9081())
        };
        test___45.assert(t___9091, fn__9081.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__1426() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let params__688: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___9074: TableDef = userTable__460();
        let mut t___9075: SafeIdentifier = csid__459("name");
        let cs__689: Changeset = changeset(t___9074.clone(), params__688.clone()).cast(std::sync::Arc::new(vec![t___9075.clone()])).validate_required(std::sync::Arc::new(vec![csid__459("name")]));
        let didBubble__690: bool;
        'ok___9724: {
            'orelse___1706: {
                match cs__689.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___1706
                };
                didBubble__690 = false;
                break 'ok___9724;
            }
            didBubble__690 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__9072(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__9072 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9072())
        };
        test___46.assert(didBubble__690, fn__9072.clone());
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__1475() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let q__935: Query = from(sid__461("users"));
        let mut t___8681: bool = Some(q__935.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__8676(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__8676 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8676())
        };
        test___47.assert(t___8681, fn__8676.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__1476() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___8667: SafeIdentifier = sid__461("users");
        let mut t___8668: SafeIdentifier = sid__461("id");
        let mut t___8669: SafeIdentifier = sid__461("name");
        let q__937: Query = from(t___8667.clone()).select([t___8668.clone(), t___8669.clone()]);
        let mut t___8674: bool = Some(q__937.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__8666(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__8666 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8666())
        };
        test___48.assert(t___8674, fn__8666.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__1477() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___8655: SafeIdentifier = sid__461("users");
        let mut t___8656: SqlBuilder = SqlBuilder::new();
        t___8656.append_safe("age > ");
        t___8656.append_int32(18);
        let mut t___8659: SqlFragment = t___8656.accumulated();
        let q__939: Query = from(t___8655.clone()).r#where(t___8659.clone());
        let mut t___8664: bool = Some(q__939.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__8654(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__8654 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8654())
        };
        test___49.assert(t___8664, fn__8654.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__1479() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___8643: SafeIdentifier = sid__461("users");
        let mut t___8644: SqlBuilder = SqlBuilder::new();
        t___8644.append_safe("active = ");
        t___8644.append_boolean(true);
        let mut t___8647: SqlFragment = t___8644.accumulated();
        let q__941: Query = from(t___8643.clone()).r#where(t___8647.clone());
        let mut t___8652: bool = Some(q__941.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__8642(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__8642 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8642())
        };
        test___50.assert(t___8652, fn__8642.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__1481() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let mut t___8626: SafeIdentifier = sid__461("users");
        let mut t___8627: SqlBuilder = SqlBuilder::new();
        t___8627.append_safe("age > ");
        t___8627.append_int32(18);
        let mut t___8630: SqlFragment = t___8627.accumulated();
        let mut t___8631: Query = from(t___8626.clone()).r#where(t___8630.clone());
        let mut t___8632: SqlBuilder = SqlBuilder::new();
        t___8632.append_safe("active = ");
        t___8632.append_boolean(true);
        let q__943: Query = t___8631.r#where(t___8632.accumulated());
        let mut t___8640: bool = Some(q__943.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__8625(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__8625 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8625())
        };
        test___51.assert(t___8640, fn__8625.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__1484() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___8617: SafeIdentifier = sid__461("users");
        let mut t___8618: SafeIdentifier = sid__461("name");
        let q__945: Query = from(t___8617.clone()).order_by(t___8618.clone(), true);
        let mut t___8623: bool = Some(q__945.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__8616(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__8616 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8616())
        };
        test___52.assert(t___8623, fn__8616.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__1485() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let mut t___8608: SafeIdentifier = sid__461("users");
        let mut t___8609: SafeIdentifier = sid__461("created_at");
        let q__947: Query = from(t___8608.clone()).order_by(t___8609.clone(), false);
        let mut t___8614: bool = Some(q__947.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__8607(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__8607 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8607())
        };
        test___53.assert(t___8614, fn__8607.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__1486() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let mut t___4537: Query;
        let mut t___4538: Query;
        let q__949: Query;
        'ok___9726: {
            'orelse___1708: {
                t___4537 = match from(sid__461("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1708
                };
                t___4538 = match t___4537.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1708
                };
                q__949 = t___4538.clone();
                break 'ok___9726;
            }
            q__949 = panic!();
        }
        let mut t___8605: bool = Some(q__949.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__8600(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__8600 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8600())
        };
        test___54.assert(t___8605, fn__8600.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__1487() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let didBubble__951: bool;
        'ok___9727: {
            'orelse___1709: {
                match from(sid__461("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1709
                };
                didBubble__951 = false;
                break 'ok___9727;
            }
            didBubble__951 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___63 {}
        impl ClosureGroup___63 {
            fn fn__8596(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___63 {};
        let fn__8596 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8596())
        };
        test___55.assert(didBubble__951, fn__8596.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__1488() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let didBubble__953: bool;
        'ok___9728: {
            'orelse___1710: {
                match from(sid__461("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1710
                };
                didBubble__953 = false;
                break 'ok___9728;
            }
            didBubble__953 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___64 {}
        impl ClosureGroup___64 {
            fn fn__8592(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___64 {};
        let fn__8592 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8592())
        };
        test___56.assert(didBubble__953, fn__8592.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__1489() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___8570: SafeIdentifier;
        let mut t___8571: SafeIdentifier;
        let mut t___8572: SafeIdentifier;
        let mut t___8573: SafeIdentifier;
        let mut t___8574: Query;
        let mut t___8575: SqlBuilder;
        let mut t___8579: Query;
        let mut t___8580: SqlBuilder;
        let mut t___4523: Query;
        let mut t___4524: Query;
        let minAge__955: i32 = 21;
        let q__956: Query;
        'ok___9729: {
            'orelse___1711: {
                t___8570 = sid__461("users");
                t___8571 = sid__461("id");
                t___8572 = sid__461("name");
                t___8573 = sid__461("email");
                t___8574 = from(t___8570.clone()).select([t___8571.clone(), t___8572.clone(), t___8573.clone()]);
                t___8575 = SqlBuilder::new();
                t___8575.append_safe("age >= ");
                t___8575.append_int32(21);
                t___8579 = t___8574.r#where(t___8575.accumulated());
                t___8580 = SqlBuilder::new();
                t___8580.append_safe("active = ");
                t___8580.append_boolean(true);
                t___4523 = match t___8579.r#where(t___8580.accumulated()).order_by(sid__461("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___1711
                };
                t___4524 = match t___4523.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___1711
                };
                q__956 = t___4524.clone();
                break 'ok___9729;
            }
            q__956 = panic!();
        }
        let mut t___8590: bool = Some(q__956.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__8569(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__8569 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8569())
        };
        test___57.assert(t___8590, fn__8569.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__1492() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let mut t___4500: SqlFragment;
        let mut t___4501: SqlFragment;
        let q__958: Query = from(sid__461("users"));
        'ok___9730: {
            'orelse___1712: {
                t___4500 = match q__958.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1712
                };
                t___4501 = t___4500.clone();
                break 'ok___9730;
            }
            t___4501 = panic!();
        }
        let s__959: std::sync::Arc<String> = t___4501.to_string();
        let mut t___8567: bool = Some(s__959.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___66 {
            s__959: std::sync::Arc<String>
        }
        impl ClosureGroup___66 {
            fn fn__8563(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__959.clone()));
            }
        }
        let closure_group = ClosureGroup___66 {
            s__959: s__959.clone()
        };
        let fn__8563 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8563())
        };
        test___58.assert(t___8567, fn__8563.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__1493() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let mut t___4492: Query;
        let mut t___4495: SqlFragment;
        let mut t___4496: SqlFragment;
        let q__961: Query;
        'ok___9731: {
            'orelse___1713: {
                t___4492 = match from(sid__461("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___1713
                };
                q__961 = t___4492.clone();
                break 'ok___9731;
            }
            q__961 = panic!();
        }
        'ok___9732: {
            'orelse___1714: {
                t___4495 = match q__961.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1714
                };
                t___4496 = t___4495.clone();
                break 'ok___9732;
            }
            t___4496 = panic!();
        }
        let s__962: std::sync::Arc<String> = t___4496.to_string();
        let mut t___8561: bool = Some(s__962.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___67 {
            s__962: std::sync::Arc<String>
        }
        impl ClosureGroup___67 {
            fn fn__8557(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__962.clone()));
            }
        }
        let closure_group = ClosureGroup___67 {
            s__962: s__962.clone()
        };
        let fn__8557 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8557())
        };
        test___59.assert(t___8561, fn__8557.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__1494() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let didBubble__964: bool;
        'ok___9733: {
            'orelse___1715: {
                match from(sid__461("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1715
                };
                didBubble__964 = false;
                break 'ok___9733;
            }
            didBubble__964 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__8553(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__8553 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8553())
        };
        test___60.assert(didBubble__964, fn__8553.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__1495() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let evil__966: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___8537: SafeIdentifier = sid__461("users");
        let mut t___8538: SqlBuilder = SqlBuilder::new();
        t___8538.append_safe("name = ");
        t___8538.append_string("'; DROP TABLE users; --");
        let mut t___8541: SqlFragment = t___8538.accumulated();
        let q__967: Query = from(t___8537.clone()).r#where(t___8541.clone());
        let s__968: std::sync::Arc<String> = q__967.to_sql().to_string();
        let mut t___8546: bool = temper_core::string::index_of( & s__968, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___69 {
            s__968: std::sync::Arc<String>
        }
        impl ClosureGroup___69 {
            fn fn__8536(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__968.clone()));
            }
        }
        let closure_group = ClosureGroup___69 {
            s__968: s__968.clone()
        };
        let fn__8536 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8536())
        };
        test___61.assert(t___8546, fn__8536.clone());
        let mut t___8550: bool = temper_core::string::index_of( & s__968, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___70 {
            s__968: std::sync::Arc<String>
        }
        impl ClosureGroup___70 {
            fn fn__8535(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__968.clone()));
            }
        }
        let closure_group = ClosureGroup___70 {
            s__968: s__968.clone()
        };
        let fn__8535 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8535())
        };
        test___61.assert(t___8550, fn__8535.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__1497() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let attack__970: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__971: bool;
        'ok___9734: {
            'orelse___1716: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___1716
                };
                didBubble__971 = false;
                break 'ok___9734;
            }
            didBubble__971 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__8532(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__8532 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8532())
        };
        test___62.assert(didBubble__971, fn__8532.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__1498() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let mut t___8521: SafeIdentifier = sid__461("users");
        let mut t___8522: SafeIdentifier = sid__461("orders");
        let mut t___8523: SqlBuilder = SqlBuilder::new();
        t___8523.append_safe("users.id = orders.user_id");
        let mut t___8525: SqlFragment = t___8523.accumulated();
        let q__973: Query = from(t___8521.clone()).inner_join(t___8522.clone(), t___8525.clone());
        let mut t___8530: bool = Some(q__973.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__8520(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__8520 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8520())
        };
        test___63.assert(t___8530, fn__8520.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__1500() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let mut t___8509: SafeIdentifier = sid__461("users");
        let mut t___8510: SafeIdentifier = sid__461("profiles");
        let mut t___8511: SqlBuilder = SqlBuilder::new();
        t___8511.append_safe("users.id = profiles.user_id");
        let mut t___8513: SqlFragment = t___8511.accumulated();
        let q__975: Query = from(t___8509.clone()).left_join(t___8510.clone(), t___8513.clone());
        let mut t___8518: bool = Some(q__975.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__8508(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__8508 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8508())
        };
        test___64.assert(t___8518, fn__8508.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__1502() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let mut t___8497: SafeIdentifier = sid__461("orders");
        let mut t___8498: SafeIdentifier = sid__461("users");
        let mut t___8499: SqlBuilder = SqlBuilder::new();
        t___8499.append_safe("orders.user_id = users.id");
        let mut t___8501: SqlFragment = t___8499.accumulated();
        let q__977: Query = from(t___8497.clone()).right_join(t___8498.clone(), t___8501.clone());
        let mut t___8506: bool = Some(q__977.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__8496(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__8496 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8496())
        };
        test___65.assert(t___8506, fn__8496.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__1504() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let mut t___8485: SafeIdentifier = sid__461("users");
        let mut t___8486: SafeIdentifier = sid__461("orders");
        let mut t___8487: SqlBuilder = SqlBuilder::new();
        t___8487.append_safe("users.id = orders.user_id");
        let mut t___8489: SqlFragment = t___8487.accumulated();
        let q__979: Query = from(t___8485.clone()).full_join(t___8486.clone(), t___8489.clone());
        let mut t___8494: bool = Some(q__979.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__8484(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__8484 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8484())
        };
        test___66.assert(t___8494, fn__8484.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__1506() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let mut t___8468: SafeIdentifier = sid__461("users");
        let mut t___8469: SafeIdentifier = sid__461("orders");
        let mut t___8470: SqlBuilder = SqlBuilder::new();
        t___8470.append_safe("users.id = orders.user_id");
        let mut t___8472: SqlFragment = t___8470.accumulated();
        let mut t___8473: Query = from(t___8468.clone()).inner_join(t___8469.clone(), t___8472.clone());
        let mut t___8474: SafeIdentifier = sid__461("profiles");
        let mut t___8475: SqlBuilder = SqlBuilder::new();
        t___8475.append_safe("users.id = profiles.user_id");
        let q__981: Query = t___8473.left_join(t___8474.clone(), t___8475.accumulated());
        let mut t___8482: bool = Some(q__981.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__8467(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__8467 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8467())
        };
        test___67.assert(t___8482, fn__8467.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__1509() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let mut t___8449: SafeIdentifier;
        let mut t___8450: SafeIdentifier;
        let mut t___8451: SqlBuilder;
        let mut t___8453: SqlFragment;
        let mut t___8454: Query;
        let mut t___8455: SqlBuilder;
        let mut t___4407: Query;
        let q__983: Query;
        'ok___9735: {
            'orelse___1717: {
                t___8449 = sid__461("users");
                t___8450 = sid__461("orders");
                t___8451 = SqlBuilder::new();
                t___8451.append_safe("users.id = orders.user_id");
                t___8453 = t___8451.accumulated();
                t___8454 = from(t___8449.clone()).inner_join(t___8450.clone(), t___8453.clone());
                t___8455 = SqlBuilder::new();
                t___8455.append_safe("orders.total > ");
                t___8455.append_int32(100);
                t___4407 = match t___8454.r#where(t___8455.accumulated()).order_by(sid__461("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1717
                };
                q__983 = t___4407.clone();
                break 'ok___9735;
            }
            q__983 = panic!();
        }
        let mut t___8465: bool = Some(q__983.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__8448(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__8448 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8448())
        };
        test___68.assert(t___8465, fn__8448.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__1512() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let c__985: SqlFragment = col(sid__461("users"), sid__461("id"));
        let mut t___8446: bool = Some(c__985.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__8440(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__8440 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8440())
        };
        test___69.assert(t___8446, fn__8440.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__1513() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let onCond__987: SqlFragment = col(sid__461("users"), sid__461("id"));
        let b__988: SqlBuilder = SqlBuilder::new();
        b__988.append_fragment(onCond__987.clone());
        b__988.append_safe(" = ");
        b__988.append_fragment(col(sid__461("orders"), sid__461("user_id")));
        let mut t___8431: SafeIdentifier = sid__461("users");
        let mut t___8432: SafeIdentifier = sid__461("orders");
        let mut t___8433: SqlFragment = b__988.accumulated();
        let q__989: Query = from(t___8431.clone()).inner_join(t___8432.clone(), t___8433.clone());
        let mut t___8438: bool = Some(q__989.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__8420(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__8420 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8420())
        };
        test___70.assert(t___8438, fn__8420.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn orWhereBasic__1514() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let mut t___8409: SafeIdentifier = sid__461("users");
        let mut t___8410: SqlBuilder = SqlBuilder::new();
        t___8410.append_safe("status = ");
        t___8410.append_string("active");
        let mut t___8413: SqlFragment = t___8410.accumulated();
        let q__991: Query = from(t___8409.clone()).or_where(t___8413.clone());
        let mut t___8418: bool = Some(q__991.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE status = 'active'");
        #[derive(Clone)]
        struct ClosureGroup___80 {}
        impl ClosureGroup___80 {
            fn fn__8408(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orWhere basic".to_string());
            }
        }
        let closure_group = ClosureGroup___80 {};
        let fn__8408 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8408())
        };
        test___71.assert(t___8418, fn__8408.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn whereThenOrWhere__1516() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let mut t___8392: SafeIdentifier = sid__461("users");
        let mut t___8393: SqlBuilder = SqlBuilder::new();
        t___8393.append_safe("age > ");
        t___8393.append_int32(18);
        let mut t___8396: SqlFragment = t___8393.accumulated();
        let mut t___8397: Query = from(t___8392.clone()).r#where(t___8396.clone());
        let mut t___8398: SqlBuilder = SqlBuilder::new();
        t___8398.append_safe("vip = ");
        t___8398.append_boolean(true);
        let q__993: Query = t___8397.or_where(t___8398.accumulated());
        let mut t___8406: bool = Some(q__993.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___81 {}
        impl ClosureGroup___81 {
            fn fn__8391(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where then orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___81 {};
        let fn__8391 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8391())
        };
        test___72.assert(t___8406, fn__8391.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn multipleOrWhere__1519() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let mut t___8370: SafeIdentifier = sid__461("users");
        let mut t___8371: SqlBuilder = SqlBuilder::new();
        t___8371.append_safe("active = ");
        t___8371.append_boolean(true);
        let mut t___8374: SqlFragment = t___8371.accumulated();
        let mut t___8375: Query = from(t___8370.clone()).r#where(t___8374.clone());
        let mut t___8376: SqlBuilder = SqlBuilder::new();
        t___8376.append_safe("role = ");
        t___8376.append_string("admin");
        let mut t___8380: Query = t___8375.or_where(t___8376.accumulated());
        let mut t___8381: SqlBuilder = SqlBuilder::new();
        t___8381.append_safe("role = ");
        t___8381.append_string("moderator");
        let q__995: Query = t___8380.or_where(t___8381.accumulated());
        let mut t___8389: bool = Some(q__995.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'");
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__8369(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("multiple orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__8369 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8369())
        };
        test___73.assert(t___8389, fn__8369.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn mixedWhereAndOrWhere__1523() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let mut t___8348: SafeIdentifier = sid__461("users");
        let mut t___8349: SqlBuilder = SqlBuilder::new();
        t___8349.append_safe("age > ");
        t___8349.append_int32(18);
        let mut t___8352: SqlFragment = t___8349.accumulated();
        let mut t___8353: Query = from(t___8348.clone()).r#where(t___8352.clone());
        let mut t___8354: SqlBuilder = SqlBuilder::new();
        t___8354.append_safe("active = ");
        t___8354.append_boolean(true);
        let mut t___8358: Query = t___8353.r#where(t___8354.accumulated());
        let mut t___8359: SqlBuilder = SqlBuilder::new();
        t___8359.append_safe("vip = ");
        t___8359.append_boolean(true);
        let q__997: Query = t___8358.or_where(t___8359.accumulated());
        let mut t___8367: bool = Some(q__997.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__8347(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed where and orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__8347 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8347())
        };
        test___74.assert(t___8367, fn__8347.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn whereNull__1527() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let mut t___8339: SafeIdentifier = sid__461("users");
        let mut t___8340: SafeIdentifier = sid__461("deleted_at");
        let q__999: Query = from(t___8339.clone()).where_null(t___8340.clone());
        let mut t___8345: bool = Some(q__999.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___84 {}
        impl ClosureGroup___84 {
            fn fn__8338(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull".to_string());
            }
        }
        let closure_group = ClosureGroup___84 {};
        let fn__8338 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8338())
        };
        test___75.assert(t___8345, fn__8338.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNull__1528() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        let mut t___8330: SafeIdentifier = sid__461("users");
        let mut t___8331: SafeIdentifier = sid__461("email");
        let q__1001: Query = from(t___8330.clone()).where_not_null(t___8331.clone());
        let mut t___8336: bool = Some(q__1001.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email IS NOT NULL");
        #[derive(Clone)]
        struct ClosureGroup___85 {}
        impl ClosureGroup___85 {
            fn fn__8329(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull".to_string());
            }
        }
        let closure_group = ClosureGroup___85 {};
        let fn__8329 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8329())
        };
        test___76.assert(t___8336, fn__8329.clone());
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn whereNullChainedWithWhere__1529() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let mut t___8316: SafeIdentifier = sid__461("users");
        let mut t___8317: SqlBuilder = SqlBuilder::new();
        t___8317.append_safe("active = ");
        t___8317.append_boolean(true);
        let mut t___8320: SqlFragment = t___8317.accumulated();
        let q__1003: Query = from(t___8316.clone()).r#where(t___8320.clone()).where_null(sid__461("deleted_at"));
        let mut t___8327: bool = Some(q__1003.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn fn__8315(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull chained".to_string());
            }
        }
        let closure_group = ClosureGroup___86 {};
        let fn__8315 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8315())
        };
        test___77.assert(t___8327, fn__8315.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNullChainedWithOrWhere__1531() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let mut t___8302: SafeIdentifier = sid__461("users");
        let mut t___8303: SafeIdentifier = sid__461("deleted_at");
        let mut t___8304: Query = from(t___8302.clone()).where_null(t___8303.clone());
        let mut t___8305: SqlBuilder = SqlBuilder::new();
        t___8305.append_safe("role = ");
        t___8305.append_string("admin");
        let q__1005: Query = t___8304.or_where(t___8305.accumulated());
        let mut t___8313: bool = Some(q__1005.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'");
        #[derive(Clone)]
        struct ClosureGroup___87 {}
        impl ClosureGroup___87 {
            fn fn__8301(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull with orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___87 {};
        let fn__8301 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8301())
        };
        test___78.assert(t___8313, fn__8301.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithIntValues__1533() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let mut t___8290: SafeIdentifier = sid__461("users");
        let mut t___8291: SafeIdentifier = sid__461("id");
        let mut t___8292: SqlInt32 = SqlInt32::new(1);
        let mut t___8293: SqlInt32 = SqlInt32::new(2);
        let mut t___8294: SqlInt32 = SqlInt32::new(3);
        let q__1007: Query = from(t___8290.clone()).where_in(t___8291.clone(), [SqlPart::new(t___8292.clone()), SqlPart::new(t___8293.clone()), SqlPart::new(t___8294.clone())]);
        let mut t___8299: bool = Some(q__1007.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___88 {}
        impl ClosureGroup___88 {
            fn fn__8289(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn ints".to_string());
            }
        }
        let closure_group = ClosureGroup___88 {};
        let fn__8289 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8289())
        };
        test___79.assert(t___8299, fn__8289.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithStringValuesEscaping__1534() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let mut t___8279: SafeIdentifier = sid__461("users");
        let mut t___8280: SafeIdentifier = sid__461("name");
        let mut t___8281: SqlString = SqlString::new("Alice");
        let mut t___8282: SqlString = SqlString::new("Bob's");
        let q__1009: Query = from(t___8279.clone()).where_in(t___8280.clone(), [SqlPart::new(t___8281.clone()), SqlPart::new(t___8282.clone())]);
        let mut t___8287: bool = Some(q__1009.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')");
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__8278(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn strings".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__8278 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8278())
        };
        test___80.assert(t___8287, fn__8278.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithEmptyListProduces1_0__1535() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let mut t___8270: SafeIdentifier = sid__461("users");
        let mut t___8271: SafeIdentifier = sid__461("id");
        let q__1011: Query = from(t___8270.clone()).where_in(t___8271.clone(), []);
        let mut t___8276: bool = Some(q__1011.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___90 {}
        impl ClosureGroup___90 {
            fn fn__8269(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn empty".to_string());
            }
        }
        let closure_group = ClosureGroup___90 {};
        let fn__8269 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8269())
        };
        test___81.assert(t___8276, fn__8269.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn whereInChained__1536() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let mut t___8254: SafeIdentifier = sid__461("users");
        let mut t___8255: SqlBuilder = SqlBuilder::new();
        t___8255.append_safe("active = ");
        t___8255.append_boolean(true);
        let mut t___8258: SqlFragment = t___8255.accumulated();
        let q__1013: Query = from(t___8254.clone()).r#where(t___8258.clone()).where_in(sid__461("role"), [SqlPart::new(SqlString::new("admin")), SqlPart::new(SqlString::new("user"))]);
        let mut t___8267: bool = Some(q__1013.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')");
        #[derive(Clone)]
        struct ClosureGroup___91 {}
        impl ClosureGroup___91 {
            fn fn__8253(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn chained".to_string());
            }
        }
        let closure_group = ClosureGroup___91 {};
        let fn__8253 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8253())
        };
        test___82.assert(t___8267, fn__8253.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn whereInSingleElement__1538() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let mut t___8244: SafeIdentifier = sid__461("users");
        let mut t___8245: SafeIdentifier = sid__461("id");
        let mut t___8246: SqlInt32 = SqlInt32::new(42);
        let q__1015: Query = from(t___8244.clone()).where_in(t___8245.clone(), [SqlPart::new(t___8246.clone())]);
        let mut t___8251: bool = Some(q__1015.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___92 {}
        impl ClosureGroup___92 {
            fn fn__8243(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn single".to_string());
            }
        }
        let closure_group = ClosureGroup___92 {};
        let fn__8243 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8243())
        };
        test___83.assert(t___8251, fn__8243.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn whereNotBasic__1539() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let mut t___8232: SafeIdentifier = sid__461("users");
        let mut t___8233: SqlBuilder = SqlBuilder::new();
        t___8233.append_safe("active = ");
        t___8233.append_boolean(true);
        let mut t___8236: SqlFragment = t___8233.accumulated();
        let q__1017: Query = from(t___8232.clone()).where_not(t___8236.clone());
        let mut t___8241: bool = Some(q__1017.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE NOT (active = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___93 {}
        impl ClosureGroup___93 {
            fn fn__8231(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot".to_string());
            }
        }
        let closure_group = ClosureGroup___93 {};
        let fn__8231 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8231())
        };
        test___84.assert(t___8241, fn__8231.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn whereNotChained__1541() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let mut t___8215: SafeIdentifier = sid__461("users");
        let mut t___8216: SqlBuilder = SqlBuilder::new();
        t___8216.append_safe("age > ");
        t___8216.append_int32(18);
        let mut t___8219: SqlFragment = t___8216.accumulated();
        let mut t___8220: Query = from(t___8215.clone()).r#where(t___8219.clone());
        let mut t___8221: SqlBuilder = SqlBuilder::new();
        t___8221.append_safe("banned = ");
        t___8221.append_boolean(true);
        let q__1019: Query = t___8220.where_not(t___8221.accumulated());
        let mut t___8229: bool = Some(q__1019.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___94 {}
        impl ClosureGroup___94 {
            fn fn__8214(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot chained".to_string());
            }
        }
        let closure_group = ClosureGroup___94 {};
        let fn__8214 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8214())
        };
        test___85.assert(t___8229, fn__8214.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenIntegers__1544() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let mut t___8204: SafeIdentifier = sid__461("users");
        let mut t___8205: SafeIdentifier = sid__461("age");
        let mut t___8206: SqlInt32 = SqlInt32::new(18);
        let mut t___8207: SqlInt32 = SqlInt32::new(65);
        let q__1021: Query = from(t___8204.clone()).where_between(t___8205.clone(), SqlPart::new(t___8206.clone()), SqlPart::new(t___8207.clone()));
        let mut t___8212: bool = Some(q__1021.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age BETWEEN 18 AND 65");
        #[derive(Clone)]
        struct ClosureGroup___95 {}
        impl ClosureGroup___95 {
            fn fn__8203(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween ints".to_string());
            }
        }
        let closure_group = ClosureGroup___95 {};
        let fn__8203 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8203())
        };
        test___86.assert(t___8212, fn__8203.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenChained__1545() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let mut t___8188: SafeIdentifier = sid__461("users");
        let mut t___8189: SqlBuilder = SqlBuilder::new();
        t___8189.append_safe("active = ");
        t___8189.append_boolean(true);
        let mut t___8192: SqlFragment = t___8189.accumulated();
        let q__1023: Query = from(t___8188.clone()).r#where(t___8192.clone()).where_between(sid__461("age"), SqlPart::new(SqlInt32::new(21)), SqlPart::new(SqlInt32::new(30)));
        let mut t___8201: bool = Some(q__1023.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30");
        #[derive(Clone)]
        struct ClosureGroup___96 {}
        impl ClosureGroup___96 {
            fn fn__8187(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween chained".to_string());
            }
        }
        let closure_group = ClosureGroup___96 {};
        let fn__8187 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8187())
        };
        test___87.assert(t___8201, fn__8187.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeBasic__1547() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let mut t___8179: SafeIdentifier = sid__461("users");
        let mut t___8180: SafeIdentifier = sid__461("name");
        let q__1025: Query = from(t___8179.clone()).where_like(t___8180.clone(), "John%");
        let mut t___8185: bool = Some(q__1025.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE 'John%'");
        #[derive(Clone)]
        struct ClosureGroup___97 {}
        impl ClosureGroup___97 {
            fn fn__8178(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike".to_string());
            }
        }
        let closure_group = ClosureGroup___97 {};
        let fn__8178 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8178())
        };
        test___88.assert(t___8185, fn__8178.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn whereIlikeBasic__1548() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let mut t___8170: SafeIdentifier = sid__461("users");
        let mut t___8171: SafeIdentifier = sid__461("email");
        let q__1027: Query = from(t___8170.clone()).where_i_like(t___8171.clone(), "%@gmail.com");
        let mut t___8176: bool = Some(q__1027.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email ILIKE '%@gmail.com'");
        #[derive(Clone)]
        struct ClosureGroup___98 {}
        impl ClosureGroup___98 {
            fn fn__8169(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereILike".to_string());
            }
        }
        let closure_group = ClosureGroup___98 {};
        let fn__8169 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8169())
        };
        test___89.assert(t___8176, fn__8169.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWithInjectionAttempt__1549() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___90 = temper_std::testing::Test::new();
        let mut t___8156: SafeIdentifier = sid__461("users");
        let mut t___8157: SafeIdentifier = sid__461("name");
        let q__1029: Query = from(t___8156.clone()).where_like(t___8157.clone(), "'; DROP TABLE users; --");
        let s__1030: std::sync::Arc<String> = q__1029.to_sql().to_string();
        let mut t___8162: bool = temper_core::string::index_of( & s__1030, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___99 {
            s__1030: std::sync::Arc<String>
        }
        impl ClosureGroup___99 {
            fn fn__8155(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like injection escaped: {}", self.s__1030.clone()));
            }
        }
        let closure_group = ClosureGroup___99 {
            s__1030: s__1030.clone()
        };
        let fn__8155 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8155())
        };
        test___90.assert(t___8162, fn__8155.clone());
        let mut t___8166: bool = temper_core::string::index_of( & s__1030, "LIKE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___100 {
            s__1030: std::sync::Arc<String>
        }
        impl ClosureGroup___100 {
            fn fn__8154(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like structure intact: {}", self.s__1030.clone()));
            }
        }
        let closure_group = ClosureGroup___100 {
            s__1030: s__1030.clone()
        };
        let fn__8154 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8154())
        };
        test___90.assert(t___8166, fn__8154.clone());
        test___90.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWildcardPatterns__1550() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___91 = temper_std::testing::Test::new();
        let mut t___8146: SafeIdentifier = sid__461("users");
        let mut t___8147: SafeIdentifier = sid__461("name");
        let q__1032: Query = from(t___8146.clone()).where_like(t___8147.clone(), "%son%");
        let mut t___8152: bool = Some(q__1032.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE '%son%'");
        #[derive(Clone)]
        struct ClosureGroup___101 {}
        impl ClosureGroup___101 {
            fn fn__8145(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike wildcard".to_string());
            }
        }
        let closure_group = ClosureGroup___101 {};
        let fn__8145 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8145())
        };
        test___91.assert(t___8152, fn__8145.clone());
        test___91.soft_fail_to_hard()
    }
    #[test]
    fn countAllProducesCount__1551() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___92 = temper_std::testing::Test::new();
        let f__1034: SqlFragment = count_all();
        let mut t___8143: bool = Some(f__1034.to_string().as_str()) == Some("COUNT(*)");
        #[derive(Clone)]
        struct ClosureGroup___102 {}
        impl ClosureGroup___102 {
            fn fn__8139(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countAll".to_string());
            }
        }
        let closure_group = ClosureGroup___102 {};
        let fn__8139 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8139())
        };
        test___92.assert(t___8143, fn__8139.clone());
        test___92.soft_fail_to_hard()
    }
    #[test]
    fn countColProducesCountField__1552() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___93 = temper_std::testing::Test::new();
        let f__1036: SqlFragment = count_col(sid__461("id"));
        let mut t___8137: bool = Some(f__1036.to_string().as_str()) == Some("COUNT(id)");
        #[derive(Clone)]
        struct ClosureGroup___103 {}
        impl ClosureGroup___103 {
            fn fn__8132(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countCol".to_string());
            }
        }
        let closure_group = ClosureGroup___103 {};
        let fn__8132 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8132())
        };
        test___93.assert(t___8137, fn__8132.clone());
        test___93.soft_fail_to_hard()
    }
    #[test]
    fn sumColProducesSumField__1553() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___94 = temper_std::testing::Test::new();
        let f__1038: SqlFragment = sum_col(sid__461("amount"));
        let mut t___8130: bool = Some(f__1038.to_string().as_str()) == Some("SUM(amount)");
        #[derive(Clone)]
        struct ClosureGroup___104 {}
        impl ClosureGroup___104 {
            fn fn__8125(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("sumCol".to_string());
            }
        }
        let closure_group = ClosureGroup___104 {};
        let fn__8125 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8125())
        };
        test___94.assert(t___8130, fn__8125.clone());
        test___94.soft_fail_to_hard()
    }
    #[test]
    fn avgColProducesAvgField__1554() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___95 = temper_std::testing::Test::new();
        let f__1040: SqlFragment = avg_col(sid__461("price"));
        let mut t___8123: bool = Some(f__1040.to_string().as_str()) == Some("AVG(price)");
        #[derive(Clone)]
        struct ClosureGroup___105 {}
        impl ClosureGroup___105 {
            fn fn__8118(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("avgCol".to_string());
            }
        }
        let closure_group = ClosureGroup___105 {};
        let fn__8118 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8118())
        };
        test___95.assert(t___8123, fn__8118.clone());
        test___95.soft_fail_to_hard()
    }
    #[test]
    fn minColProducesMinField__1555() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___96 = temper_std::testing::Test::new();
        let f__1042: SqlFragment = min_col(sid__461("created_at"));
        let mut t___8116: bool = Some(f__1042.to_string().as_str()) == Some("MIN(created_at)");
        #[derive(Clone)]
        struct ClosureGroup___106 {}
        impl ClosureGroup___106 {
            fn fn__8111(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("minCol".to_string());
            }
        }
        let closure_group = ClosureGroup___106 {};
        let fn__8111 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8111())
        };
        test___96.assert(t___8116, fn__8111.clone());
        test___96.soft_fail_to_hard()
    }
    #[test]
    fn maxColProducesMaxField__1556() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___97 = temper_std::testing::Test::new();
        let f__1044: SqlFragment = max_col(sid__461("score"));
        let mut t___8109: bool = Some(f__1044.to_string().as_str()) == Some("MAX(score)");
        #[derive(Clone)]
        struct ClosureGroup___107 {}
        impl ClosureGroup___107 {
            fn fn__8104(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("maxCol".to_string());
            }
        }
        let closure_group = ClosureGroup___107 {};
        let fn__8104 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8104())
        };
        test___97.assert(t___8109, fn__8104.clone());
        test___97.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithAggregate__1557() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___98 = temper_std::testing::Test::new();
        let mut t___8096: SafeIdentifier = sid__461("orders");
        let mut t___8097: SqlFragment = count_all();
        let q__1046: Query = from(t___8096.clone()).select_expr([t___8097.clone()]);
        let mut t___8102: bool = Some(q__1046.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM orders");
        #[derive(Clone)]
        struct ClosureGroup___108 {}
        impl ClosureGroup___108 {
            fn fn__8095(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr count".to_string());
            }
        }
        let closure_group = ClosureGroup___108 {};
        let fn__8095 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8095())
        };
        test___98.assert(t___8102, fn__8095.clone());
        test___98.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithMultipleExpressions__1558() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___99 = temper_std::testing::Test::new();
        let nameFrag__1048: SqlFragment = col(sid__461("users"), sid__461("name"));
        let mut t___8087: SafeIdentifier = sid__461("users");
        let mut t___8088: SqlFragment = count_all();
        let q__1049: Query = from(t___8087.clone()).select_expr([nameFrag__1048.clone(), t___8088.clone()]);
        let mut t___8093: bool = Some(q__1049.to_sql().to_string().as_str()) == Some("SELECT users.name, COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___109 {}
        impl ClosureGroup___109 {
            fn fn__8083(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr multi".to_string());
            }
        }
        let closure_group = ClosureGroup___109 {};
        let fn__8083 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8083())
        };
        test___99.assert(t___8093, fn__8083.clone());
        test___99.soft_fail_to_hard()
    }
    #[test]
    fn selectExprOverridesSelectedFields__1559() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___100 = temper_std::testing::Test::new();
        let mut t___8072: SafeIdentifier = sid__461("users");
        let mut t___8073: SafeIdentifier = sid__461("id");
        let mut t___8074: SafeIdentifier = sid__461("name");
        let q__1051: Query = from(t___8072.clone()).select([t___8073.clone(), t___8074.clone()]).select_expr([count_all()]);
        let mut t___8081: bool = Some(q__1051.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___110 {}
        impl ClosureGroup___110 {
            fn fn__8071(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr overrides select".to_string());
            }
        }
        let closure_group = ClosureGroup___110 {};
        let fn__8071 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8071())
        };
        test___100.assert(t___8081, fn__8071.clone());
        test___100.soft_fail_to_hard()
    }
    #[test]
    fn groupBySingleField__1560() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___101 = temper_std::testing::Test::new();
        let mut t___8058: SafeIdentifier = sid__461("orders");
        let mut t___8061: SqlFragment = col(sid__461("orders"), sid__461("status"));
        let mut t___8062: SqlFragment = count_all();
        let q__1053: Query = from(t___8058.clone()).select_expr([t___8061.clone(), t___8062.clone()]).group_by(sid__461("status"));
        let mut t___8069: bool = Some(q__1053.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status");
        #[derive(Clone)]
        struct ClosureGroup___111 {}
        impl ClosureGroup___111 {
            fn fn__8057(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy single".to_string());
            }
        }
        let closure_group = ClosureGroup___111 {};
        let fn__8057 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8057())
        };
        test___101.assert(t___8069, fn__8057.clone());
        test___101.soft_fail_to_hard()
    }
    #[test]
    fn groupByMultipleFields__1561() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___102 = temper_std::testing::Test::new();
        let mut t___8047: SafeIdentifier = sid__461("orders");
        let mut t___8048: SafeIdentifier = sid__461("status");
        let q__1055: Query = from(t___8047.clone()).group_by(t___8048.clone()).group_by(sid__461("category"));
        let mut t___8055: bool = Some(q__1055.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status, category");
        #[derive(Clone)]
        struct ClosureGroup___112 {}
        impl ClosureGroup___112 {
            fn fn__8046(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy multiple".to_string());
            }
        }
        let closure_group = ClosureGroup___112 {};
        let fn__8046 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8046())
        };
        test___102.assert(t___8055, fn__8046.clone());
        test___102.soft_fail_to_hard()
    }
    #[test]
    fn havingBasic__1562() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___103 = temper_std::testing::Test::new();
        let mut t___8028: SafeIdentifier = sid__461("orders");
        let mut t___8031: SqlFragment = col(sid__461("orders"), sid__461("status"));
        let mut t___8032: SqlFragment = count_all();
        let mut t___8035: Query = from(t___8028.clone()).select_expr([t___8031.clone(), t___8032.clone()]).group_by(sid__461("status"));
        let mut t___8036: SqlBuilder = SqlBuilder::new();
        t___8036.append_safe("COUNT(*) > ");
        t___8036.append_int32(5);
        let q__1057: Query = t___8035.having(t___8036.accumulated());
        let mut t___8044: bool = Some(q__1057.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status HAVING COUNT(*) > 5");
        #[derive(Clone)]
        struct ClosureGroup___113 {}
        impl ClosureGroup___113 {
            fn fn__8027(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("having basic".to_string());
            }
        }
        let closure_group = ClosureGroup___113 {};
        let fn__8027 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8027())
        };
        test___103.assert(t___8044, fn__8027.clone());
        test___103.soft_fail_to_hard()
    }
    #[test]
    fn orHaving__1564() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___104 = temper_std::testing::Test::new();
        let mut t___8009: SafeIdentifier = sid__461("orders");
        let mut t___8010: SafeIdentifier = sid__461("status");
        let mut t___8011: Query = from(t___8009.clone()).group_by(t___8010.clone());
        let mut t___8012: SqlBuilder = SqlBuilder::new();
        t___8012.append_safe("COUNT(*) > ");
        t___8012.append_int32(5);
        let mut t___8016: Query = t___8011.having(t___8012.accumulated());
        let mut t___8017: SqlBuilder = SqlBuilder::new();
        t___8017.append_safe("SUM(total) > ");
        t___8017.append_int32(1000);
        let q__1059: Query = t___8016.or_having(t___8017.accumulated());
        let mut t___8025: bool = Some(q__1059.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status HAVING COUNT(*) > 5 OR SUM(total) > 1000");
        #[derive(Clone)]
        struct ClosureGroup___114 {}
        impl ClosureGroup___114 {
            fn fn__8008(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orHaving".to_string());
            }
        }
        let closure_group = ClosureGroup___114 {};
        let fn__8008 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8008())
        };
        test___104.assert(t___8025, fn__8008.clone());
        test___104.soft_fail_to_hard()
    }
    #[test]
    fn distinctBasic__1567() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___105 = temper_std::testing::Test::new();
        let mut t___7999: SafeIdentifier = sid__461("users");
        let mut t___8000: SafeIdentifier = sid__461("name");
        let q__1061: Query = from(t___7999.clone()).select([t___8000.clone()]).distinct();
        let mut t___8006: bool = Some(q__1061.to_sql().to_string().as_str()) == Some("SELECT DISTINCT name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___115 {}
        impl ClosureGroup___115 {
            fn fn__7998(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct".to_string());
            }
        }
        let closure_group = ClosureGroup___115 {};
        let fn__7998 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7998())
        };
        test___105.assert(t___8006, fn__7998.clone());
        test___105.soft_fail_to_hard()
    }
    #[test]
    fn distinctWithWhere__1568() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___106 = temper_std::testing::Test::new();
        let mut t___7984: SafeIdentifier = sid__461("users");
        let mut t___7985: SafeIdentifier = sid__461("email");
        let mut t___7986: Query = from(t___7984.clone()).select([t___7985.clone()]);
        let mut t___7987: SqlBuilder = SqlBuilder::new();
        t___7987.append_safe("active = ");
        t___7987.append_boolean(true);
        let q__1063: Query = t___7986.r#where(t___7987.accumulated()).distinct();
        let mut t___7996: bool = Some(q__1063.to_sql().to_string().as_str()) == Some("SELECT DISTINCT email FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___116 {}
        impl ClosureGroup___116 {
            fn fn__7983(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct with where".to_string());
            }
        }
        let closure_group = ClosureGroup___116 {};
        let fn__7983 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7983())
        };
        test___106.assert(t___7996, fn__7983.clone());
        test___106.soft_fail_to_hard()
    }
    #[test]
    fn countSqlBare__1570() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___107 = temper_std::testing::Test::new();
        let q__1065: Query = from(sid__461("users"));
        let mut t___7981: bool = Some(q__1065.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___117 {}
        impl ClosureGroup___117 {
            fn fn__7976(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql bare".to_string());
            }
        }
        let closure_group = ClosureGroup___117 {};
        let fn__7976 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7976())
        };
        test___107.assert(t___7981, fn__7976.clone());
        test___107.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithWhere__1571() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___108 = temper_std::testing::Test::new();
        let mut t___7965: SafeIdentifier = sid__461("users");
        let mut t___7966: SqlBuilder = SqlBuilder::new();
        t___7966.append_safe("active = ");
        t___7966.append_boolean(true);
        let mut t___7969: SqlFragment = t___7966.accumulated();
        let q__1067: Query = from(t___7965.clone()).r#where(t___7969.clone());
        let mut t___7974: bool = Some(q__1067.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___118 {}
        impl ClosureGroup___118 {
            fn fn__7964(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with where".to_string());
            }
        }
        let closure_group = ClosureGroup___118 {};
        let fn__7964 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7964())
        };
        test___108.assert(t___7974, fn__7964.clone());
        test___108.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithJoin__1573() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___109 = temper_std::testing::Test::new();
        let mut t___7948: SafeIdentifier = sid__461("users");
        let mut t___7949: SafeIdentifier = sid__461("orders");
        let mut t___7950: SqlBuilder = SqlBuilder::new();
        t___7950.append_safe("users.id = orders.user_id");
        let mut t___7952: SqlFragment = t___7950.accumulated();
        let mut t___7953: Query = from(t___7948.clone()).inner_join(t___7949.clone(), t___7952.clone());
        let mut t___7954: SqlBuilder = SqlBuilder::new();
        t___7954.append_safe("orders.total > ");
        t___7954.append_int32(100);
        let q__1069: Query = t___7953.r#where(t___7954.accumulated());
        let mut t___7962: bool = Some(q__1069.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100");
        #[derive(Clone)]
        struct ClosureGroup___119 {}
        impl ClosureGroup___119 {
            fn fn__7947(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with join".to_string());
            }
        }
        let closure_group = ClosureGroup___119 {};
        let fn__7947 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7947())
        };
        test___109.assert(t___7962, fn__7947.clone());
        test___109.soft_fail_to_hard()
    }
    #[test]
    fn countSqlDropsOrderByLimitOffset__1576() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___110 = temper_std::testing::Test::new();
        let mut t___7934: SafeIdentifier;
        let mut t___7935: SqlBuilder;
        let mut t___7938: SqlFragment;
        let mut t___3983: Query;
        let mut t___3984: Query;
        let q__1071: Query;
        'ok___9736: {
            'orelse___1718: {
                t___7934 = sid__461("users");
                t___7935 = SqlBuilder::new();
                t___7935.append_safe("active = ");
                t___7935.append_boolean(true);
                t___7938 = t___7935.accumulated();
                t___3983 = match from(t___7934.clone()).r#where(t___7938.clone()).order_by(sid__461("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1718
                };
                t___3984 = match t___3983.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1718
                };
                q__1071 = t___3984.clone();
                break 'ok___9736;
            }
            q__1071 = panic!();
        }
        let s__1072: std::sync::Arc<String> = q__1071.count_sql().to_string();
        let mut t___7945: bool = Some(s__1072.as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___120 {
            s__1072: std::sync::Arc<String>
        }
        impl ClosureGroup___120 {
            fn fn__7933(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("countSql drops extras: {}", self.s__1072.clone()));
            }
        }
        let closure_group = ClosureGroup___120 {
            s__1072: s__1072.clone()
        };
        let fn__7933 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7933())
        };
        test___110.assert(t___7945, fn__7933.clone());
        test___110.soft_fail_to_hard()
    }
    #[test]
    fn fullAggregationQuery__1578() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___111 = temper_std::testing::Test::new();
        let mut t___7901: SafeIdentifier = sid__461("orders");
        let mut t___7904: SqlFragment = col(sid__461("orders"), sid__461("status"));
        let mut t___7905: SqlFragment = count_all();
        let mut t___7907: SqlFragment = sum_col(sid__461("total"));
        let mut t___7908: Query = from(t___7901.clone()).select_expr([t___7904.clone(), t___7905.clone(), t___7907.clone()]);
        let mut t___7909: SafeIdentifier = sid__461("users");
        let mut t___7910: SqlBuilder = SqlBuilder::new();
        t___7910.append_safe("orders.user_id = users.id");
        let mut t___7913: Query = t___7908.inner_join(t___7909.clone(), t___7910.accumulated());
        let mut t___7914: SqlBuilder = SqlBuilder::new();
        t___7914.append_safe("users.active = ");
        t___7914.append_boolean(true);
        let mut t___7920: Query = t___7913.r#where(t___7914.accumulated()).group_by(sid__461("status"));
        let mut t___7921: SqlBuilder = SqlBuilder::new();
        t___7921.append_safe("COUNT(*) > ");
        t___7921.append_int32(3);
        let q__1074: Query = t___7920.having(t___7921.accumulated()).order_by(sid__461("status"), true);
        let expected__1075: std::sync::Arc<String> = std::sync::Arc::new("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC".to_string());
        let mut t___7931: bool = Some(q__1074.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC");
        #[derive(Clone)]
        struct ClosureGroup___121 {}
        impl ClosureGroup___121 {
            fn fn__7900(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full aggregation".to_string());
            }
        }
        let closure_group = ClosureGroup___121 {};
        let fn__7900 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7900())
        };
        test___111.assert(t___7931, fn__7900.clone());
        test___111.soft_fail_to_hard()
    }
    #[test]
    fn unionSql__1582() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___112 = temper_std::testing::Test::new();
        let mut t___7883: SafeIdentifier = sid__461("users");
        let mut t___7884: SqlBuilder = SqlBuilder::new();
        t___7884.append_safe("role = ");
        t___7884.append_string("admin");
        let mut t___7887: SqlFragment = t___7884.accumulated();
        let a__1077: Query = from(t___7883.clone()).r#where(t___7887.clone());
        let mut t___7889: SafeIdentifier = sid__461("users");
        let mut t___7890: SqlBuilder = SqlBuilder::new();
        t___7890.append_safe("role = ");
        t___7890.append_string("moderator");
        let mut t___7893: SqlFragment = t___7890.accumulated();
        let b__1078: Query = from(t___7889.clone()).r#where(t___7893.clone());
        let s__1079: std::sync::Arc<String> = union_sql(a__1077.clone(), b__1078.clone()).to_string();
        let mut t___7898: bool = Some(s__1079.as_str()) == Some("(SELECT * FROM users WHERE role = 'admin') UNION (SELECT * FROM users WHERE role = 'moderator')");
        #[derive(Clone)]
        struct ClosureGroup___122 {
            s__1079: std::sync::Arc<String>
        }
        impl ClosureGroup___122 {
            fn fn__7882(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionSql: {}", self.s__1079.clone()));
            }
        }
        let closure_group = ClosureGroup___122 {
            s__1079: s__1079.clone()
        };
        let fn__7882 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7882())
        };
        test___112.assert(t___7898, fn__7882.clone());
        test___112.soft_fail_to_hard()
    }
    #[test]
    fn unionAllSql__1585() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___113 = temper_std::testing::Test::new();
        let mut t___7871: SafeIdentifier = sid__461("users");
        let mut t___7872: SafeIdentifier = sid__461("name");
        let a__1081: Query = from(t___7871.clone()).select([t___7872.clone()]);
        let mut t___7874: SafeIdentifier = sid__461("contacts");
        let mut t___7875: SafeIdentifier = sid__461("name");
        let b__1082: Query = from(t___7874.clone()).select([t___7875.clone()]);
        let s__1083: std::sync::Arc<String> = union_all_sql(a__1081.clone(), b__1082.clone()).to_string();
        let mut t___7880: bool = Some(s__1083.as_str()) == Some("(SELECT name FROM users) UNION ALL (SELECT name FROM contacts)");
        #[derive(Clone)]
        struct ClosureGroup___123 {
            s__1083: std::sync::Arc<String>
        }
        impl ClosureGroup___123 {
            fn fn__7870(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionAllSql: {}", self.s__1083.clone()));
            }
        }
        let closure_group = ClosureGroup___123 {
            s__1083: s__1083.clone()
        };
        let fn__7870 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7870())
        };
        test___113.assert(t___7880, fn__7870.clone());
        test___113.soft_fail_to_hard()
    }
    #[test]
    fn intersectSql__1586() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___114 = temper_std::testing::Test::new();
        let mut t___7859: SafeIdentifier = sid__461("users");
        let mut t___7860: SafeIdentifier = sid__461("email");
        let a__1085: Query = from(t___7859.clone()).select([t___7860.clone()]);
        let mut t___7862: SafeIdentifier = sid__461("subscribers");
        let mut t___7863: SafeIdentifier = sid__461("email");
        let b__1086: Query = from(t___7862.clone()).select([t___7863.clone()]);
        let s__1087: std::sync::Arc<String> = intersect_sql(a__1085.clone(), b__1086.clone()).to_string();
        let mut t___7868: bool = Some(s__1087.as_str()) == Some("(SELECT email FROM users) INTERSECT (SELECT email FROM subscribers)");
        #[derive(Clone)]
        struct ClosureGroup___124 {
            s__1087: std::sync::Arc<String>
        }
        impl ClosureGroup___124 {
            fn fn__7858(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("intersectSql: {}", self.s__1087.clone()));
            }
        }
        let closure_group = ClosureGroup___124 {
            s__1087: s__1087.clone()
        };
        let fn__7858 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7858())
        };
        test___114.assert(t___7868, fn__7858.clone());
        test___114.soft_fail_to_hard()
    }
    #[test]
    fn exceptSql__1587() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___115 = temper_std::testing::Test::new();
        let mut t___7847: SafeIdentifier = sid__461("users");
        let mut t___7848: SafeIdentifier = sid__461("id");
        let a__1089: Query = from(t___7847.clone()).select([t___7848.clone()]);
        let mut t___7850: SafeIdentifier = sid__461("banned");
        let mut t___7851: SafeIdentifier = sid__461("id");
        let b__1090: Query = from(t___7850.clone()).select([t___7851.clone()]);
        let s__1091: std::sync::Arc<String> = except_sql(a__1089.clone(), b__1090.clone()).to_string();
        let mut t___7856: bool = Some(s__1091.as_str()) == Some("(SELECT id FROM users) EXCEPT (SELECT id FROM banned)");
        #[derive(Clone)]
        struct ClosureGroup___125 {
            s__1091: std::sync::Arc<String>
        }
        impl ClosureGroup___125 {
            fn fn__7846(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exceptSql: {}", self.s__1091.clone()));
            }
        }
        let closure_group = ClosureGroup___125 {
            s__1091: s__1091.clone()
        };
        let fn__7846 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7846())
        };
        test___115.assert(t___7856, fn__7846.clone());
        test___115.soft_fail_to_hard()
    }
    #[test]
    fn subqueryWithAlias__1588() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___116 = temper_std::testing::Test::new();
        let mut t___7832: SafeIdentifier = sid__461("orders");
        let mut t___7833: SafeIdentifier = sid__461("user_id");
        let mut t___7834: Query = from(t___7832.clone()).select([t___7833.clone()]);
        let mut t___7835: SqlBuilder = SqlBuilder::new();
        t___7835.append_safe("total > ");
        t___7835.append_int32(100);
        let inner__1093: Query = t___7834.r#where(t___7835.accumulated());
        let s__1094: std::sync::Arc<String> = subquery(inner__1093.clone(), sid__461("big_orders")).to_string();
        let mut t___7844: bool = Some(s__1094.as_str()) == Some("(SELECT user_id FROM orders WHERE total > 100) AS big_orders");
        #[derive(Clone)]
        struct ClosureGroup___126 {
            s__1094: std::sync::Arc<String>
        }
        impl ClosureGroup___126 {
            fn fn__7831(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("subquery: {}", self.s__1094.clone()));
            }
        }
        let closure_group = ClosureGroup___126 {
            s__1094: s__1094.clone()
        };
        let fn__7831 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7831())
        };
        test___116.assert(t___7844, fn__7831.clone());
        test___116.soft_fail_to_hard()
    }
    #[test]
    fn existsSql__1590() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___117 = temper_std::testing::Test::new();
        let mut t___7821: SafeIdentifier = sid__461("orders");
        let mut t___7822: SqlBuilder = SqlBuilder::new();
        t___7822.append_safe("orders.user_id = users.id");
        let mut t___7824: SqlFragment = t___7822.accumulated();
        let inner__1096: Query = from(t___7821.clone()).r#where(t___7824.clone());
        let s__1097: std::sync::Arc<String> = exists_sql(inner__1096.clone()).to_string();
        let mut t___7829: bool = Some(s__1097.as_str()) == Some("EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___127 {
            s__1097: std::sync::Arc<String>
        }
        impl ClosureGroup___127 {
            fn fn__7820(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("existsSql: {}", self.s__1097.clone()));
            }
        }
        let closure_group = ClosureGroup___127 {
            s__1097: s__1097.clone()
        };
        let fn__7820 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7820())
        };
        test___117.assert(t___7829, fn__7820.clone());
        test___117.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubquery__1592() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___118 = temper_std::testing::Test::new();
        let mut t___7804: SafeIdentifier = sid__461("orders");
        let mut t___7805: SafeIdentifier = sid__461("user_id");
        let mut t___7806: Query = from(t___7804.clone()).select([t___7805.clone()]);
        let mut t___7807: SqlBuilder = SqlBuilder::new();
        t___7807.append_safe("total > ");
        t___7807.append_int32(1000);
        let sub__1099: Query = t___7806.r#where(t___7807.accumulated());
        let mut t___7812: SafeIdentifier = sid__461("users");
        let mut t___7813: SafeIdentifier = sid__461("id");
        let q__1100: Query = from(t___7812.clone()).where_in_subquery(t___7813.clone(), sub__1099.clone());
        let s__1101: std::sync::Arc<String> = q__1100.to_sql().to_string();
        let mut t___7818: bool = Some(s__1101.as_str()) == Some("SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 1000)");
        #[derive(Clone)]
        struct ClosureGroup___128 {
            s__1101: std::sync::Arc<String>
        }
        impl ClosureGroup___128 {
            fn fn__7803(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery: {}", self.s__1101.clone()));
            }
        }
        let closure_group = ClosureGroup___128 {
            s__1101: s__1101.clone()
        };
        let fn__7803 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7803())
        };
        test___118.assert(t___7818, fn__7803.clone());
        test___118.soft_fail_to_hard()
    }
    #[test]
    fn setOperationWithWhereOnEachSide__1594() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___119 = temper_std::testing::Test::new();
        let mut t___7781: SafeIdentifier = sid__461("users");
        let mut t___7782: SqlBuilder = SqlBuilder::new();
        t___7782.append_safe("age > ");
        t___7782.append_int32(18);
        let mut t___7785: SqlFragment = t___7782.accumulated();
        let mut t___7786: Query = from(t___7781.clone()).r#where(t___7785.clone());
        let mut t___7787: SqlBuilder = SqlBuilder::new();
        t___7787.append_safe("active = ");
        t___7787.append_boolean(true);
        let a__1103: Query = t___7786.r#where(t___7787.accumulated());
        let mut t___7792: SafeIdentifier = sid__461("users");
        let mut t___7793: SqlBuilder = SqlBuilder::new();
        t___7793.append_safe("role = ");
        t___7793.append_string("vip");
        let mut t___7796: SqlFragment = t___7793.accumulated();
        let b__1104: Query = from(t___7792.clone()).r#where(t___7796.clone());
        let s__1105: std::sync::Arc<String> = union_sql(a__1103.clone(), b__1104.clone()).to_string();
        let mut t___7801: bool = Some(s__1105.as_str()) == Some("(SELECT * FROM users WHERE age > 18 AND active = TRUE) UNION (SELECT * FROM users WHERE role = 'vip')");
        #[derive(Clone)]
        struct ClosureGroup___129 {
            s__1105: std::sync::Arc<String>
        }
        impl ClosureGroup___129 {
            fn fn__7780(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("union with where: {}", self.s__1105.clone()));
            }
        }
        let closure_group = ClosureGroup___129 {
            s__1105: s__1105.clone()
        };
        let fn__7780 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7780())
        };
        test___119.assert(t___7801, fn__7780.clone());
        test___119.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubqueryChainedWithWhere__1598() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___120 = temper_std::testing::Test::new();
        let mut t___7764: SafeIdentifier = sid__461("orders");
        let mut t___7765: SafeIdentifier = sid__461("user_id");
        let sub__1107: Query = from(t___7764.clone()).select([t___7765.clone()]);
        let mut t___7767: SafeIdentifier = sid__461("users");
        let mut t___7768: SqlBuilder = SqlBuilder::new();
        t___7768.append_safe("active = ");
        t___7768.append_boolean(true);
        let mut t___7771: SqlFragment = t___7768.accumulated();
        let q__1108: Query = from(t___7767.clone()).r#where(t___7771.clone()).where_in_subquery(sid__461("id"), sub__1107.clone());
        let s__1109: std::sync::Arc<String> = q__1108.to_sql().to_string();
        let mut t___7778: bool = Some(s__1109.as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND id IN (SELECT user_id FROM orders)");
        #[derive(Clone)]
        struct ClosureGroup___130 {
            s__1109: std::sync::Arc<String>
        }
        impl ClosureGroup___130 {
            fn fn__7763(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery chained: {}", self.s__1109.clone()));
            }
        }
        let closure_group = ClosureGroup___130 {
            s__1109: s__1109.clone()
        };
        let fn__7763 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7763())
        };
        test___120.assert(t___7778, fn__7763.clone());
        test___120.soft_fail_to_hard()
    }
    #[test]
    fn existsSqlUsedInWhere__1600() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___121 = temper_std::testing::Test::new();
        let mut t___7750: SafeIdentifier = sid__461("orders");
        let mut t___7751: SqlBuilder = SqlBuilder::new();
        t___7751.append_safe("orders.user_id = users.id");
        let mut t___7753: SqlFragment = t___7751.accumulated();
        let sub__1111: Query = from(t___7750.clone()).r#where(t___7753.clone());
        let mut t___7755: SafeIdentifier = sid__461("users");
        let mut t___7756: SqlFragment = exists_sql(sub__1111.clone());
        let q__1112: Query = from(t___7755.clone()).r#where(t___7756.clone());
        let s__1113: std::sync::Arc<String> = q__1112.to_sql().to_string();
        let mut t___7761: bool = Some(s__1113.as_str()) == Some("SELECT * FROM users WHERE EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___131 {
            s__1113: std::sync::Arc<String>
        }
        impl ClosureGroup___131 {
            fn fn__7749(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exists in where: {}", self.s__1113.clone()));
            }
        }
        let closure_group = ClosureGroup___131 {
            s__1113: s__1113.clone()
        };
        let fn__7749 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7749())
        };
        test___121.assert(t___7761, fn__7749.clone());
        test___121.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__1602() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___128 = temper_std::testing::Test::new();
        let mut t___3806: SafeIdentifier;
        let id__1151: SafeIdentifier;
        'ok___9737: {
            'orelse___1719: {
                t___3806 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___1719
                };
                id__1151 = t___3806.clone();
                break 'ok___9737;
            }
            id__1151 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7747: bool = Some(id__1151.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___132 {}
        impl ClosureGroup___132 {
            fn fn__7744(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___132 {};
        let fn__7744 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7744())
        };
        test___128.assert(t___7747, fn__7744.clone());
        test___128.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__1603() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___129 = temper_std::testing::Test::new();
        let didBubble__1153: bool;
        'ok___9738: {
            'orelse___1720: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___1720
                };
                didBubble__1153 = false;
                break 'ok___9738;
            }
            didBubble__1153 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___133 {}
        impl ClosureGroup___133 {
            fn fn__7741(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___133 {};
        let fn__7741 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7741())
        };
        test___129.assert(didBubble__1153, fn__7741.clone());
        test___129.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__1604() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___130 = temper_std::testing::Test::new();
        let didBubble__1155: bool;
        'ok___9739: {
            'orelse___1721: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___1721
                };
                didBubble__1155 = false;
                break 'ok___9739;
            }
            didBubble__1155 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___134 {}
        impl ClosureGroup___134 {
            fn fn__7738(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___134 {};
        let fn__7738 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7738())
        };
        test___130.assert(didBubble__1155, fn__7738.clone());
        test___130.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__1605() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___131 = temper_std::testing::Test::new();
        let cases__1157: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___135 {
            test___131: temper_std::testing::Test
        }
        impl ClosureGroup___135 {
            fn fn__7735(& self, c__1158: impl temper_core::ToArcString) {
                let c__1158 = c__1158.to_arc_string();
                let didBubble__1159: bool;
                'ok___9740: {
                    'orelse___1722: {
                        match safe_identifier(c__1158.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___1722
                        };
                        didBubble__1159 = false;
                        break 'ok___9740;
                    }
                    didBubble__1159 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___136 {
                    c__1158: std::sync::Arc<String>
                }
                impl ClosureGroup___136 {
                    fn fn__7732(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__1158.clone()));
                    }
                }
                let closure_group = ClosureGroup___136 {
                    c__1158: c__1158.clone()
                };
                let fn__7732 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__7732())
                };
                self.test___131.assert(didBubble__1159, fn__7732.clone());
            }
        }
        let closure_group = ClosureGroup___135 {
            test___131: test___131.clone()
        };
        let fn__7735 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1158: std::sync::Arc<String> | closure_group.fn__7735(c__1158))
        };
        temper_core::listed::list_for_each( & cases__1157, & ( * fn__7735.clone()));
        test___131.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__1606() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___132 = temper_std::testing::Test::new();
        let mut t___3783: SafeIdentifier;
        let mut t___3784: SafeIdentifier;
        let mut t___3785: SafeIdentifier;
        let mut t___3786: SafeIdentifier;
        let mut t___3789: SafeIdentifier;
        let mut t___3790: SafeIdentifier;
        let mut t___3794: FieldDef;
        'ok___9741: {
            'orelse___1723: {
                t___3783 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1723
                };
                t___3784 = t___3783.clone();
                break 'ok___9741;
            }
            t___3784 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___9742: {
            'orelse___1724: {
                t___3785 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1724
                };
                t___3786 = t___3785.clone();
                break 'ok___9742;
            }
            t___3786 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7722: StringField = StringField::new();
        let mut t___7723: FieldDef = FieldDef::new(t___3786.clone(), FieldType::new(t___7722.clone()), false);
        'ok___9743: {
            'orelse___1725: {
                t___3789 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1725
                };
                t___3790 = t___3789.clone();
                break 'ok___9743;
            }
            t___3790 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7724: IntField = IntField::new();
        let mut t___7725: FieldDef = FieldDef::new(t___3790.clone(), FieldType::new(t___7724.clone()), false);
        let td__1161: TableDef = TableDef::new(t___3784.clone(), [t___7723.clone(), t___7725.clone()]);
        let f__1162: FieldDef;
        'ok___9744: {
            'orelse___1726: {
                t___3794 = match td__1161.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1726
                };
                f__1162 = t___3794.clone();
                break 'ok___9744;
            }
            f__1162 = panic!();
        }
        let mut t___7730: bool = Some(f__1162.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___137 {}
        impl ClosureGroup___137 {
            fn fn__7721(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___137 {};
        let fn__7721 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7721())
        };
        test___132.assert(t___7730, fn__7721.clone());
        test___132.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__1607() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___133 = temper_std::testing::Test::new();
        let mut t___3774: SafeIdentifier;
        let mut t___3775: SafeIdentifier;
        let mut t___3776: SafeIdentifier;
        let mut t___3777: SafeIdentifier;
        'ok___9745: {
            'orelse___1727: {
                t___3774 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1727
                };
                t___3775 = t___3774.clone();
                break 'ok___9745;
            }
            t___3775 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___9746: {
            'orelse___1728: {
                t___3776 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1728
                };
                t___3777 = t___3776.clone();
                break 'ok___9746;
            }
            t___3777 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7716: StringField = StringField::new();
        let mut t___7717: FieldDef = FieldDef::new(t___3777.clone(), FieldType::new(t___7716.clone()), false);
        let td__1164: TableDef = TableDef::new(t___3775.clone(), [t___7717.clone()]);
        let didBubble__1165: bool;
        'ok___9747: {
            'orelse___1729: {
                match td__1164.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___1729
                };
                didBubble__1165 = false;
                break 'ok___9747;
            }
            didBubble__1165 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___138 {}
        impl ClosureGroup___138 {
            fn fn__7715(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___138 {};
        let fn__7715 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7715())
        };
        test___133.assert(didBubble__1165, fn__7715.clone());
        test___133.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__1608() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___134 = temper_std::testing::Test::new();
        let mut t___3762: SafeIdentifier;
        let mut t___3763: SafeIdentifier;
        let mut t___3766: SafeIdentifier;
        let mut t___3767: SafeIdentifier;
        'ok___9748: {
            'orelse___1730: {
                t___3762 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___1730
                };
                t___3763 = t___3762.clone();
                break 'ok___9748;
            }
            t___3763 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7704: StringField = StringField::new();
        let required__1167: FieldDef = FieldDef::new(t___3763.clone(), FieldType::new(t___7704.clone()), false);
        'ok___9749: {
            'orelse___1731: {
                t___3766 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___1731
                };
                t___3767 = t___3766.clone();
                break 'ok___9749;
            }
            t___3767 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7706: StringField = StringField::new();
        let optional__1168: FieldDef = FieldDef::new(t___3767.clone(), FieldType::new(t___7706.clone()), true);
        let mut t___7710: bool = ! required__1167.nullable();
        #[derive(Clone)]
        struct ClosureGroup___139 {}
        impl ClosureGroup___139 {
            fn fn__7703(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___139 {};
        let fn__7703 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7703())
        };
        test___134.assert(t___7710, fn__7703.clone());
        let mut t___7712: bool = optional__1168.nullable();
        #[derive(Clone)]
        struct ClosureGroup___140 {}
        impl ClosureGroup___140 {
            fn fn__7702(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___140 {};
        let fn__7702 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7702())
        };
        test___134.assert(t___7712, fn__7702.clone());
        test___134.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__1609() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___138 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___141 {}
        impl ClosureGroup___141 {
            fn build__1294(& self, name__1296: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1296 = name__1296.to_arc_string();
                let mut t___7684: SqlBuilder = SqlBuilder::new();
                t___7684.append_safe("select * from hi where name = ");
                t___7684.append_string(name__1296.clone());
                return t___7684.accumulated().to_string();
            }
            fn buildWrong__1295(& self, name__1298: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1298 = name__1298.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__1298.clone()));
            }
        }
        let closure_group = ClosureGroup___141 {};
        let build__1294 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1296: std::sync::Arc<String> | closure_group.build__1294(name__1296))
        };
        let buildWrong__1295 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1298: std::sync::Arc<String> | closure_group.buildWrong__1295(name__1298))
        };
        let actual___1611: std::sync::Arc<String> = build__1294(std::sync::Arc::new("world".to_string()));
        let mut t___7694: bool = Some(actual___1611.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___142 {
            actual___1611: std::sync::Arc<String>
        }
        impl ClosureGroup___142 {
            fn fn__7691(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___1611.clone()));
            }
        }
        let closure_group = ClosureGroup___142 {
            actual___1611: actual___1611.clone()
        };
        let fn__7691 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7691())
        };
        test___138.assert(t___7694, fn__7691.clone());
        let bobbyTables__1300: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___1613: std::sync::Arc<String> = build__1294(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___7698: bool = Some(actual___1613.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___143 {
            actual___1613: std::sync::Arc<String>
        }
        impl ClosureGroup___143 {
            fn fn__7690(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___1613.clone()));
            }
        }
        let closure_group = ClosureGroup___143 {
            actual___1613: actual___1613.clone()
        };
        let fn__7690 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7690())
        };
        test___138.assert(t___7698, fn__7690.clone());
        #[derive(Clone)]
        struct ClosureGroup___144 {}
        impl ClosureGroup___144 {
            fn fn__7689(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___144 {};
        let fn__7689 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7689())
        };
        test___138.assert(true, fn__7689.clone());
        test___138.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__1617() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___139 = temper_std::testing::Test::new();
        let mut t___7652: SqlBuilder = SqlBuilder::new();
        t___7652.append_safe("v = ");
        t___7652.append_string("");
        let actual___1618: std::sync::Arc<String> = t___7652.accumulated().to_string();
        let mut t___7658: bool = Some(actual___1618.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___145 {
            actual___1618: std::sync::Arc<String>
        }
        impl ClosureGroup___145 {
            fn fn__7651(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___1618.clone()));
            }
        }
        let closure_group = ClosureGroup___145 {
            actual___1618: actual___1618.clone()
        };
        let fn__7651 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7651())
        };
        test___139.assert(t___7658, fn__7651.clone());
        let mut t___7660: SqlBuilder = SqlBuilder::new();
        t___7660.append_safe("v = ");
        t___7660.append_string("a''b");
        let actual___1621: std::sync::Arc<String> = t___7660.accumulated().to_string();
        let mut t___7666: bool = Some(actual___1621.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___146 {
            actual___1621: std::sync::Arc<String>
        }
        impl ClosureGroup___146 {
            fn fn__7650(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___1621.clone()));
            }
        }
        let closure_group = ClosureGroup___146 {
            actual___1621: actual___1621.clone()
        };
        let fn__7650 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7650())
        };
        test___139.assert(t___7666, fn__7650.clone());
        let mut t___7668: SqlBuilder = SqlBuilder::new();
        t___7668.append_safe("v = ");
        t___7668.append_string("Hello 世界");
        let actual___1624: std::sync::Arc<String> = t___7668.accumulated().to_string();
        let mut t___7674: bool = Some(actual___1624.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___147 {
            actual___1624: std::sync::Arc<String>
        }
        impl ClosureGroup___147 {
            fn fn__7649(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___1624.clone()));
            }
        }
        let closure_group = ClosureGroup___147 {
            actual___1624: actual___1624.clone()
        };
        let fn__7649 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7649())
        };
        test___139.assert(t___7674, fn__7649.clone());
        let mut t___7676: SqlBuilder = SqlBuilder::new();
        t___7676.append_safe("v = ");
        t___7676.append_string("Line1\x0aLine2");
        let actual___1627: std::sync::Arc<String> = t___7676.accumulated().to_string();
        let mut t___7682: bool = Some(actual___1627.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___148 {
            actual___1627: std::sync::Arc<String>
        }
        impl ClosureGroup___148 {
            fn fn__7648(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___1627.clone()));
            }
        }
        let closure_group = ClosureGroup___148 {
            actual___1627: actual___1627.clone()
        };
        let fn__7648 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7648())
        };
        test___139.assert(t___7682, fn__7648.clone());
        test___139.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__1630() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___140 = temper_std::testing::Test::new();
        let mut t___3707: temper_std::temporal::Date;
        let mut t___7623: SqlBuilder = SqlBuilder::new();
        t___7623.append_safe("select ");
        t___7623.append_int32(42);
        t___7623.append_safe(", ");
        t___7623.append_int64(43);
        t___7623.append_safe(", ");
        t___7623.append_float64(19.99f64);
        t___7623.append_safe(", ");
        t___7623.append_boolean(true);
        t___7623.append_safe(", ");
        t___7623.append_boolean(false);
        let actual___1631: std::sync::Arc<String> = t___7623.accumulated().to_string();
        let mut t___7637: bool = Some(actual___1631.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___149 {
            actual___1631: std::sync::Arc<String>
        }
        impl ClosureGroup___149 {
            fn fn__7622(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___1631.clone()));
            }
        }
        let closure_group = ClosureGroup___149 {
            actual___1631: actual___1631.clone()
        };
        let fn__7622 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7622())
        };
        test___140.assert(t___7637, fn__7622.clone());
        let date__1303: temper_std::temporal::Date;
        'ok___9750: {
            'orelse___1732: {
                t___3707 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1732
                };
                date__1303 = t___3707.clone();
                break 'ok___9750;
            }
            date__1303 = panic!();
        }
        let mut t___7639: SqlBuilder = SqlBuilder::new();
        t___7639.append_safe("insert into t values (");
        t___7639.append_date(date__1303.clone());
        t___7639.append_safe(")");
        let actual___1634: std::sync::Arc<String> = t___7639.accumulated().to_string();
        let mut t___7646: bool = Some(actual___1634.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___150 {
            actual___1634: std::sync::Arc<String>
        }
        impl ClosureGroup___150 {
            fn fn__7621(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___1634.clone()));
            }
        }
        let closure_group = ClosureGroup___150 {
            actual___1634: actual___1634.clone()
        };
        let fn__7621 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7621())
        };
        test___140.assert(t___7646, fn__7621.clone());
        test___140.soft_fail_to_hard()
    }
    #[test]
    fn lists__1637() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___141 = temper_std::testing::Test::new();
        let mut t___3679: temper_std::temporal::Date;
        let mut t___3680: temper_std::temporal::Date;
        let mut t___3681: temper_std::temporal::Date;
        let mut t___3682: temper_std::temporal::Date;
        let mut t___7567: SqlBuilder = SqlBuilder::new();
        t___7567.append_safe("v IN (");
        t___7567.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___7567.append_safe(")");
        let actual___1638: std::sync::Arc<String> = t___7567.accumulated().to_string();
        let mut t___7574: bool = Some(actual___1638.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___151 {
            actual___1638: std::sync::Arc<String>
        }
        impl ClosureGroup___151 {
            fn fn__7566(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___1638.clone()));
            }
        }
        let closure_group = ClosureGroup___151 {
            actual___1638: actual___1638.clone()
        };
        let fn__7566 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7566())
        };
        test___141.assert(t___7574, fn__7566.clone());
        let mut t___7576: SqlBuilder = SqlBuilder::new();
        t___7576.append_safe("v IN (");
        t___7576.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___7576.append_safe(")");
        let actual___1641: std::sync::Arc<String> = t___7576.accumulated().to_string();
        let mut t___7583: bool = Some(actual___1641.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___152 {
            actual___1641: std::sync::Arc<String>
        }
        impl ClosureGroup___152 {
            fn fn__7565(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___1641.clone()));
            }
        }
        let closure_group = ClosureGroup___152 {
            actual___1641: actual___1641.clone()
        };
        let fn__7565 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7565())
        };
        test___141.assert(t___7583, fn__7565.clone());
        let mut t___7585: SqlBuilder = SqlBuilder::new();
        t___7585.append_safe("v IN (");
        t___7585.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___7585.append_safe(")");
        let actual___1644: std::sync::Arc<String> = t___7585.accumulated().to_string();
        let mut t___7592: bool = Some(actual___1644.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___153 {
            actual___1644: std::sync::Arc<String>
        }
        impl ClosureGroup___153 {
            fn fn__7564(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___1644.clone()));
            }
        }
        let closure_group = ClosureGroup___153 {
            actual___1644: actual___1644.clone()
        };
        let fn__7564 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7564())
        };
        test___141.assert(t___7592, fn__7564.clone());
        let mut t___7594: SqlBuilder = SqlBuilder::new();
        t___7594.append_safe("v IN (");
        t___7594.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___7594.append_safe(")");
        let actual___1647: std::sync::Arc<String> = t___7594.accumulated().to_string();
        let mut t___7601: bool = Some(actual___1647.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___154 {
            actual___1647: std::sync::Arc<String>
        }
        impl ClosureGroup___154 {
            fn fn__7563(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___1647.clone()));
            }
        }
        let closure_group = ClosureGroup___154 {
            actual___1647: actual___1647.clone()
        };
        let fn__7563 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7563())
        };
        test___141.assert(t___7601, fn__7563.clone());
        let mut t___7603: SqlBuilder = SqlBuilder::new();
        t___7603.append_safe("v IN (");
        t___7603.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___7603.append_safe(")");
        let actual___1650: std::sync::Arc<String> = t___7603.accumulated().to_string();
        let mut t___7610: bool = Some(actual___1650.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___155 {
            actual___1650: std::sync::Arc<String>
        }
        impl ClosureGroup___155 {
            fn fn__7562(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___1650.clone()));
            }
        }
        let closure_group = ClosureGroup___155 {
            actual___1650: actual___1650.clone()
        };
        let fn__7562 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7562())
        };
        test___141.assert(t___7610, fn__7562.clone());
        'ok___9751: {
            'orelse___1733: {
                t___3679 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___1733
                };
                t___3680 = t___3679.clone();
                break 'ok___9751;
            }
            t___3680 = panic!();
        }
        'ok___9752: {
            'orelse___1734: {
                t___3681 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1734
                };
                t___3682 = t___3681.clone();
                break 'ok___9752;
            }
            t___3682 = panic!();
        }
        let dates__1305: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___3680.clone(), t___3682.clone()]);
        let mut t___7612: SqlBuilder = SqlBuilder::new();
        t___7612.append_safe("v IN (");
        t___7612.append_date_list(temper_core::ToListed::to_listed(dates__1305.clone()));
        t___7612.append_safe(")");
        let actual___1653: std::sync::Arc<String> = t___7612.accumulated().to_string();
        let mut t___7619: bool = Some(actual___1653.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___156 {
            actual___1653: std::sync::Arc<String>
        }
        impl ClosureGroup___156 {
            fn fn__7561(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___1653.clone()));
            }
        }
        let closure_group = ClosureGroup___156 {
            actual___1653: actual___1653.clone()
        };
        let fn__7561 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7561())
        };
        test___141.assert(t___7619, fn__7561.clone());
        test___141.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__1656() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___142 = temper_std::testing::Test::new();
        let nan__1307: f64;
        nan__1307 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___7553: SqlBuilder = SqlBuilder::new();
        t___7553.append_safe("v = ");
        t___7553.append_float64(nan__1307);
        let actual___1657: std::sync::Arc<String> = t___7553.accumulated().to_string();
        let mut t___7559: bool = Some(actual___1657.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___157 {
            actual___1657: std::sync::Arc<String>
        }
        impl ClosureGroup___157 {
            fn fn__7552(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___1657.clone()));
            }
        }
        let closure_group = ClosureGroup___157 {
            actual___1657: actual___1657.clone()
        };
        let fn__7552 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7552())
        };
        test___142.assert(t___7559, fn__7552.clone());
        test___142.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__1660() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___143 = temper_std::testing::Test::new();
        let inf__1309: f64;
        inf__1309 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___7544: SqlBuilder = SqlBuilder::new();
        t___7544.append_safe("v = ");
        t___7544.append_float64(inf__1309);
        let actual___1661: std::sync::Arc<String> = t___7544.accumulated().to_string();
        let mut t___7550: bool = Some(actual___1661.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___158 {
            actual___1661: std::sync::Arc<String>
        }
        impl ClosureGroup___158 {
            fn fn__7543(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___1661.clone()));
            }
        }
        let closure_group = ClosureGroup___158 {
            actual___1661: actual___1661.clone()
        };
        let fn__7543 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7543())
        };
        test___143.assert(t___7550, fn__7543.clone());
        test___143.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__1664() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___144 = temper_std::testing::Test::new();
        let ninf__1311: f64;
        ninf__1311 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___7535: SqlBuilder = SqlBuilder::new();
        t___7535.append_safe("v = ");
        t___7535.append_float64(ninf__1311);
        let actual___1665: std::sync::Arc<String> = t___7535.accumulated().to_string();
        let mut t___7541: bool = Some(actual___1665.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___159 {
            actual___1665: std::sync::Arc<String>
        }
        impl ClosureGroup___159 {
            fn fn__7534(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___1665.clone()));
            }
        }
        let closure_group = ClosureGroup___159 {
            actual___1665: actual___1665.clone()
        };
        let fn__7534 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7534())
        };
        test___144.assert(t___7541, fn__7534.clone());
        test___144.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__1668() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___145 = temper_std::testing::Test::new();
        let mut t___7510: SqlBuilder = SqlBuilder::new();
        t___7510.append_safe("v = ");
        t___7510.append_float64(3.14f64);
        let actual___1669: std::sync::Arc<String> = t___7510.accumulated().to_string();
        let mut t___7516: bool = Some(actual___1669.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___160 {
            actual___1669: std::sync::Arc<String>
        }
        impl ClosureGroup___160 {
            fn fn__7509(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___1669.clone()));
            }
        }
        let closure_group = ClosureGroup___160 {
            actual___1669: actual___1669.clone()
        };
        let fn__7509 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7509())
        };
        test___145.assert(t___7516, fn__7509.clone());
        let mut t___7518: SqlBuilder = SqlBuilder::new();
        t___7518.append_safe("v = ");
        t___7518.append_float64(0.0f64);
        let actual___1672: std::sync::Arc<String> = t___7518.accumulated().to_string();
        let mut t___7524: bool = Some(actual___1672.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___161 {
            actual___1672: std::sync::Arc<String>
        }
        impl ClosureGroup___161 {
            fn fn__7508(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___1672.clone()));
            }
        }
        let closure_group = ClosureGroup___161 {
            actual___1672: actual___1672.clone()
        };
        let fn__7508 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7508())
        };
        test___145.assert(t___7524, fn__7508.clone());
        let mut t___7526: SqlBuilder = SqlBuilder::new();
        t___7526.append_safe("v = ");
        t___7526.append_float64(-42.5f64);
        let actual___1675: std::sync::Arc<String> = t___7526.accumulated().to_string();
        let mut t___7532: bool = Some(actual___1675.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___162 {
            actual___1675: std::sync::Arc<String>
        }
        impl ClosureGroup___162 {
            fn fn__7507(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___1675.clone()));
            }
        }
        let closure_group = ClosureGroup___162 {
            actual___1675: actual___1675.clone()
        };
        let fn__7507 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7507())
        };
        test___145.assert(t___7532, fn__7507.clone());
        test___145.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__1678() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___146 = temper_std::testing::Test::new();
        let mut t___3575: temper_std::temporal::Date;
        let d__1314: temper_std::temporal::Date;
        'ok___9753: {
            'orelse___1735: {
                t___3575 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___1735
                };
                d__1314 = t___3575.clone();
                break 'ok___9753;
            }
            d__1314 = panic!();
        }
        let mut t___7499: SqlBuilder = SqlBuilder::new();
        t___7499.append_safe("v = ");
        t___7499.append_date(d__1314.clone());
        let actual___1679: std::sync::Arc<String> = t___7499.accumulated().to_string();
        let mut t___7505: bool = Some(actual___1679.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___163 {
            actual___1679: std::sync::Arc<String>
        }
        impl ClosureGroup___163 {
            fn fn__7498(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___1679.clone()));
            }
        }
        let closure_group = ClosureGroup___163 {
            actual___1679: actual___1679.clone()
        };
        let fn__7498 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7498())
        };
        test___146.assert(t___7505, fn__7498.clone());
        test___146.soft_fail_to_hard()
    }
    #[test]
    fn nesting__1682() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___147 = temper_std::testing::Test::new();
        let name__1316: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___7467: SqlBuilder = SqlBuilder::new();
        t___7467.append_safe("where p.last_name = ");
        t___7467.append_string("Someone");
        let condition__1317: SqlFragment = t___7467.accumulated();
        let mut t___7471: SqlBuilder = SqlBuilder::new();
        t___7471.append_safe("select p.id from person p ");
        t___7471.append_fragment(condition__1317.clone());
        let actual___1684: std::sync::Arc<String> = t___7471.accumulated().to_string();
        let mut t___7477: bool = Some(actual___1684.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___164 {
            actual___1684: std::sync::Arc<String>
        }
        impl ClosureGroup___164 {
            fn fn__7466(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1684.clone()));
            }
        }
        let closure_group = ClosureGroup___164 {
            actual___1684: actual___1684.clone()
        };
        let fn__7466 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7466())
        };
        test___147.assert(t___7477, fn__7466.clone());
        let mut t___7479: SqlBuilder = SqlBuilder::new();
        t___7479.append_safe("select p.id from person p ");
        t___7479.append_part(SqlPart::new(condition__1317.to_source()));
        let actual___1687: std::sync::Arc<String> = t___7479.accumulated().to_string();
        let mut t___7486: bool = Some(actual___1687.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___165 {
            actual___1687: std::sync::Arc<String>
        }
        impl ClosureGroup___165 {
            fn fn__7465(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1687.clone()));
            }
        }
        let closure_group = ClosureGroup___165 {
            actual___1687: actual___1687.clone()
        };
        let fn__7465 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7465())
        };
        test___147.assert(t___7486, fn__7465.clone());
        let parts__1318: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___7490: SqlBuilder = SqlBuilder::new();
        t___7490.append_safe("select ");
        t___7490.append_part_list(parts__1318.clone());
        let actual___1690: std::sync::Arc<String> = t___7490.accumulated().to_string();
        let mut t___7496: bool = Some(actual___1690.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___166 {
            actual___1690: std::sync::Arc<String>
        }
        impl ClosureGroup___166 {
            fn fn__7464(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___1690.clone()));
            }
        }
        let closure_group = ClosureGroup___166 {
            actual___1690: actual___1690.clone()
        };
        let fn__7464 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7464())
        };
        test___147.assert(t___7496, fn__7464.clone());
        test___147.soft_fail_to_hard()
    }
    use super::*;
}
