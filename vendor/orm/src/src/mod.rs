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
    pub fn new(field__451: impl temper_core::ToArcString, message__452: impl temper_core::ToArcString) -> ChangesetError {
        let field__451 = field__451.to_arc_string();
        let message__452 = message__452.to_arc_string();
        let field;
        let message;
        field = field__451.clone();
        message = message__452.clone();
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
    fn cast(& self, allowedFields__462: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__465: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__468: SafeIdentifier, min__469: i32, max__470: i32) -> Changeset;
    fn validate_int(& self, field__473: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__476: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__479: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__482: SafeIdentifier) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__487: i32) -> temper_core::Result<SqlFragment>;
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
    pub fn cast(& self, allowedFields__503: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__503 = allowedFields__503.to_list();
        let mb__505: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__156: ChangesetImpl, mb__505: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__8709(& self, f__506: SafeIdentifier) {
                let mut t___8707: std::sync::Arc<String>;
                let mut t___8704: std::sync::Arc<String> = f__506.sql_value();
                let val__507: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__156.0.params, t___8704.clone(), std::sync::Arc::new("".to_string()));
                if ! val__507.is_empty() {
                    t___8707 = f__506.sql_value();
                    temper_core::MapBuilder::set( & self.mb__505, t___8707.clone(), val__507.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__156: self.clone(), mb__505: mb__505.clone()
        };
        let fn__8709 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__506: SafeIdentifier | closure_group.fn__8709(f__506))
        };
        temper_core::listed::list_for_each( & allowedFields__503, & ( * fn__8709.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__505), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__509: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__509 = fields__509.to_list();
        let return__273: Changeset;
        let mut t___8702: temper_core::List<ChangesetError>;
        let mut t___5006: TableDef;
        let mut t___5007: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___5008: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__510: {
            if ! self.0.is_valid {
                return__273 = Changeset::new(self.clone());
                break 'fn__510;
            }
            let eb__511: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__512: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__157: ChangesetImpl, eb__511: temper_core::ListBuilder<ChangesetError>, valid__512: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__8698(& self, f__513: SafeIdentifier) {
                    let mut t___8696: ChangesetError;
                    let mut t___8693: std::sync::Arc<String> = f__513.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__157.0.changes, t___8693.clone()) {
                        t___8696 = ChangesetError::new(f__513.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__511, t___8696.clone(), None);
                        {
                            * self.valid__512.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__157: self.clone(), eb__511: eb__511.clone(), valid__512: valid__512.clone()
            };
            let fn__8698 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__513: SafeIdentifier | closure_group.fn__8698(f__513))
            };
            temper_core::listed::list_for_each( & fields__509, & ( * fn__8698.clone()));
            t___5006 = self.0.table_def.clone();
            t___5007 = self.0.params.clone();
            t___5008 = self.0.changes.clone();
            t___8702 = temper_core::ListedTrait::to_list( & eb__511);
            return__273 = Changeset::new(ChangesetImpl::new(t___5006.clone(), t___5007.clone(), t___5008.clone(), t___8702.clone(), temper_core::read_locked( & valid__512)));
        }
        return return__273.clone();
    }
    pub fn validate_length(& self, field__515: SafeIdentifier, min__516: i32, max__517: i32) -> Changeset {
        let return__274: Changeset;
        let mut t___8680: std::sync::Arc<String>;
        let mut t___8691: temper_core::List<ChangesetError>;
        let mut t___4989: bool;
        let mut t___4995: TableDef;
        let mut t___4996: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4997: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__518: {
            if ! self.0.is_valid {
                return__274 = Changeset::new(self.clone());
                break 'fn__518;
            }
            t___8680 = field__515.sql_value();
            let val__519: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___8680.clone(), std::sync::Arc::new("".to_string()));
            let len__520: i32 = temper_core::string::count_between( & val__519, 0usize, val__519.len());
            if Some(len__520) < Some(min__516) {
                t___4989 = true;
            } else {
                t___4989 = Some(len__520) > Some(max__517);
            }
            if t___4989 {
                let msg__521: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__516, max__517));
                let eb__522: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__522, ChangesetError::new(field__515.sql_value(), msg__521.clone()), None);
                t___4995 = self.0.table_def.clone();
                t___4996 = self.0.params.clone();
                t___4997 = self.0.changes.clone();
                t___8691 = temper_core::ListedTrait::to_list( & eb__522);
                return__274 = Changeset::new(ChangesetImpl::new(t___4995.clone(), t___4996.clone(), t___4997.clone(), t___8691.clone(), false));
                break 'fn__518;
            }
            return__274 = Changeset::new(self.clone());
        }
        return return__274.clone();
    }
    pub fn validate_int(& self, field__524: SafeIdentifier) -> Changeset {
        let return__275: Changeset;
        let mut t___8671: std::sync::Arc<String>;
        let mut t___8678: temper_core::List<ChangesetError>;
        let mut t___4980: TableDef;
        let mut t___4981: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4982: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__525: {
            if ! self.0.is_valid {
                return__275 = Changeset::new(self.clone());
                break 'fn__525;
            }
            t___8671 = field__524.sql_value();
            let val__526: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___8671.clone(), std::sync::Arc::new("".to_string()));
            if val__526.is_empty() {
                return__275 = Changeset::new(self.clone());
                break 'fn__525;
            }
            let parseOk__527: bool;
            'ok___8814: {
                'orelse___1592: {
                    match temper_core::string::to_int( & val__526, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1592
                    };
                    parseOk__527 = true;
                    break 'ok___8814;
                }
                parseOk__527 = false;
            }
            if ! parseOk__527 {
                let eb__528: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__528, ChangesetError::new(field__524.sql_value(), "must be an integer"), None);
                t___4980 = self.0.table_def.clone();
                t___4981 = self.0.params.clone();
                t___4982 = self.0.changes.clone();
                t___8678 = temper_core::ListedTrait::to_list( & eb__528);
                return__275 = Changeset::new(ChangesetImpl::new(t___4980.clone(), t___4981.clone(), t___4982.clone(), t___8678.clone(), false));
                break 'fn__525;
            }
            return__275 = Changeset::new(self.clone());
        }
        return return__275.clone();
    }
    pub fn validate_int64(& self, field__530: SafeIdentifier) -> Changeset {
        let return__276: Changeset;
        let mut t___8662: std::sync::Arc<String>;
        let mut t___8669: temper_core::List<ChangesetError>;
        let mut t___4967: TableDef;
        let mut t___4968: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4969: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__531: {
            if ! self.0.is_valid {
                return__276 = Changeset::new(self.clone());
                break 'fn__531;
            }
            t___8662 = field__530.sql_value();
            let val__532: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___8662.clone(), std::sync::Arc::new("".to_string()));
            if val__532.is_empty() {
                return__276 = Changeset::new(self.clone());
                break 'fn__531;
            }
            let parseOk__533: bool;
            'ok___8816: {
                'orelse___1593: {
                    match temper_core::string::to_int64( & val__532, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1593
                    };
                    parseOk__533 = true;
                    break 'ok___8816;
                }
                parseOk__533 = false;
            }
            if ! parseOk__533 {
                let eb__534: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__534, ChangesetError::new(field__530.sql_value(), "must be a 64-bit integer"), None);
                t___4967 = self.0.table_def.clone();
                t___4968 = self.0.params.clone();
                t___4969 = self.0.changes.clone();
                t___8669 = temper_core::ListedTrait::to_list( & eb__534);
                return__276 = Changeset::new(ChangesetImpl::new(t___4967.clone(), t___4968.clone(), t___4969.clone(), t___8669.clone(), false));
                break 'fn__531;
            }
            return__276 = Changeset::new(self.clone());
        }
        return return__276.clone();
    }
    pub fn validate_float(& self, field__536: SafeIdentifier) -> Changeset {
        let return__277: Changeset;
        let mut t___8653: std::sync::Arc<String>;
        let mut t___8660: temper_core::List<ChangesetError>;
        let mut t___4954: TableDef;
        let mut t___4955: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4956: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__537: {
            if ! self.0.is_valid {
                return__277 = Changeset::new(self.clone());
                break 'fn__537;
            }
            t___8653 = field__536.sql_value();
            let val__538: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___8653.clone(), std::sync::Arc::new("".to_string()));
            if val__538.is_empty() {
                return__277 = Changeset::new(self.clone());
                break 'fn__537;
            }
            let parseOk__539: bool;
            'ok___8818: {
                'orelse___1594: {
                    match temper_core::string::to_float64( & val__538) {
                        Ok(x) => x,
                        _ => break 'orelse___1594
                    };
                    parseOk__539 = true;
                    break 'ok___8818;
                }
                parseOk__539 = false;
            }
            if ! parseOk__539 {
                let eb__540: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__540, ChangesetError::new(field__536.sql_value(), "must be a number"), None);
                t___4954 = self.0.table_def.clone();
                t___4955 = self.0.params.clone();
                t___4956 = self.0.changes.clone();
                t___8660 = temper_core::ListedTrait::to_list( & eb__540);
                return__277 = Changeset::new(ChangesetImpl::new(t___4954.clone(), t___4955.clone(), t___4956.clone(), t___8660.clone(), false));
                break 'fn__537;
            }
            return__277 = Changeset::new(self.clone());
        }
        return return__277.clone();
    }
    pub fn validate_bool(& self, field__542: SafeIdentifier) -> Changeset {
        let return__278: Changeset;
        let mut t___8644: std::sync::Arc<String>;
        let mut t___8651: temper_core::List<ChangesetError>;
        let mut t___4929: bool;
        let mut t___4930: bool;
        let mut t___4932: bool;
        let mut t___4933: bool;
        let mut t___4935: bool;
        let mut t___4941: TableDef;
        let mut t___4942: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___4943: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__543: {
            if ! self.0.is_valid {
                return__278 = Changeset::new(self.clone());
                break 'fn__543;
            }
            t___8644 = field__542.sql_value();
            let val__544: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___8644.clone(), std::sync::Arc::new("".to_string()));
            if val__544.is_empty() {
                return__278 = Changeset::new(self.clone());
                break 'fn__543;
            }
            let isTrue__545: bool;
            if Some(val__544.as_str()) == Some("true") {
                isTrue__545 = true;
            } else {
                if Some(val__544.as_str()) == Some("1") {
                    t___4930 = true;
                } else {
                    if Some(val__544.as_str()) == Some("yes") {
                        t___4929 = true;
                    } else {
                        t___4929 = Some(val__544.as_str()) == Some("on");
                    }
                    t___4930 = t___4929;
                }
                isTrue__545 = t___4930;
            }
            let isFalse__546: bool;
            if Some(val__544.as_str()) == Some("false") {
                isFalse__546 = true;
            } else {
                if Some(val__544.as_str()) == Some("0") {
                    t___4933 = true;
                } else {
                    if Some(val__544.as_str()) == Some("no") {
                        t___4932 = true;
                    } else {
                        t___4932 = Some(val__544.as_str()) == Some("off");
                    }
                    t___4933 = t___4932;
                }
                isFalse__546 = t___4933;
            }
            if ! isTrue__545 {
                t___4935 = ! isFalse__546;
            } else {
                t___4935 = false;
            }
            if t___4935 {
                let eb__547: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__547, ChangesetError::new(field__542.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___4941 = self.0.table_def.clone();
                t___4942 = self.0.params.clone();
                t___4943 = self.0.changes.clone();
                t___8651 = temper_core::ListedTrait::to_list( & eb__547);
                return__278 = Changeset::new(ChangesetImpl::new(t___4941.clone(), t___4942.clone(), t___4943.clone(), t___8651.clone(), false));
                break 'fn__543;
            }
            return__278 = Changeset::new(self.clone());
        }
        return return__278.clone();
    }
    fn parse_bool_sql_part(& self, val__549: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__549 = val__549.to_arc_string();
        let return__279: SqlBoolean;
        let mut t___4918: bool;
        let mut t___4919: bool;
        let mut t___4920: bool;
        let mut t___4922: bool;
        let mut t___4923: bool;
        let mut t___4924: bool;
        'fn__550: {
            if Some(val__549.as_str()) == Some("true") {
                t___4920 = true;
            } else {
                if Some(val__549.as_str()) == Some("1") {
                    t___4919 = true;
                } else {
                    if Some(val__549.as_str()) == Some("yes") {
                        t___4918 = true;
                    } else {
                        t___4918 = Some(val__549.as_str()) == Some("on");
                    }
                    t___4919 = t___4918;
                }
                t___4920 = t___4919;
            }
            if t___4920 {
                return__279 = SqlBoolean::new(true);
                break 'fn__550;
            }
            if Some(val__549.as_str()) == Some("false") {
                t___4924 = true;
            } else {
                if Some(val__549.as_str()) == Some("0") {
                    t___4923 = true;
                } else {
                    if Some(val__549.as_str()) == Some("no") {
                        t___4922 = true;
                    } else {
                        t___4922 = Some(val__549.as_str()) == Some("off");
                    }
                    t___4923 = t___4922;
                }
                t___4924 = t___4923;
            }
            if t___4924 {
                return__279 = SqlBoolean::new(false);
                break 'fn__550;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__279.clone());
    }
    fn value_to_sql_part(& self, fieldDef__552: FieldDef, val__553: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__553 = val__553.to_arc_string();
        let return__280: SqlPart;
        let mut t___4905: i32;
        let mut t___4908: i64;
        let mut t___4911: f64;
        let mut t___4916: temper_std::temporal::Date;
        'fn__554: {
            let ft__555: FieldType = fieldDef__552.field_type();
            if temper_core::is::<StringField>(ft__555.clone()) {
                return__280 = SqlPart::new(SqlString::new(val__553.clone()));
                break 'fn__554;
            }
            if temper_core::is::<IntField>(ft__555.clone()) {
                t___4905 = temper_core::string::to_int( & val__553, None) ? ;
                return__280 = SqlPart::new(SqlInt32::new(t___4905));
                break 'fn__554;
            }
            if temper_core::is::<Int64Field>(ft__555.clone()) {
                t___4908 = temper_core::string::to_int64( & val__553, None) ? ;
                return__280 = SqlPart::new(SqlInt64::new(t___4908));
                break 'fn__554;
            }
            if temper_core::is::<FloatField>(ft__555.clone()) {
                t___4911 = temper_core::string::to_float64( & val__553) ? ;
                return__280 = SqlPart::new(SqlFloat64::new(t___4911));
                break 'fn__554;
            }
            if temper_core::is::<BoolField>(ft__555.clone()) {
                return__280 = SqlPart::new(self.parse_bool_sql_part(val__553.clone()) ? );
                break 'fn__554;
            }
            if temper_core::is::<DateField>(ft__555.clone()) {
                t___4916 = temper_std::temporal::Date::from_iso_string(val__553.clone()) ? ;
                return__280 = SqlPart::new(SqlDate::new(t___4916.clone()));
                break 'fn__554;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__280.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___8592: i32;
        let mut t___8597: std::sync::Arc<String>;
        let mut t___8598: bool;
        let mut t___8603: i32;
        let mut t___8605: std::sync::Arc<String>;
        let mut t___8609: std::sync::Arc<String>;
        let mut t___8624: i32;
        let mut t___4869: bool;
        let mut t___4877: FieldDef;
        let mut t___4882: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__558: i32 = 0;
        'loop___8880: loop {
            t___8592 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__558) < Some(t___8592)) {
                break;
            }
            let f__559: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__558);
            if ! f__559.nullable() {
                t___8597 = f__559.name().sql_value();
                t___8598 = temper_core::MappedTrait::has( & self.0.changes, t___8597.clone());
                t___4869 = ! t___8598;
            } else {
                t___4869 = false;
            }
            if t___4869 {
                return Err(temper_core::Error::new());
            }
            i__558 = i__558.wrapping_add(1);
        }
        let pairs__560: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__560)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__561: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__562: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__563: i32 = 0;
        'loop___8881: loop {
            t___8603 = temper_core::ListedTrait::len( & pairs__560);
            if ! (Some(i__563) < Some(t___8603)) {
                break;
            }
            let pair__564: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__560, i__563);
            t___8605 = pair__564.key();
            t___4877 = self.0.table_def.field(t___8605.clone()) ? ;
            let fd__565: FieldDef = t___4877.clone();
            temper_core::listed::add( & colNames__561, fd__565.name().sql_value(), None);
            t___8609 = pair__564.value();
            t___4882 = self.value_to_sql_part(fd__565.clone(), t___8609.clone()) ? ;
            temper_core::listed::add( & valParts__562, t___4882.clone(), None);
            i__563 = i__563.wrapping_add(1);
        }
        let b__566: SqlBuilder = SqlBuilder::new();
        b__566.append_safe("INSERT INTO ");
        b__566.append_safe(self.0.table_def.table_name().sql_value());
        b__566.append_safe(" (");
        let mut t___8617: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__561);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__8590(& self, c__567: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__567 = c__567.to_arc_string();
                return c__567.clone();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__8590 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__567: std::sync::Arc<String> | closure_group.fn__8590(c__567))
        };
        b__566.append_safe(temper_core::listed::join( & t___8617, std::sync::Arc::new(", ".to_string()), & ( * fn__8590.clone())));
        b__566.append_safe(") VALUES (");
        b__566.append_part(temper_core::ListedTrait::get( & valParts__562, 0));
        let mut j__568: i32 = 1;
        'loop___8882: loop {
            t___8624 = temper_core::ListedTrait::len( & valParts__562);
            if ! (Some(j__568) < Some(t___8624)) {
                break;
            }
            b__566.append_safe(", ");
            b__566.append_part(temper_core::ListedTrait::get( & valParts__562, j__568));
            j__568 = j__568.wrapping_add(1);
        }
        b__566.append_safe(")");
        return Ok(b__566.accumulated());
    }
    pub fn to_update_sql(& self, id__570: i32) -> temper_core::Result<SqlFragment> {
        let mut t___8577: i32;
        let mut t___8580: std::sync::Arc<String>;
        let mut t___8585: std::sync::Arc<String>;
        let mut t___4850: FieldDef;
        let mut t___4856: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__572: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__572)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__573: SqlBuilder = SqlBuilder::new();
        b__573.append_safe("UPDATE ");
        b__573.append_safe(self.0.table_def.table_name().sql_value());
        b__573.append_safe(" SET ");
        let mut i__574: i32 = 0;
        'loop___8883: loop {
            t___8577 = temper_core::ListedTrait::len( & pairs__572);
            if ! (Some(i__574) < Some(t___8577)) {
                break;
            }
            if Some(i__574) > Some(0) {
                b__573.append_safe(", ");
            }
            let pair__575: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__572, i__574);
            t___8580 = pair__575.key();
            t___4850 = self.0.table_def.field(t___8580.clone()) ? ;
            let fd__576: FieldDef = t___4850.clone();
            b__573.append_safe(fd__576.name().sql_value());
            b__573.append_safe(" = ");
            t___8585 = pair__575.value();
            t___4856 = self.value_to_sql_part(fd__576.clone(), t___8585.clone()) ? ;
            b__573.append_part(t___4856.clone());
            i__574 = i__574.wrapping_add(1);
        }
        b__573.append_safe(" WHERE id = ");
        b__573.append_int32(id__570);
        return Ok(b__573.accumulated());
    }
    pub fn new(_tableDef__578: TableDef, _params__579: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__580: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__581: impl temper_core::ToList<ChangesetError>, _isValid__582: bool) -> ChangesetImpl {
        let _errors__581 = _errors__581.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__578.clone();
        params = _params__579.clone();
        changes = _changes__580.clone();
        errors = _errors__581.clone();
        is_valid = _isValid__582;
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
    fn cast(& self, allowedFields__503: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__503)
    }
    fn validate_required(& self, fields__509: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__509)
    }
    fn validate_length(& self, field__515: SafeIdentifier, min__516: i32, max__517: i32) -> Changeset {
        self.validate_length(field__515, min__516, max__517)
    }
    fn validate_int(& self, field__524: SafeIdentifier) -> Changeset {
        self.validate_int(field__524)
    }
    fn validate_int64(& self, field__530: SafeIdentifier) -> Changeset {
        self.validate_int64(field__530)
    }
    fn validate_float(& self, field__536: SafeIdentifier) -> Changeset {
        self.validate_float(field__536)
    }
    fn validate_bool(& self, field__542: SafeIdentifier) -> Changeset {
        self.validate_bool(field__542)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__570: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__570)
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
    pub fn new(joinType__695: JoinType, table__696: SafeIdentifier, onCondition__697: SqlFragment) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__695.clone();
        table = table__696.clone();
        on_condition = onCondition__697.clone();
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
    pub fn new(field__701: SafeIdentifier, ascending__702: bool) -> OrderClause {
        let field;
        let ascending;
        field = field__701.clone();
        ascending = ascending__702;
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
    pub fn new(_condition__713: SqlFragment) -> AndCondition {
        let condition;
        condition = _condition__713.clone();
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
    pub fn new(_condition__720: SqlFragment) -> OrCondition {
        let condition;
        condition = _condition__720.clone();
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
    pub fn r#where(& self, condition__733: SqlFragment) -> Query {
        let nb__735: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__735, WhereClause::new(AndCondition::new(condition__733.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__735), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn or_where(& self, condition__737: SqlFragment) -> Query {
        let nb__739: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__739, WhereClause::new(OrCondition::new(condition__737.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__739), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn where_null(& self, field__741: SafeIdentifier) -> Query {
        let b__743: SqlBuilder = SqlBuilder::new();
        b__743.append_safe(field__741.sql_value());
        b__743.append_safe(" IS NULL");
        let mut t___8176: SqlFragment = b__743.accumulated();
        return self.r#where(t___8176.clone());
    }
    pub fn where_not_null(& self, field__745: SafeIdentifier) -> Query {
        let b__747: SqlBuilder = SqlBuilder::new();
        b__747.append_safe(field__745.sql_value());
        b__747.append_safe(" IS NOT NULL");
        let mut t___8170: SqlFragment = b__747.accumulated();
        return self.r#where(t___8170.clone());
    }
    pub fn where_in(& self, field__749: SafeIdentifier, values__750: impl temper_core::ToList<SqlPart>) -> Query {
        let values__750 = values__750.to_list();
        let return__332: Query;
        let mut t___8151: SqlFragment;
        let mut t___8159: i32;
        let mut t___8164: SqlFragment;
        'fn__751: {
            if temper_core::ListedTrait::is_empty( & values__750) {
                let b__752: SqlBuilder = SqlBuilder::new();
                b__752.append_safe("1 = 0");
                t___8151 = b__752.accumulated();
                return__332 = self.r#where(t___8151.clone());
                break 'fn__751;
            }
            let b__753: SqlBuilder = SqlBuilder::new();
            b__753.append_safe(field__749.sql_value());
            b__753.append_safe(" IN (");
            b__753.append_part(temper_core::ListedTrait::get( & values__750, 0));
            let mut i__754: i32 = 1;
            'loop___8888: loop {
                t___8159 = temper_core::ListedTrait::len( & values__750);
                if ! (Some(i__754) < Some(t___8159)) {
                    break;
                }
                b__753.append_safe(", ");
                b__753.append_part(temper_core::ListedTrait::get( & values__750, i__754));
                i__754 = i__754.wrapping_add(1);
            }
            b__753.append_safe(")");
            t___8164 = b__753.accumulated();
            return__332 = self.r#where(t___8164.clone());
        }
        return return__332.clone();
    }
    pub fn where_not(& self, condition__756: SqlFragment) -> Query {
        let b__758: SqlBuilder = SqlBuilder::new();
        b__758.append_safe("NOT (");
        b__758.append_fragment(condition__756.clone());
        b__758.append_safe(")");
        let mut t___8146: SqlFragment = b__758.accumulated();
        return self.r#where(t___8146.clone());
    }
    pub fn where_between(& self, field__760: SafeIdentifier, low__761: SqlPart, high__762: SqlPart) -> Query {
        let b__764: SqlBuilder = SqlBuilder::new();
        b__764.append_safe(field__760.sql_value());
        b__764.append_safe(" BETWEEN ");
        b__764.append_part(low__761.clone());
        b__764.append_safe(" AND ");
        b__764.append_part(high__762.clone());
        let mut t___8140: SqlFragment = b__764.accumulated();
        return self.r#where(t___8140.clone());
    }
    pub fn where_like(& self, field__766: SafeIdentifier, pattern__767: impl temper_core::ToArcString) -> Query {
        let pattern__767 = pattern__767.to_arc_string();
        let b__769: SqlBuilder = SqlBuilder::new();
        b__769.append_safe(field__766.sql_value());
        b__769.append_safe(" LIKE ");
        b__769.append_string(pattern__767.clone());
        let mut t___8131: SqlFragment = b__769.accumulated();
        return self.r#where(t___8131.clone());
    }
    pub fn where_i_like(& self, field__771: SafeIdentifier, pattern__772: impl temper_core::ToArcString) -> Query {
        let pattern__772 = pattern__772.to_arc_string();
        let b__774: SqlBuilder = SqlBuilder::new();
        b__774.append_safe(field__771.sql_value());
        b__774.append_safe(" ILIKE ");
        b__774.append_string(pattern__772.clone());
        let mut t___8124: SqlFragment = b__774.accumulated();
        return self.r#where(t___8124.clone());
    }
    pub fn select(& self, fields__776: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__776 = fields__776.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__776.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn select_expr(& self, exprs__779: impl temper_core::ToList<SqlFragment>) -> Query {
        let exprs__779 = exprs__779.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, exprs__779.clone());
    }
    pub fn order_by(& self, field__782: SafeIdentifier, ascending__783: bool) -> Query {
        let nb__785: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__785, OrderClause::new(field__782.clone(), ascending__783), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__785), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn limit(& self, n__787: i32) -> temper_core::Result<Query> {
        if Some(n__787) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__787), self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone()));
    }
    pub fn offset(& self, n__790: i32) -> temper_core::Result<Query> {
        if Some(n__790) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__790), self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone()));
    }
    pub fn join(& self, joinType__793: JoinType, table__794: SafeIdentifier, onCondition__795: SqlFragment) -> Query {
        let nb__797: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__797, JoinClause::new(joinType__793.clone(), table__794.clone(), onCondition__795.clone()), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__797), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn inner_join(& self, table__799: SafeIdentifier, onCondition__800: SqlFragment) -> Query {
        let mut t___8094: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___8094.clone()), table__799.clone(), onCondition__800.clone());
    }
    pub fn left_join(& self, table__803: SafeIdentifier, onCondition__804: SqlFragment) -> Query {
        let mut t___8092: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___8092.clone()), table__803.clone(), onCondition__804.clone());
    }
    pub fn right_join(& self, table__807: SafeIdentifier, onCondition__808: SqlFragment) -> Query {
        let mut t___8090: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___8090.clone()), table__807.clone(), onCondition__808.clone());
    }
    pub fn full_join(& self, table__811: SafeIdentifier, onCondition__812: SqlFragment) -> Query {
        let mut t___8088: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___8088.clone()), table__811.clone(), onCondition__812.clone());
    }
    pub fn group_by(& self, field__815: SafeIdentifier) -> Query {
        let nb__817: temper_core::ListBuilder<SafeIdentifier> = temper_core::ListedTrait::to_list_builder( & self.0.group_by_fields);
        temper_core::listed::add( & nb__817, field__815.clone(), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), temper_core::ListedTrait::to_list( & nb__817), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn having(& self, condition__819: SqlFragment) -> Query {
        let nb__821: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__821, WhereClause::new(AndCondition::new(condition__819.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__821), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn or_having(& self, condition__823: SqlFragment) -> Query {
        let nb__825: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__825, WhereClause::new(OrCondition::new(condition__823.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__825), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn distinct(& self) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), true, self.0.select_exprs.clone());
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___7994: i32;
        let mut t___8013: i32;
        let mut t___8032: i32;
        let b__830: SqlBuilder = SqlBuilder::new();
        if self.0.is_distinct {
            b__830.append_safe("SELECT DISTINCT ");
        } else {
            b__830.append_safe("SELECT ");
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.select_exprs) {
            b__830.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, 0));
            let mut i__831: i32 = 1;
            'loop___8907: loop {
                t___7994 = temper_core::ListedTrait::len( & self.0.select_exprs);
                if ! (Some(i__831) < Some(t___7994)) {
                    break;
                }
                b__830.append_safe(", ");
                b__830.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, i__831));
                i__831 = i__831.wrapping_add(1);
            }
        } else {
            if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
                b__830.append_safe("*");
            } else {
                #[derive(Clone)]
                struct ClosureGroup___4 {}
                impl ClosureGroup___4 {
                    fn fn__7987(& self, f__832: SafeIdentifier) -> std::sync::Arc<String> {
                        return f__832.sql_value();
                    }
                }
                let closure_group = ClosureGroup___4 {};
                let fn__7987 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | f__832: SafeIdentifier | closure_group.fn__7987(f__832))
                };
                b__830.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__7987.clone())));
            }
        }
        b__830.append_safe(" FROM ");
        b__830.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___5 {
            b__830: SqlBuilder
        }
        impl ClosureGroup___5 {
            fn fn__7986(& self, jc__833: JoinClause) {
                self.b__830.append_safe(" ");
                let mut t___7974: std::sync::Arc<String> = jc__833.join_type().keyword();
                self.b__830.append_safe(t___7974.clone());
                self.b__830.append_safe(" ");
                let mut t___7978: std::sync::Arc<String> = jc__833.table().sql_value();
                self.b__830.append_safe(t___7978.clone());
                self.b__830.append_safe(" ON ");
                let mut t___7981: SqlFragment = jc__833.on_condition();
                self.b__830.append_fragment(t___7981.clone());
            }
        }
        let closure_group = ClosureGroup___5 {
            b__830: b__830.clone()
        };
        let fn__7986 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__833: JoinClause | closure_group.fn__7986(jc__833))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__7986.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__830.append_safe(" WHERE ");
            b__830.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__834: i32 = 1;
            'loop___8908: loop {
                t___8013 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__834) < Some(t___8013)) {
                    break;
                }
                b__830.append_safe(" ");
                b__830.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__834).keyword());
                b__830.append_safe(" ");
                b__830.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__834).condition());
                i__834 = i__834.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__830.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___6 {}
            impl ClosureGroup___6 {
                fn fn__7985(& self, f__835: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__835.sql_value();
                }
            }
            let closure_group = ClosureGroup___6 {};
            let fn__7985 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__835: SafeIdentifier | closure_group.fn__7985(f__835))
            };
            b__830.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__7985.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__830.append_safe(" HAVING ");
            b__830.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__836: i32 = 1;
            'loop___8909: loop {
                t___8032 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__836) < Some(t___8032)) {
                    break;
                }
                b__830.append_safe(" ");
                b__830.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__836).keyword());
                b__830.append_safe(" ");
                b__830.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__836).condition());
                i__836 = i__836.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__830.append_safe(" ORDER BY ");
            let mut first__837: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___7 {
                first__837: std::sync::Arc<std::sync::RwLock<bool>>, b__830: SqlBuilder
            }
            impl ClosureGroup___7 {
                fn fn__7984(& self, oc__838: OrderClause) {
                    let mut t___4304: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__837) {
                        self.b__830.append_safe(", ");
                    }
                    {
                        * self.first__837.write().unwrap() = false;
                    }
                    let mut t___7967: std::sync::Arc<String> = oc__838.field().sql_value();
                    self.b__830.append_safe(t___7967.clone());
                    if oc__838.ascending() {
                        t___4304 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___4304 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__830.append_safe(t___4304.clone());
                }
            }
            let closure_group = ClosureGroup___7 {
                first__837: first__837.clone(), b__830: b__830.clone()
            };
            let fn__7984 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | oc__838: OrderClause | closure_group.fn__7984(oc__838))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__7984.clone()));
        }
        let lv__839: Option<i32> = self.0.limit_val;
        if ! lv__839.is_none() {
            let lv___1638: i32 = lv__839.unwrap();
            b__830.append_safe(" LIMIT ");
            b__830.append_int32(lv___1638);
        }
        let ov__840: Option<i32> = self.0.offset_val;
        if ! ov__840.is_none() {
            let ov___1639: i32 = ov__840.unwrap();
            b__830.append_safe(" OFFSET ");
            b__830.append_int32(ov___1639);
        }
        return b__830.accumulated();
    }
    pub fn count_sql(& self) -> SqlFragment {
        let mut t___7936: i32;
        let mut t___7955: i32;
        let b__843: SqlBuilder = SqlBuilder::new();
        b__843.append_safe("SELECT COUNT(*) FROM ");
        b__843.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___8 {
            b__843: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__7924(& self, jc__844: JoinClause) {
                self.b__843.append_safe(" ");
                let mut t___7914: std::sync::Arc<String> = jc__844.join_type().keyword();
                self.b__843.append_safe(t___7914.clone());
                self.b__843.append_safe(" ");
                let mut t___7918: std::sync::Arc<String> = jc__844.table().sql_value();
                self.b__843.append_safe(t___7918.clone());
                self.b__843.append_safe(" ON ");
                let mut t___7921: SqlFragment = jc__844.on_condition();
                self.b__843.append_fragment(t___7921.clone());
            }
        }
        let closure_group = ClosureGroup___8 {
            b__843: b__843.clone()
        };
        let fn__7924 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__844: JoinClause | closure_group.fn__7924(jc__844))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__7924.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__843.append_safe(" WHERE ");
            b__843.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__845: i32 = 1;
            'loop___8912: loop {
                t___7936 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__845) < Some(t___7936)) {
                    break;
                }
                b__843.append_safe(" ");
                b__843.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__845).keyword());
                b__843.append_safe(" ");
                b__843.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__845).condition());
                i__845 = i__845.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__843.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___9 {}
            impl ClosureGroup___9 {
                fn fn__7923(& self, f__846: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__846.sql_value();
                }
            }
            let closure_group = ClosureGroup___9 {};
            let fn__7923 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__846: SafeIdentifier | closure_group.fn__7923(f__846))
            };
            b__843.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__7923.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__843.append_safe(" HAVING ");
            b__843.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__847: i32 = 1;
            'loop___8913: loop {
                t___7955 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__847) < Some(t___7955)) {
                    break;
                }
                b__843.append_safe(" ");
                b__843.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__847).keyword());
                b__843.append_safe(" ");
                b__843.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__847).condition());
                i__847 = i__847.wrapping_add(1);
            }
        }
        return b__843.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__849: i32) -> temper_core::Result<SqlFragment> {
        let return__353: SqlFragment;
        let mut t___4253: Query;
        if Some(defaultLimit__849) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__353 = self.to_sql();
        } else {
            t___4253 = self.limit(defaultLimit__849) ? ;
            return__353 = t___4253.to_sql();
        }
        return Ok(return__353.clone());
    }
    pub fn new(tableName__852: SafeIdentifier, conditions__853: impl temper_core::ToList<WhereClause>, selectedFields__854: impl temper_core::ToList<SafeIdentifier>, orderClauses__855: impl temper_core::ToList<OrderClause>, limitVal__856: Option<i32>, offsetVal__857: Option<i32>, joinClauses__858: impl temper_core::ToList<JoinClause>, groupByFields__859: impl temper_core::ToList<SafeIdentifier>, havingConditions__860: impl temper_core::ToList<WhereClause>, isDistinct__861: bool, selectExprs__862: impl temper_core::ToList<SqlFragment>) -> Query {
        let conditions__853 = conditions__853.to_list();
        let selectedFields__854 = selectedFields__854.to_list();
        let orderClauses__855 = orderClauses__855.to_list();
        let joinClauses__858 = joinClauses__858.to_list();
        let groupByFields__859 = groupByFields__859.to_list();
        let havingConditions__860 = havingConditions__860.to_list();
        let selectExprs__862 = selectExprs__862.to_list();
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
        table_name = tableName__852.clone();
        conditions = conditions__853.clone();
        selected_fields = selectedFields__854.clone();
        order_clauses = orderClauses__855.clone();
        limit_val = limitVal__856;
        offset_val = offsetVal__857;
        join_clauses = joinClauses__858.clone();
        group_by_fields = groupByFields__859.clone();
        having_conditions = havingConditions__860.clone();
        is_distinct = isDistinct__861;
        select_exprs = selectExprs__862.clone();
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
    pub fn new(_value__1036: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__1036 = _value__1036.to_arc_string();
        let value;
        value = _value__1036.clone();
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
    pub fn new(name__1054: SafeIdentifier, fieldType__1055: FieldType, nullable__1056: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__1054.clone();
        field_type = fieldType__1055.clone();
        nullable = nullable__1056;
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
    pub fn field(& self, name__1060: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__1060 = name__1060.to_arc_string();
        let return__389: FieldDef;
        'fn__1061: {
            let this__5239: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__5240: i32 = temper_core::ListedTrait::len( & this__5239);
            let mut i__5241: i32 = 0;
            'loop___8918: while Some(i__5241) < Some(n__5240) {
                let el__5242: FieldDef = temper_core::ListedTrait::get( & this__5239, i__5241);
                i__5241 = i__5241.wrapping_add(1);
                let f__1062: FieldDef = el__5242.clone();
                if Some(f__1062.name().sql_value().as_str()) == Some(name__1060.as_str()) {
                    return__389 = f__1062.clone();
                    break 'fn__1061;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__389.clone());
    }
    pub fn new(tableName__1064: SafeIdentifier, fields__1065: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__1065 = fields__1065.to_list();
        let table_name;
        let fields;
        table_name = tableName__1064.clone();
        fields = fields__1065.clone();
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
    pub fn append_safe(& self, sqlSource__1087: impl temper_core::ToArcString) {
        let sqlSource__1087 = sqlSource__1087.to_arc_string();
        let mut t___8767: SqlSource = SqlSource::new(sqlSource__1087.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___8767.clone()), None);
    }
    pub fn append_fragment(& self, fragment__1090: SqlFragment) {
        let mut t___8765: temper_core::List<SqlPart> = fragment__1090.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___8765.clone()), None);
    }
    pub fn append_part(& self, part__1093: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__1093.clone(), None);
    }
    pub fn append_part_list(& self, values__1096: impl temper_core::ToList<SqlPart>) {
        let values__1096 = values__1096.to_list();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__211: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__8761(& self, x__1098: SqlPart) {
                self.this__211.append_part(x__1098.clone());
            }
        }
        let closure_group = ClosureGroup___10 {
            this__211: self.clone()
        };
        let fn__8761 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1098: SqlPart | closure_group.fn__8761(x__1098))
        };
        self.append_list(temper_core::ToListed::to_listed(values__1096.clone()), fn__8761.clone());
    }
    pub fn append_boolean(& self, value__1100: bool) {
        let mut t___8758: SqlBoolean = SqlBoolean::new(value__1100);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___8758.clone()), None);
    }
    pub fn append_boolean_list(& self, values__1103: impl temper_core::ToListed<bool>) {
        let values__1103 = values__1103.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__213: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__8755(& self, x__1105: bool) {
                self.this__213.append_boolean(x__1105);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__213: self.clone()
        };
        let fn__8755 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1105: bool | closure_group.fn__8755(x__1105))
        };
        self.append_list(values__1103.clone(), fn__8755.clone());
    }
    pub fn append_date(& self, value__1107: temper_std::temporal::Date) {
        let mut t___8752: SqlDate = SqlDate::new(value__1107.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___8752.clone()), None);
    }
    pub fn append_date_list(& self, values__1110: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__1110 = values__1110.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__215: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__8749(& self, x__1112: temper_std::temporal::Date) {
                self.this__215.append_date(x__1112.clone());
            }
        }
        let closure_group = ClosureGroup___12 {
            this__215: self.clone()
        };
        let fn__8749 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1112: temper_std::temporal::Date | closure_group.fn__8749(x__1112))
        };
        self.append_list(values__1110.clone(), fn__8749.clone());
    }
    pub fn append_float64(& self, value__1114: f64) {
        let mut t___8746: SqlFloat64 = SqlFloat64::new(value__1114);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___8746.clone()), None);
    }
    pub fn append_float64_list(& self, values__1117: impl temper_core::ToListed<f64>) {
        let values__1117 = values__1117.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__217: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__8743(& self, x__1119: f64) {
                self.this__217.append_float64(x__1119);
            }
        }
        let closure_group = ClosureGroup___13 {
            this__217: self.clone()
        };
        let fn__8743 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1119: f64 | closure_group.fn__8743(x__1119))
        };
        self.append_list(values__1117.clone(), fn__8743.clone());
    }
    pub fn append_int32(& self, value__1121: i32) {
        let mut t___8740: SqlInt32 = SqlInt32::new(value__1121);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___8740.clone()), None);
    }
    pub fn append_int32_list(& self, values__1124: impl temper_core::ToListed<i32>) {
        let values__1124 = values__1124.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            this__219: SqlBuilder
        }
        impl ClosureGroup___14 {
            fn fn__8737(& self, x__1126: i32) {
                self.this__219.append_int32(x__1126);
            }
        }
        let closure_group = ClosureGroup___14 {
            this__219: self.clone()
        };
        let fn__8737 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1126: i32 | closure_group.fn__8737(x__1126))
        };
        self.append_list(values__1124.clone(), fn__8737.clone());
    }
    pub fn append_int64(& self, value__1128: i64) {
        let mut t___8734: SqlInt64 = SqlInt64::new(value__1128);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___8734.clone()), None);
    }
    pub fn append_int64_list(& self, values__1131: impl temper_core::ToListed<i64>) {
        let values__1131 = values__1131.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___15 {
            this__221: SqlBuilder
        }
        impl ClosureGroup___15 {
            fn fn__8731(& self, x__1133: i64) {
                self.this__221.append_int64(x__1133);
            }
        }
        let closure_group = ClosureGroup___15 {
            this__221: self.clone()
        };
        let fn__8731 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1133: i64 | closure_group.fn__8731(x__1133))
        };
        self.append_list(values__1131.clone(), fn__8731.clone());
    }
    pub fn append_string(& self, value__1135: impl temper_core::ToArcString) {
        let value__1135 = value__1135.to_arc_string();
        let mut t___8728: SqlString = SqlString::new(value__1135.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___8728.clone()), None);
    }
    pub fn append_string_list(& self, values__1138: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__1138 = values__1138.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___16 {
            this__223: SqlBuilder
        }
        impl ClosureGroup___16 {
            fn fn__8725(& self, x__1140: impl temper_core::ToArcString) {
                let x__1140 = x__1140.to_arc_string();
                self.this__223.append_string(x__1140.clone());
            }
        }
        let closure_group = ClosureGroup___16 {
            this__223: self.clone()
        };
        let fn__8725 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1140: std::sync::Arc<String> | closure_group.fn__8725(x__1140))
        };
        self.append_list(values__1138.clone(), fn__8725.clone());
    }
    fn append_list<T>(& self, values__1142: impl temper_core::ToListed<T>, appendValue__1143: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__1142 = values__1142.to_listed();
        let mut t___8720: i32;
        let mut t___8722: T;
        let mut i__1145: i32 = 0;
        'loop___8919: loop {
            t___8720 = temper_core::ListedTrait::len( & ( * values__1142));
            if ! (Some(i__1145) < Some(t___8720)) {
                break;
            }
            if Some(i__1145) > Some(0) {
                self.append_safe(", ");
            }
            t___8722 = temper_core::ListedTrait::get( & ( * values__1142), i__1145);
            appendValue__1143(t___8722.clone());
            i__1145 = i__1145.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___8717: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___8717.clone();
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
        let mut t___8791: i32;
        let builder__1157: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__1158: i32 = 0;
        'loop___8920: loop {
            t___8791 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__1158) < Some(t___8791)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__1158).format_to(builder__1157.clone());
            i__1158 = i__1158.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__1157);
    }
    pub fn new(parts__1160: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__1160 = parts__1160.to_list();
        let parts;
        parts = parts__1160.clone();
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
    fn format_to(& self, builder__1162: std::sync::Arc<std::sync::RwLock<String>>);
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
    pub fn format_to(& self, builder__1166: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1166, self.0.source.clone());
    }
    pub fn new(source__1169: impl temper_core::ToArcString) -> SqlSource {
        let source__1169 = source__1169.to_arc_string();
        let source;
        source = source__1169.clone();
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
    fn format_to(& self, builder__1166: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1166)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__1172: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___5061: std::sync::Arc<String>;
        if self.0.value {
            t___5061 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___5061 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__1172, t___5061.clone());
    }
    pub fn new(value__1175: bool) -> SqlBoolean {
        let value;
        value = value__1175;
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
    fn format_to(& self, builder__1172: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1172)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__1178: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1178, "'");
        let mut t___8772: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___17 {
            builder__1178: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___17 {
            fn fn__8770(& self, c__1180: i32) {
                if Some(c__1180) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1178, "''");
                } else {
                    'ok___8830: {
                        'orelse___1591: {
                            match temper_core::string::builder::append_code_point( & self.builder__1178, c__1180) {
                                Ok(x) => x,
                                _ => break 'orelse___1591
                            };
                            break 'ok___8830;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___17 {
            builder__1178: builder__1178.clone()
        };
        let fn__8770 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1180: i32 | closure_group.fn__8770(c__1180))
        };
        temper_core::string::for_each( & t___8772, & ( * fn__8770.clone()));
        temper_core::string::builder::append( & builder__1178, "'");
    }
    pub fn new(value__1182: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__1182.clone();
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
    fn format_to(& self, builder__1178: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1178)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__1185: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___5050: bool;
        let mut t___5051: bool;
        let s__1187: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__1187.as_str()) == Some("NaN") {
            t___5051 = true;
        } else {
            if Some(s__1187.as_str()) == Some("Infinity") {
                t___5050 = true;
            } else {
                t___5050 = Some(s__1187.as_str()) == Some("-Infinity");
            }
            t___5051 = t___5050;
        }
        if t___5051 {
            temper_core::string::builder::append( & builder__1185, "NULL");
        } else {
            temper_core::string::builder::append( & builder__1185, s__1187.clone());
        }
    }
    pub fn new(value__1189: f64) -> SqlFloat64 {
        let value;
        value = value__1189;
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
    fn format_to(& self, builder__1185: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1185)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__1192: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___8781: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1192, t___8781.clone());
    }
    pub fn new(value__1195: i32) -> SqlInt32 {
        let value;
        value = value__1195;
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
    fn format_to(& self, builder__1192: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1192)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__1198: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___8779: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1198, t___8779.clone());
    }
    pub fn new(value__1201: i64) -> SqlInt64 {
        let value;
        value = value__1201;
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
    fn format_to(& self, builder__1198: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1198)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__1204: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1204, "'");
        #[derive(Clone)]
        struct ClosureGroup___18 {
            builder__1204: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___18 {
            fn fn__8784(& self, c__1206: i32) {
                if Some(c__1206) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1204, "''");
                } else {
                    'ok___8835: {
                        'orelse___1590: {
                            match temper_core::string::builder::append_code_point( & self.builder__1204, c__1206) {
                                Ok(x) => x,
                                _ => break 'orelse___1590
                            };
                            break 'ok___8835;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___18 {
            builder__1204: builder__1204.clone()
        };
        let fn__8784 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1206: i32 | closure_group.fn__8784(c__1206))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__8784.clone()));
        temper_core::string::builder::append( & builder__1204, "'");
    }
    pub fn new(value__1208: impl temper_core::ToArcString) -> SqlString {
        let value__1208 = value__1208.to_arc_string();
        let value;
        value = value__1208.clone();
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
    fn format_to(& self, builder__1204: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1204)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__583: TableDef, params__584: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___8567: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__583.clone(), params__584.clone(), t___8567.clone(), [], true));
}
fn isIdentStart__444(c__1037: i32) -> bool {
    let return__369: bool;
    let mut t___4824: bool;
    let mut t___4825: bool;
    if Some(c__1037) >= Some(97) {
        t___4824 = Some(c__1037) <= Some(122);
    } else {
        t___4824 = false;
    }
    if t___4824 {
        return__369 = true;
    } else {
        if Some(c__1037) >= Some(65) {
            t___4825 = Some(c__1037) <= Some(90);
        } else {
            t___4825 = false;
        }
        if t___4825 {
            return__369 = true;
        } else {
            return__369 = Some(c__1037) == Some(95);
        }
    }
    return return__369;
}
fn isIdentPart__445(c__1039: i32) -> bool {
    let return__370: bool;
    if isIdentStart__444(c__1039) {
        return__370 = true;
    } else {
        if Some(c__1039) >= Some(48) {
            return__370 = Some(c__1039) <= Some(57);
        } else {
            return__370 = false;
        }
    }
    return return__370;
}
pub fn safe_identifier(name__1041: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__1041 = name__1041.to_arc_string();
    let mut t___8565: usize;
    if name__1041.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__1043: usize = 0usize;
    if ! isIdentStart__444(temper_core::string::get( & name__1041, idx__1043)) {
        return Err(temper_core::Error::new());
    }
    let mut t___8562: usize = temper_core::string::next( & name__1041, idx__1043);
    idx__1043 = t___8562;
    'loop___8921: loop {
        if ! temper_core::string::has_index( & name__1041, idx__1043) {
            break;
        }
        if ! isIdentPart__445(temper_core::string::get( & name__1041, idx__1043)) {
            return Err(temper_core::Error::new());
        }
        t___8565 = temper_core::string::next( & name__1041, idx__1043);
        idx__1043 = t___8565;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__1041.clone())));
}
fn csid__441(name__586: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__586 = name__586.to_arc_string();
    let return__284: SafeIdentifier;
    let mut t___4812: SafeIdentifier;
    'ok___8840: {
        'orelse___1595: {
            t___4812 = match safe_identifier(name__586.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1595
            };
            return__284 = t___4812.clone();
            break 'ok___8840;
        }
        return__284 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__284.clone();
}
fn userTable__442() -> TableDef {
    return TableDef::new(csid__441("users"), [FieldDef::new(csid__441("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__441("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__441("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__441("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__441("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__673: TableDef, id__674: i32) -> SqlFragment {
    let b__676: SqlBuilder = SqlBuilder::new();
    b__676.append_safe("DELETE FROM ");
    b__676.append_safe(tableDef__673.table_name().sql_value());
    b__676.append_safe(" WHERE id = ");
    b__676.append_int32(id__674);
    return b__676.accumulated();
}
pub fn from(tableName__863: SafeIdentifier) -> Query {
    return Query::new(tableName__863.clone(), [], [], [], None, None, [], [], [], false, []);
}
pub fn col(table__865: SafeIdentifier, column__866: SafeIdentifier) -> SqlFragment {
    let b__868: SqlBuilder = SqlBuilder::new();
    b__868.append_safe(table__865.sql_value());
    b__868.append_safe(".");
    b__868.append_safe(column__866.sql_value());
    return b__868.accumulated();
}
pub fn count_all() -> SqlFragment {
    let b__870: SqlBuilder = SqlBuilder::new();
    b__870.append_safe("COUNT(*)");
    return b__870.accumulated();
}
pub fn count_col(field__871: SafeIdentifier) -> SqlFragment {
    let b__873: SqlBuilder = SqlBuilder::new();
    b__873.append_safe("COUNT(");
    b__873.append_safe(field__871.sql_value());
    b__873.append_safe(")");
    return b__873.accumulated();
}
pub fn sum_col(field__874: SafeIdentifier) -> SqlFragment {
    let b__876: SqlBuilder = SqlBuilder::new();
    b__876.append_safe("SUM(");
    b__876.append_safe(field__874.sql_value());
    b__876.append_safe(")");
    return b__876.accumulated();
}
pub fn avg_col(field__877: SafeIdentifier) -> SqlFragment {
    let b__879: SqlBuilder = SqlBuilder::new();
    b__879.append_safe("AVG(");
    b__879.append_safe(field__877.sql_value());
    b__879.append_safe(")");
    return b__879.accumulated();
}
pub fn min_col(field__880: SafeIdentifier) -> SqlFragment {
    let b__882: SqlBuilder = SqlBuilder::new();
    b__882.append_safe("MIN(");
    b__882.append_safe(field__880.sql_value());
    b__882.append_safe(")");
    return b__882.accumulated();
}
pub fn max_col(field__883: SafeIdentifier) -> SqlFragment {
    let b__885: SqlBuilder = SqlBuilder::new();
    b__885.append_safe("MAX(");
    b__885.append_safe(field__883.sql_value());
    b__885.append_safe(")");
    return b__885.accumulated();
}
fn sid__443(name__886: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__886 = name__886.to_arc_string();
    let return__362: SafeIdentifier;
    let mut t___4188: SafeIdentifier;
    'ok___8851: {
        'orelse___1603: {
            t___4188 = match safe_identifier(name__886.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1603
            };
            return__362 = t___4188.clone();
            break 'ok___8851;
        }
        return__362 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__362.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__1320() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let params__590: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___8523: TableDef = userTable__442();
        let mut t___8524: SafeIdentifier = csid__441("name");
        let mut t___8525: SafeIdentifier = csid__441("email");
        let cs__591: Changeset = changeset(t___8523.clone(), params__590.clone()).cast(std::sync::Arc::new(vec![t___8524.clone(), t___8525.clone()]));
        let mut t___8528: bool = temper_core::MappedTrait::has( & cs__591.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__8518(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__8518 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8518())
        };
        test___24.assert(t___8528, fn__8518.clone());
        let mut t___8532: bool = temper_core::MappedTrait::has( & cs__591.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__8517(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__8517 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8517())
        };
        test___24.assert(t___8532, fn__8517.clone());
        let mut t___8538: bool = ! temper_core::MappedTrait::has( & cs__591.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__8516(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__8516 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8516())
        };
        test___24.assert(t___8538, fn__8516.clone());
        let mut t___8540: bool = cs__591.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__8515(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__8515 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8515())
        };
        test___24.assert(t___8540, fn__8515.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__1321() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let params__593: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___8501: TableDef = userTable__442();
        let mut t___8502: SafeIdentifier = csid__441("name");
        let cs__594: Changeset = changeset(t___8501.clone(), params__593.clone()).cast(std::sync::Arc::new(vec![t___8502.clone()])).cast(std::sync::Arc::new(vec![csid__441("email")]));
        let mut t___8509: bool = ! temper_core::MappedTrait::has( & cs__594.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__8497(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__8497 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8497())
        };
        test___25.assert(t___8509, fn__8497.clone());
        let mut t___8512: bool = temper_core::MappedTrait::has( & cs__594.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__8496(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__8496 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8496())
        };
        test___25.assert(t___8512, fn__8496.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__1322() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let params__596: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___8483: TableDef = userTable__442();
        let mut t___8484: SafeIdentifier = csid__441("name");
        let mut t___8485: SafeIdentifier = csid__441("email");
        let cs__597: Changeset = changeset(t___8483.clone(), params__596.clone()).cast(std::sync::Arc::new(vec![t___8484.clone(), t___8485.clone()]));
        let mut t___8490: bool = ! temper_core::MappedTrait::has( & cs__597.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__8479(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__8479 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8479())
        };
        test___26.assert(t___8490, fn__8479.clone());
        let mut t___8493: bool = temper_core::MappedTrait::has( & cs__597.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__8478(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__8478 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8478())
        };
        test___26.assert(t___8493, fn__8478.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__1323() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let params__599: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___8465: TableDef = userTable__442();
        let mut t___8466: SafeIdentifier = csid__441("name");
        let cs__600: Changeset = changeset(t___8465.clone(), params__599.clone()).cast(std::sync::Arc::new(vec![t___8466.clone()])).validate_required(std::sync::Arc::new(vec![csid__441("name")]));
        let mut t___8470: bool = cs__600.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__8462(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__8462 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8462())
        };
        test___27.assert(t___8470, fn__8462.clone());
        let mut t___8476: bool = Some(temper_core::ListedTrait::len( & cs__600.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__8461(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__8461 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8461())
        };
        test___27.assert(t___8476, fn__8461.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__1324() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let params__602: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___8441: TableDef = userTable__442();
        let mut t___8442: SafeIdentifier = csid__441("name");
        let cs__603: Changeset = changeset(t___8441.clone(), params__602.clone()).cast(std::sync::Arc::new(vec![t___8442.clone()])).validate_required(std::sync::Arc::new(vec![csid__441("name")]));
        let mut t___8448: bool = ! cs__603.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__8439(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__8439 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8439())
        };
        test___28.assert(t___8448, fn__8439.clone());
        let mut t___8453: bool = Some(temper_core::ListedTrait::len( & cs__603.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__8438(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__8438 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8438())
        };
        test___28.assert(t___8453, fn__8438.clone());
        let mut t___8459: bool = Some(temper_core::ListedTrait::get( & cs__603.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__8437(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__8437 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8437())
        };
        test___28.assert(t___8459, fn__8437.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__1325() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let params__605: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___8429: TableDef = userTable__442();
        let mut t___8430: SafeIdentifier = csid__441("name");
        let cs__606: Changeset = changeset(t___8429.clone(), params__605.clone()).cast(std::sync::Arc::new(vec![t___8430.clone()])).validate_length(csid__441("name"), 2, 50);
        let mut t___8434: bool = cs__606.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__8426(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__8426 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8426())
        };
        test___29.assert(t___8434, fn__8426.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__1326() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let params__608: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___8417: TableDef = userTable__442();
        let mut t___8418: SafeIdentifier = csid__441("name");
        let cs__609: Changeset = changeset(t___8417.clone(), params__608.clone()).cast(std::sync::Arc::new(vec![t___8418.clone()])).validate_length(csid__441("name"), 2, 50);
        let mut t___8424: bool = ! cs__609.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__8414(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__8414 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8414())
        };
        test___30.assert(t___8424, fn__8414.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__1327() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__611: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___8405: TableDef = userTable__442();
        let mut t___8406: SafeIdentifier = csid__441("name");
        let cs__612: Changeset = changeset(t___8405.clone(), params__611.clone()).cast(std::sync::Arc::new(vec![t___8406.clone()])).validate_length(csid__441("name"), 2, 10);
        let mut t___8412: bool = ! cs__612.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__8402(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__8402 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8402())
        };
        test___31.assert(t___8412, fn__8402.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__1328() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__614: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___8394: TableDef = userTable__442();
        let mut t___8395: SafeIdentifier = csid__441("age");
        let cs__615: Changeset = changeset(t___8394.clone(), params__614.clone()).cast(std::sync::Arc::new(vec![t___8395.clone()])).validate_int(csid__441("age"));
        let mut t___8399: bool = cs__615.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__8391(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__8391 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8391())
        };
        test___32.assert(t___8399, fn__8391.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__1329() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__617: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___8382: TableDef = userTable__442();
        let mut t___8383: SafeIdentifier = csid__441("age");
        let cs__618: Changeset = changeset(t___8382.clone(), params__617.clone()).cast(std::sync::Arc::new(vec![t___8383.clone()])).validate_int(csid__441("age"));
        let mut t___8389: bool = ! cs__618.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__8379(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__8379 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8379())
        };
        test___33.assert(t___8389, fn__8379.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__1330() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__620: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___8371: TableDef = userTable__442();
        let mut t___8372: SafeIdentifier = csid__441("score");
        let cs__621: Changeset = changeset(t___8371.clone(), params__620.clone()).cast(std::sync::Arc::new(vec![t___8372.clone()])).validate_float(csid__441("score"));
        let mut t___8376: bool = cs__621.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__8368(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__8368 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8368())
        };
        test___34.assert(t___8376, fn__8368.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__1331() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let params__623: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___8360: TableDef = userTable__442();
        let mut t___8361: SafeIdentifier = csid__441("age");
        let cs__624: Changeset = changeset(t___8360.clone(), params__623.clone()).cast(std::sync::Arc::new(vec![t___8361.clone()])).validate_int64(csid__441("age"));
        let mut t___8365: bool = cs__624.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___38 {}
        impl ClosureGroup___38 {
            fn fn__8357(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___38 {};
        let fn__8357 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8357())
        };
        test___35.assert(t___8365, fn__8357.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__1332() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let params__626: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___8348: TableDef = userTable__442();
        let mut t___8349: SafeIdentifier = csid__441("age");
        let cs__627: Changeset = changeset(t___8348.clone(), params__626.clone()).cast(std::sync::Arc::new(vec![t___8349.clone()])).validate_int64(csid__441("age"));
        let mut t___8355: bool = ! cs__627.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__8345(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__8345 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8345())
        };
        test___36.assert(t___8355, fn__8345.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__1333() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___40 {
            test___37: temper_std::testing::Test
        }
        impl ClosureGroup___40 {
            fn fn__8342(& self, v__629: impl temper_core::ToArcString) {
                let v__629 = v__629.to_arc_string();
                let params__630: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__629.clone())]);
                let mut t___8334: TableDef = userTable__442();
                let mut t___8335: SafeIdentifier = csid__441("active");
                let cs__631: Changeset = changeset(t___8334.clone(), params__630.clone()).cast(std::sync::Arc::new(vec![t___8335.clone()])).validate_bool(csid__441("active"));
                let mut t___8339: bool = cs__631.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___41 {
                    v__629: std::sync::Arc<String>
                }
                impl ClosureGroup___41 {
                    fn fn__8331(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__629.clone()));
                    }
                }
                let closure_group = ClosureGroup___41 {
                    v__629: v__629.clone()
                };
                let fn__8331 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__8331())
                };
                self.test___37.assert(t___8339, fn__8331.clone());
            }
        }
        let closure_group = ClosureGroup___40 {
            test___37: test___37.clone()
        };
        let fn__8342 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__629: std::sync::Arc<String> | closure_group.fn__8342(v__629))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__8342.clone()));
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__1334() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            test___38: temper_std::testing::Test
        }
        impl ClosureGroup___42 {
            fn fn__8328(& self, v__633: impl temper_core::ToArcString) {
                let v__633 = v__633.to_arc_string();
                let params__634: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__633.clone())]);
                let mut t___8320: TableDef = userTable__442();
                let mut t___8321: SafeIdentifier = csid__441("active");
                let cs__635: Changeset = changeset(t___8320.clone(), params__634.clone()).cast(std::sync::Arc::new(vec![t___8321.clone()])).validate_bool(csid__441("active"));
                let mut t___8325: bool = cs__635.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___43 {
                    v__633: std::sync::Arc<String>
                }
                impl ClosureGroup___43 {
                    fn fn__8317(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__633.clone()));
                    }
                }
                let closure_group = ClosureGroup___43 {
                    v__633: v__633.clone()
                };
                let fn__8317 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__8317())
                };
                self.test___38.assert(t___8325, fn__8317.clone());
            }
        }
        let closure_group = ClosureGroup___42 {
            test___38: test___38.clone()
        };
        let fn__8328 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__633: std::sync::Arc<String> | closure_group.fn__8328(v__633))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__8328.clone()));
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__1335() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            test___39: temper_std::testing::Test
        }
        impl ClosureGroup___44 {
            fn fn__8314(& self, v__637: impl temper_core::ToArcString) {
                let v__637 = v__637.to_arc_string();
                let params__638: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__637.clone())]);
                let mut t___8305: TableDef = userTable__442();
                let mut t___8306: SafeIdentifier = csid__441("active");
                let cs__639: Changeset = changeset(t___8305.clone(), params__638.clone()).cast(std::sync::Arc::new(vec![t___8306.clone()])).validate_bool(csid__441("active"));
                let mut t___8312: bool = ! cs__639.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___45 {
                    v__637: std::sync::Arc<String>
                }
                impl ClosureGroup___45 {
                    fn fn__8302(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__637.clone()));
                    }
                }
                let closure_group = ClosureGroup___45 {
                    v__637: v__637.clone()
                };
                let fn__8302 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__8302())
                };
                self.test___39.assert(t___8312, fn__8302.clone());
            }
        }
        let closure_group = ClosureGroup___44 {
            test___39: test___39.clone()
        };
        let fn__8314 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__637: std::sync::Arc<String> | closure_group.fn__8314(v__637))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__8314.clone()));
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__1336() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let mut t___4613: SqlFragment;
        let params__641: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___8290: TableDef = userTable__442();
        let mut t___8291: SafeIdentifier = csid__441("name");
        let mut t___8292: SafeIdentifier = csid__441("email");
        let cs__642: Changeset = changeset(t___8290.clone(), params__641.clone()).cast(std::sync::Arc::new(vec![t___8291.clone(), t___8292.clone()])).validate_required(std::sync::Arc::new(vec![csid__441("name"), csid__441("email")]));
        let sqlFrag__643: SqlFragment;
        'ok___8842: {
            'orelse___1596: {
                t___4613 = match cs__642.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1596
                };
                sqlFrag__643 = t___4613.clone();
                break 'ok___8842;
            }
            sqlFrag__643 = panic!();
        }
        let s__644: std::sync::Arc<String> = sqlFrag__643.to_string();
        let mut t___8299: bool = temper_core::string::index_of( & s__644, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            s__644: std::sync::Arc<String>
        }
        impl ClosureGroup___46 {
            fn fn__8286(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__644.clone()));
            }
        }
        let closure_group = ClosureGroup___46 {
            s__644: s__644.clone()
        };
        let fn__8286 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8286())
        };
        test___40.assert(t___8299, fn__8286.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__1337() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let mut t___4592: SqlFragment;
        let params__646: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___8270: TableDef = userTable__442();
        let mut t___8271: SafeIdentifier = csid__441("name");
        let mut t___8272: SafeIdentifier = csid__441("email");
        let cs__647: Changeset = changeset(t___8270.clone(), params__646.clone()).cast(std::sync::Arc::new(vec![t___8271.clone(), t___8272.clone()])).validate_required(std::sync::Arc::new(vec![csid__441("name"), csid__441("email")]));
        let sqlFrag__648: SqlFragment;
        'ok___8845: {
            'orelse___1597: {
                t___4592 = match cs__647.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1597
                };
                sqlFrag__648 = t___4592.clone();
                break 'ok___8845;
            }
            sqlFrag__648 = panic!();
        }
        let s__649: std::sync::Arc<String> = sqlFrag__648.to_string();
        let mut t___8279: bool = temper_core::string::index_of( & s__649, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___47 {
            s__649: std::sync::Arc<String>
        }
        impl ClosureGroup___47 {
            fn fn__8266(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__649.clone()));
            }
        }
        let closure_group = ClosureGroup___47 {
            s__649: s__649.clone()
        };
        let fn__8266 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8266())
        };
        test___41.assert(t___8279, fn__8266.clone());
        let mut t___8283: bool = temper_core::string::index_of( & s__649, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___48 {
            s__649: std::sync::Arc<String>
        }
        impl ClosureGroup___48 {
            fn fn__8265(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__649.clone()));
            }
        }
        let closure_group = ClosureGroup___48 {
            s__649: s__649.clone()
        };
        let fn__8265 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8265())
        };
        test___41.assert(t___8283, fn__8265.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__1338() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let mut t___4575: SqlFragment;
        let params__651: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___8252: TableDef = userTable__442();
        let mut t___8253: SafeIdentifier = csid__441("name");
        let mut t___8254: SafeIdentifier = csid__441("email");
        let mut t___8255: SafeIdentifier = csid__441("age");
        let cs__652: Changeset = changeset(t___8252.clone(), params__651.clone()).cast(std::sync::Arc::new(vec![t___8253.clone(), t___8254.clone(), t___8255.clone()])).validate_required(std::sync::Arc::new(vec![csid__441("name"), csid__441("email")]));
        let sqlFrag__653: SqlFragment;
        'ok___8846: {
            'orelse___1598: {
                t___4575 = match cs__652.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1598
                };
                sqlFrag__653 = t___4575.clone();
                break 'ok___8846;
            }
            sqlFrag__653 = panic!();
        }
        let s__654: std::sync::Arc<String> = sqlFrag__653.to_string();
        let mut t___8262: bool = temper_core::string::index_of( & s__654, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___49 {
            s__654: std::sync::Arc<String>
        }
        impl ClosureGroup___49 {
            fn fn__8247(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__654.clone()));
            }
        }
        let closure_group = ClosureGroup___49 {
            s__654: s__654.clone()
        };
        let fn__8247 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8247())
        };
        test___42.assert(t___8262, fn__8247.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__1339() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let params__656: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___8240: TableDef = userTable__442();
        let mut t___8241: SafeIdentifier = csid__441("name");
        let cs__657: Changeset = changeset(t___8240.clone(), params__656.clone()).cast(std::sync::Arc::new(vec![t___8241.clone()])).validate_required(std::sync::Arc::new(vec![csid__441("name")]));
        let didBubble__658: bool;
        'ok___8847: {
            'orelse___1599: {
                match cs__657.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1599
                };
                didBubble__658 = false;
                break 'ok___8847;
            }
            didBubble__658 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___50 {}
        impl ClosureGroup___50 {
            fn fn__8238(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___50 {};
        let fn__8238 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8238())
        };
        test___43.assert(didBubble__658, fn__8238.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__1340() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let strictTable__660: TableDef = TableDef::new(csid__441("posts"), [FieldDef::new(csid__441("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__441("body"), FieldType::new(StringField::new()), true)]);
        let params__661: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___8231: SafeIdentifier = csid__441("body");
        let cs__662: Changeset = changeset(strictTable__660.clone(), params__661.clone()).cast(std::sync::Arc::new(vec![t___8231.clone()]));
        let mut t___8233: bool = cs__662.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___51 {}
        impl ClosureGroup___51 {
            fn fn__8220(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___51 {};
        let fn__8220 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8220())
        };
        test___44.assert(t___8233, fn__8220.clone());
        let didBubble__663: bool;
        'ok___8848: {
            'orelse___1600: {
                match cs__662.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1600
                };
                didBubble__663 = false;
                break 'ok___8848;
            }
            didBubble__663 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__8219(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__8219 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8219())
        };
        test___44.assert(didBubble__663, fn__8219.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__1341() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let mut t___4535: SqlFragment;
        let params__665: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___8210: TableDef = userTable__442();
        let mut t___8211: SafeIdentifier = csid__441("name");
        let cs__666: Changeset = changeset(t___8210.clone(), params__665.clone()).cast(std::sync::Arc::new(vec![t___8211.clone()])).validate_required(std::sync::Arc::new(vec![csid__441("name")]));
        let sqlFrag__667: SqlFragment;
        'ok___8849: {
            'orelse___1601: {
                t___4535 = match cs__666.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___1601
                };
                sqlFrag__667 = t___4535.clone();
                break 'ok___8849;
            }
            sqlFrag__667 = panic!();
        }
        let s__668: std::sync::Arc<String> = sqlFrag__667.to_string();
        let mut t___8217: bool = Some(s__668.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___53 {
            s__668: std::sync::Arc<String>
        }
        impl ClosureGroup___53 {
            fn fn__8207(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__668.clone()));
            }
        }
        let closure_group = ClosureGroup___53 {
            s__668: s__668.clone()
        };
        let fn__8207 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8207())
        };
        test___45.assert(t___8217, fn__8207.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__1342() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let params__670: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___8200: TableDef = userTable__442();
        let mut t___8201: SafeIdentifier = csid__441("name");
        let cs__671: Changeset = changeset(t___8200.clone(), params__670.clone()).cast(std::sync::Arc::new(vec![t___8201.clone()])).validate_required(std::sync::Arc::new(vec![csid__441("name")]));
        let didBubble__672: bool;
        'ok___8850: {
            'orelse___1602: {
                match cs__671.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___1602
                };
                didBubble__672 = false;
                break 'ok___8850;
            }
            didBubble__672 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__8198(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__8198 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8198())
        };
        test___46.assert(didBubble__672, fn__8198.clone());
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__1391() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let q__889: Query = from(sid__443("users"));
        let mut t___7866: bool = Some(q__889.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__7861(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__7861 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7861())
        };
        test___47.assert(t___7866, fn__7861.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__1392() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___7852: SafeIdentifier = sid__443("users");
        let mut t___7853: SafeIdentifier = sid__443("id");
        let mut t___7854: SafeIdentifier = sid__443("name");
        let q__891: Query = from(t___7852.clone()).select([t___7853.clone(), t___7854.clone()]);
        let mut t___7859: bool = Some(q__891.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__7851(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__7851 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7851())
        };
        test___48.assert(t___7859, fn__7851.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__1393() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___7840: SafeIdentifier = sid__443("users");
        let mut t___7841: SqlBuilder = SqlBuilder::new();
        t___7841.append_safe("age > ");
        t___7841.append_int32(18);
        let mut t___7844: SqlFragment = t___7841.accumulated();
        let q__893: Query = from(t___7840.clone()).r#where(t___7844.clone());
        let mut t___7849: bool = Some(q__893.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__7839(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__7839 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7839())
        };
        test___49.assert(t___7849, fn__7839.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__1395() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___7828: SafeIdentifier = sid__443("users");
        let mut t___7829: SqlBuilder = SqlBuilder::new();
        t___7829.append_safe("active = ");
        t___7829.append_boolean(true);
        let mut t___7832: SqlFragment = t___7829.accumulated();
        let q__895: Query = from(t___7828.clone()).r#where(t___7832.clone());
        let mut t___7837: bool = Some(q__895.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__7827(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__7827 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7827())
        };
        test___50.assert(t___7837, fn__7827.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__1397() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let mut t___7811: SafeIdentifier = sid__443("users");
        let mut t___7812: SqlBuilder = SqlBuilder::new();
        t___7812.append_safe("age > ");
        t___7812.append_int32(18);
        let mut t___7815: SqlFragment = t___7812.accumulated();
        let mut t___7816: Query = from(t___7811.clone()).r#where(t___7815.clone());
        let mut t___7817: SqlBuilder = SqlBuilder::new();
        t___7817.append_safe("active = ");
        t___7817.append_boolean(true);
        let q__897: Query = t___7816.r#where(t___7817.accumulated());
        let mut t___7825: bool = Some(q__897.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__7810(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__7810 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7810())
        };
        test___51.assert(t___7825, fn__7810.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__1400() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___7802: SafeIdentifier = sid__443("users");
        let mut t___7803: SafeIdentifier = sid__443("name");
        let q__899: Query = from(t___7802.clone()).order_by(t___7803.clone(), true);
        let mut t___7808: bool = Some(q__899.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__7801(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__7801 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7801())
        };
        test___52.assert(t___7808, fn__7801.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__1401() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let mut t___7793: SafeIdentifier = sid__443("users");
        let mut t___7794: SafeIdentifier = sid__443("created_at");
        let q__901: Query = from(t___7793.clone()).order_by(t___7794.clone(), false);
        let mut t___7799: bool = Some(q__901.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__7792(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__7792 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7792())
        };
        test___53.assert(t___7799, fn__7792.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__1402() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let mut t___4122: Query;
        let mut t___4123: Query;
        let q__903: Query;
        'ok___8852: {
            'orelse___1604: {
                t___4122 = match from(sid__443("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1604
                };
                t___4123 = match t___4122.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1604
                };
                q__903 = t___4123.clone();
                break 'ok___8852;
            }
            q__903 = panic!();
        }
        let mut t___7790: bool = Some(q__903.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__7785(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__7785 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7785())
        };
        test___54.assert(t___7790, fn__7785.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__1403() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let didBubble__905: bool;
        'ok___8853: {
            'orelse___1605: {
                match from(sid__443("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1605
                };
                didBubble__905 = false;
                break 'ok___8853;
            }
            didBubble__905 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___63 {}
        impl ClosureGroup___63 {
            fn fn__7781(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___63 {};
        let fn__7781 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7781())
        };
        test___55.assert(didBubble__905, fn__7781.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__1404() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let didBubble__907: bool;
        'ok___8854: {
            'orelse___1606: {
                match from(sid__443("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1606
                };
                didBubble__907 = false;
                break 'ok___8854;
            }
            didBubble__907 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___64 {}
        impl ClosureGroup___64 {
            fn fn__7777(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___64 {};
        let fn__7777 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7777())
        };
        test___56.assert(didBubble__907, fn__7777.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__1405() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___7755: SafeIdentifier;
        let mut t___7756: SafeIdentifier;
        let mut t___7757: SafeIdentifier;
        let mut t___7758: SafeIdentifier;
        let mut t___7759: Query;
        let mut t___7760: SqlBuilder;
        let mut t___7764: Query;
        let mut t___7765: SqlBuilder;
        let mut t___4108: Query;
        let mut t___4109: Query;
        let minAge__909: i32 = 21;
        let q__910: Query;
        'ok___8855: {
            'orelse___1607: {
                t___7755 = sid__443("users");
                t___7756 = sid__443("id");
                t___7757 = sid__443("name");
                t___7758 = sid__443("email");
                t___7759 = from(t___7755.clone()).select([t___7756.clone(), t___7757.clone(), t___7758.clone()]);
                t___7760 = SqlBuilder::new();
                t___7760.append_safe("age >= ");
                t___7760.append_int32(21);
                t___7764 = t___7759.r#where(t___7760.accumulated());
                t___7765 = SqlBuilder::new();
                t___7765.append_safe("active = ");
                t___7765.append_boolean(true);
                t___4108 = match t___7764.r#where(t___7765.accumulated()).order_by(sid__443("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___1607
                };
                t___4109 = match t___4108.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___1607
                };
                q__910 = t___4109.clone();
                break 'ok___8855;
            }
            q__910 = panic!();
        }
        let mut t___7775: bool = Some(q__910.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__7754(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__7754 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7754())
        };
        test___57.assert(t___7775, fn__7754.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__1408() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let mut t___4085: SqlFragment;
        let mut t___4086: SqlFragment;
        let q__912: Query = from(sid__443("users"));
        'ok___8856: {
            'orelse___1608: {
                t___4085 = match q__912.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1608
                };
                t___4086 = t___4085.clone();
                break 'ok___8856;
            }
            t___4086 = panic!();
        }
        let s__913: std::sync::Arc<String> = t___4086.to_string();
        let mut t___7752: bool = Some(s__913.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___66 {
            s__913: std::sync::Arc<String>
        }
        impl ClosureGroup___66 {
            fn fn__7748(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__913.clone()));
            }
        }
        let closure_group = ClosureGroup___66 {
            s__913: s__913.clone()
        };
        let fn__7748 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7748())
        };
        test___58.assert(t___7752, fn__7748.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__1409() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let mut t___4077: Query;
        let mut t___4080: SqlFragment;
        let mut t___4081: SqlFragment;
        let q__915: Query;
        'ok___8857: {
            'orelse___1609: {
                t___4077 = match from(sid__443("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___1609
                };
                q__915 = t___4077.clone();
                break 'ok___8857;
            }
            q__915 = panic!();
        }
        'ok___8858: {
            'orelse___1610: {
                t___4080 = match q__915.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1610
                };
                t___4081 = t___4080.clone();
                break 'ok___8858;
            }
            t___4081 = panic!();
        }
        let s__916: std::sync::Arc<String> = t___4081.to_string();
        let mut t___7746: bool = Some(s__916.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___67 {
            s__916: std::sync::Arc<String>
        }
        impl ClosureGroup___67 {
            fn fn__7742(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__916.clone()));
            }
        }
        let closure_group = ClosureGroup___67 {
            s__916: s__916.clone()
        };
        let fn__7742 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7742())
        };
        test___59.assert(t___7746, fn__7742.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__1410() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let didBubble__918: bool;
        'ok___8859: {
            'orelse___1611: {
                match from(sid__443("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1611
                };
                didBubble__918 = false;
                break 'ok___8859;
            }
            didBubble__918 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__7738(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__7738 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7738())
        };
        test___60.assert(didBubble__918, fn__7738.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__1411() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let evil__920: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___7722: SafeIdentifier = sid__443("users");
        let mut t___7723: SqlBuilder = SqlBuilder::new();
        t___7723.append_safe("name = ");
        t___7723.append_string("'; DROP TABLE users; --");
        let mut t___7726: SqlFragment = t___7723.accumulated();
        let q__921: Query = from(t___7722.clone()).r#where(t___7726.clone());
        let s__922: std::sync::Arc<String> = q__921.to_sql().to_string();
        let mut t___7731: bool = temper_core::string::index_of( & s__922, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___69 {
            s__922: std::sync::Arc<String>
        }
        impl ClosureGroup___69 {
            fn fn__7721(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__922.clone()));
            }
        }
        let closure_group = ClosureGroup___69 {
            s__922: s__922.clone()
        };
        let fn__7721 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7721())
        };
        test___61.assert(t___7731, fn__7721.clone());
        let mut t___7735: bool = temper_core::string::index_of( & s__922, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___70 {
            s__922: std::sync::Arc<String>
        }
        impl ClosureGroup___70 {
            fn fn__7720(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__922.clone()));
            }
        }
        let closure_group = ClosureGroup___70 {
            s__922: s__922.clone()
        };
        let fn__7720 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7720())
        };
        test___61.assert(t___7735, fn__7720.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__1413() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let attack__924: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__925: bool;
        'ok___8860: {
            'orelse___1612: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___1612
                };
                didBubble__925 = false;
                break 'ok___8860;
            }
            didBubble__925 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__7717(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__7717 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7717())
        };
        test___62.assert(didBubble__925, fn__7717.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__1414() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let mut t___7706: SafeIdentifier = sid__443("users");
        let mut t___7707: SafeIdentifier = sid__443("orders");
        let mut t___7708: SqlBuilder = SqlBuilder::new();
        t___7708.append_safe("users.id = orders.user_id");
        let mut t___7710: SqlFragment = t___7708.accumulated();
        let q__927: Query = from(t___7706.clone()).inner_join(t___7707.clone(), t___7710.clone());
        let mut t___7715: bool = Some(q__927.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__7705(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__7705 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7705())
        };
        test___63.assert(t___7715, fn__7705.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__1416() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let mut t___7694: SafeIdentifier = sid__443("users");
        let mut t___7695: SafeIdentifier = sid__443("profiles");
        let mut t___7696: SqlBuilder = SqlBuilder::new();
        t___7696.append_safe("users.id = profiles.user_id");
        let mut t___7698: SqlFragment = t___7696.accumulated();
        let q__929: Query = from(t___7694.clone()).left_join(t___7695.clone(), t___7698.clone());
        let mut t___7703: bool = Some(q__929.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__7693(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__7693 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7693())
        };
        test___64.assert(t___7703, fn__7693.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__1418() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let mut t___7682: SafeIdentifier = sid__443("orders");
        let mut t___7683: SafeIdentifier = sid__443("users");
        let mut t___7684: SqlBuilder = SqlBuilder::new();
        t___7684.append_safe("orders.user_id = users.id");
        let mut t___7686: SqlFragment = t___7684.accumulated();
        let q__931: Query = from(t___7682.clone()).right_join(t___7683.clone(), t___7686.clone());
        let mut t___7691: bool = Some(q__931.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__7681(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__7681 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7681())
        };
        test___65.assert(t___7691, fn__7681.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__1420() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let mut t___7670: SafeIdentifier = sid__443("users");
        let mut t___7671: SafeIdentifier = sid__443("orders");
        let mut t___7672: SqlBuilder = SqlBuilder::new();
        t___7672.append_safe("users.id = orders.user_id");
        let mut t___7674: SqlFragment = t___7672.accumulated();
        let q__933: Query = from(t___7670.clone()).full_join(t___7671.clone(), t___7674.clone());
        let mut t___7679: bool = Some(q__933.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__7669(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__7669 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7669())
        };
        test___66.assert(t___7679, fn__7669.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__1422() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let mut t___7653: SafeIdentifier = sid__443("users");
        let mut t___7654: SafeIdentifier = sid__443("orders");
        let mut t___7655: SqlBuilder = SqlBuilder::new();
        t___7655.append_safe("users.id = orders.user_id");
        let mut t___7657: SqlFragment = t___7655.accumulated();
        let mut t___7658: Query = from(t___7653.clone()).inner_join(t___7654.clone(), t___7657.clone());
        let mut t___7659: SafeIdentifier = sid__443("profiles");
        let mut t___7660: SqlBuilder = SqlBuilder::new();
        t___7660.append_safe("users.id = profiles.user_id");
        let q__935: Query = t___7658.left_join(t___7659.clone(), t___7660.accumulated());
        let mut t___7667: bool = Some(q__935.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__7652(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__7652 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7652())
        };
        test___67.assert(t___7667, fn__7652.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__1425() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let mut t___7634: SafeIdentifier;
        let mut t___7635: SafeIdentifier;
        let mut t___7636: SqlBuilder;
        let mut t___7638: SqlFragment;
        let mut t___7639: Query;
        let mut t___7640: SqlBuilder;
        let mut t___3992: Query;
        let q__937: Query;
        'ok___8861: {
            'orelse___1613: {
                t___7634 = sid__443("users");
                t___7635 = sid__443("orders");
                t___7636 = SqlBuilder::new();
                t___7636.append_safe("users.id = orders.user_id");
                t___7638 = t___7636.accumulated();
                t___7639 = from(t___7634.clone()).inner_join(t___7635.clone(), t___7638.clone());
                t___7640 = SqlBuilder::new();
                t___7640.append_safe("orders.total > ");
                t___7640.append_int32(100);
                t___3992 = match t___7639.r#where(t___7640.accumulated()).order_by(sid__443("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1613
                };
                q__937 = t___3992.clone();
                break 'ok___8861;
            }
            q__937 = panic!();
        }
        let mut t___7650: bool = Some(q__937.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__7633(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__7633 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7633())
        };
        test___68.assert(t___7650, fn__7633.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__1428() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let c__939: SqlFragment = col(sid__443("users"), sid__443("id"));
        let mut t___7631: bool = Some(c__939.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__7625(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__7625 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7625())
        };
        test___69.assert(t___7631, fn__7625.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__1429() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let onCond__941: SqlFragment = col(sid__443("users"), sid__443("id"));
        let b__942: SqlBuilder = SqlBuilder::new();
        b__942.append_fragment(onCond__941.clone());
        b__942.append_safe(" = ");
        b__942.append_fragment(col(sid__443("orders"), sid__443("user_id")));
        let mut t___7616: SafeIdentifier = sid__443("users");
        let mut t___7617: SafeIdentifier = sid__443("orders");
        let mut t___7618: SqlFragment = b__942.accumulated();
        let q__943: Query = from(t___7616.clone()).inner_join(t___7617.clone(), t___7618.clone());
        let mut t___7623: bool = Some(q__943.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__7605(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__7605 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7605())
        };
        test___70.assert(t___7623, fn__7605.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn orWhereBasic__1430() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let mut t___7594: SafeIdentifier = sid__443("users");
        let mut t___7595: SqlBuilder = SqlBuilder::new();
        t___7595.append_safe("status = ");
        t___7595.append_string("active");
        let mut t___7598: SqlFragment = t___7595.accumulated();
        let q__945: Query = from(t___7594.clone()).or_where(t___7598.clone());
        let mut t___7603: bool = Some(q__945.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE status = 'active'");
        #[derive(Clone)]
        struct ClosureGroup___80 {}
        impl ClosureGroup___80 {
            fn fn__7593(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orWhere basic".to_string());
            }
        }
        let closure_group = ClosureGroup___80 {};
        let fn__7593 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7593())
        };
        test___71.assert(t___7603, fn__7593.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn whereThenOrWhere__1432() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let mut t___7577: SafeIdentifier = sid__443("users");
        let mut t___7578: SqlBuilder = SqlBuilder::new();
        t___7578.append_safe("age > ");
        t___7578.append_int32(18);
        let mut t___7581: SqlFragment = t___7578.accumulated();
        let mut t___7582: Query = from(t___7577.clone()).r#where(t___7581.clone());
        let mut t___7583: SqlBuilder = SqlBuilder::new();
        t___7583.append_safe("vip = ");
        t___7583.append_boolean(true);
        let q__947: Query = t___7582.or_where(t___7583.accumulated());
        let mut t___7591: bool = Some(q__947.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___81 {}
        impl ClosureGroup___81 {
            fn fn__7576(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where then orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___81 {};
        let fn__7576 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7576())
        };
        test___72.assert(t___7591, fn__7576.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn multipleOrWhere__1435() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let mut t___7555: SafeIdentifier = sid__443("users");
        let mut t___7556: SqlBuilder = SqlBuilder::new();
        t___7556.append_safe("active = ");
        t___7556.append_boolean(true);
        let mut t___7559: SqlFragment = t___7556.accumulated();
        let mut t___7560: Query = from(t___7555.clone()).r#where(t___7559.clone());
        let mut t___7561: SqlBuilder = SqlBuilder::new();
        t___7561.append_safe("role = ");
        t___7561.append_string("admin");
        let mut t___7565: Query = t___7560.or_where(t___7561.accumulated());
        let mut t___7566: SqlBuilder = SqlBuilder::new();
        t___7566.append_safe("role = ");
        t___7566.append_string("moderator");
        let q__949: Query = t___7565.or_where(t___7566.accumulated());
        let mut t___7574: bool = Some(q__949.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'");
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__7554(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("multiple orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__7554 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7554())
        };
        test___73.assert(t___7574, fn__7554.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn mixedWhereAndOrWhere__1439() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let mut t___7533: SafeIdentifier = sid__443("users");
        let mut t___7534: SqlBuilder = SqlBuilder::new();
        t___7534.append_safe("age > ");
        t___7534.append_int32(18);
        let mut t___7537: SqlFragment = t___7534.accumulated();
        let mut t___7538: Query = from(t___7533.clone()).r#where(t___7537.clone());
        let mut t___7539: SqlBuilder = SqlBuilder::new();
        t___7539.append_safe("active = ");
        t___7539.append_boolean(true);
        let mut t___7543: Query = t___7538.r#where(t___7539.accumulated());
        let mut t___7544: SqlBuilder = SqlBuilder::new();
        t___7544.append_safe("vip = ");
        t___7544.append_boolean(true);
        let q__951: Query = t___7543.or_where(t___7544.accumulated());
        let mut t___7552: bool = Some(q__951.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__7532(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed where and orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__7532 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7532())
        };
        test___74.assert(t___7552, fn__7532.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn whereNull__1443() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let mut t___7524: SafeIdentifier = sid__443("users");
        let mut t___7525: SafeIdentifier = sid__443("deleted_at");
        let q__953: Query = from(t___7524.clone()).where_null(t___7525.clone());
        let mut t___7530: bool = Some(q__953.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___84 {}
        impl ClosureGroup___84 {
            fn fn__7523(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull".to_string());
            }
        }
        let closure_group = ClosureGroup___84 {};
        let fn__7523 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7523())
        };
        test___75.assert(t___7530, fn__7523.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNull__1444() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        let mut t___7515: SafeIdentifier = sid__443("users");
        let mut t___7516: SafeIdentifier = sid__443("email");
        let q__955: Query = from(t___7515.clone()).where_not_null(t___7516.clone());
        let mut t___7521: bool = Some(q__955.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email IS NOT NULL");
        #[derive(Clone)]
        struct ClosureGroup___85 {}
        impl ClosureGroup___85 {
            fn fn__7514(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull".to_string());
            }
        }
        let closure_group = ClosureGroup___85 {};
        let fn__7514 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7514())
        };
        test___76.assert(t___7521, fn__7514.clone());
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn whereNullChainedWithWhere__1445() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let mut t___7501: SafeIdentifier = sid__443("users");
        let mut t___7502: SqlBuilder = SqlBuilder::new();
        t___7502.append_safe("active = ");
        t___7502.append_boolean(true);
        let mut t___7505: SqlFragment = t___7502.accumulated();
        let q__957: Query = from(t___7501.clone()).r#where(t___7505.clone()).where_null(sid__443("deleted_at"));
        let mut t___7512: bool = Some(q__957.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn fn__7500(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull chained".to_string());
            }
        }
        let closure_group = ClosureGroup___86 {};
        let fn__7500 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7500())
        };
        test___77.assert(t___7512, fn__7500.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNullChainedWithOrWhere__1447() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let mut t___7487: SafeIdentifier = sid__443("users");
        let mut t___7488: SafeIdentifier = sid__443("deleted_at");
        let mut t___7489: Query = from(t___7487.clone()).where_null(t___7488.clone());
        let mut t___7490: SqlBuilder = SqlBuilder::new();
        t___7490.append_safe("role = ");
        t___7490.append_string("admin");
        let q__959: Query = t___7489.or_where(t___7490.accumulated());
        let mut t___7498: bool = Some(q__959.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'");
        #[derive(Clone)]
        struct ClosureGroup___87 {}
        impl ClosureGroup___87 {
            fn fn__7486(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull with orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___87 {};
        let fn__7486 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7486())
        };
        test___78.assert(t___7498, fn__7486.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithIntValues__1449() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let mut t___7475: SafeIdentifier = sid__443("users");
        let mut t___7476: SafeIdentifier = sid__443("id");
        let mut t___7477: SqlInt32 = SqlInt32::new(1);
        let mut t___7478: SqlInt32 = SqlInt32::new(2);
        let mut t___7479: SqlInt32 = SqlInt32::new(3);
        let q__961: Query = from(t___7475.clone()).where_in(t___7476.clone(), [SqlPart::new(t___7477.clone()), SqlPart::new(t___7478.clone()), SqlPart::new(t___7479.clone())]);
        let mut t___7484: bool = Some(q__961.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___88 {}
        impl ClosureGroup___88 {
            fn fn__7474(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn ints".to_string());
            }
        }
        let closure_group = ClosureGroup___88 {};
        let fn__7474 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7474())
        };
        test___79.assert(t___7484, fn__7474.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithStringValuesEscaping__1450() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let mut t___7464: SafeIdentifier = sid__443("users");
        let mut t___7465: SafeIdentifier = sid__443("name");
        let mut t___7466: SqlString = SqlString::new("Alice");
        let mut t___7467: SqlString = SqlString::new("Bob's");
        let q__963: Query = from(t___7464.clone()).where_in(t___7465.clone(), [SqlPart::new(t___7466.clone()), SqlPart::new(t___7467.clone())]);
        let mut t___7472: bool = Some(q__963.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')");
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__7463(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn strings".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__7463 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7463())
        };
        test___80.assert(t___7472, fn__7463.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithEmptyListProduces1_0__1451() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let mut t___7455: SafeIdentifier = sid__443("users");
        let mut t___7456: SafeIdentifier = sid__443("id");
        let q__965: Query = from(t___7455.clone()).where_in(t___7456.clone(), []);
        let mut t___7461: bool = Some(q__965.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___90 {}
        impl ClosureGroup___90 {
            fn fn__7454(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn empty".to_string());
            }
        }
        let closure_group = ClosureGroup___90 {};
        let fn__7454 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7454())
        };
        test___81.assert(t___7461, fn__7454.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn whereInChained__1452() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let mut t___7439: SafeIdentifier = sid__443("users");
        let mut t___7440: SqlBuilder = SqlBuilder::new();
        t___7440.append_safe("active = ");
        t___7440.append_boolean(true);
        let mut t___7443: SqlFragment = t___7440.accumulated();
        let q__967: Query = from(t___7439.clone()).r#where(t___7443.clone()).where_in(sid__443("role"), [SqlPart::new(SqlString::new("admin")), SqlPart::new(SqlString::new("user"))]);
        let mut t___7452: bool = Some(q__967.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')");
        #[derive(Clone)]
        struct ClosureGroup___91 {}
        impl ClosureGroup___91 {
            fn fn__7438(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn chained".to_string());
            }
        }
        let closure_group = ClosureGroup___91 {};
        let fn__7438 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7438())
        };
        test___82.assert(t___7452, fn__7438.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn whereInSingleElement__1454() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let mut t___7429: SafeIdentifier = sid__443("users");
        let mut t___7430: SafeIdentifier = sid__443("id");
        let mut t___7431: SqlInt32 = SqlInt32::new(42);
        let q__969: Query = from(t___7429.clone()).where_in(t___7430.clone(), [SqlPart::new(t___7431.clone())]);
        let mut t___7436: bool = Some(q__969.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___92 {}
        impl ClosureGroup___92 {
            fn fn__7428(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn single".to_string());
            }
        }
        let closure_group = ClosureGroup___92 {};
        let fn__7428 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7428())
        };
        test___83.assert(t___7436, fn__7428.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn whereNotBasic__1455() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let mut t___7417: SafeIdentifier = sid__443("users");
        let mut t___7418: SqlBuilder = SqlBuilder::new();
        t___7418.append_safe("active = ");
        t___7418.append_boolean(true);
        let mut t___7421: SqlFragment = t___7418.accumulated();
        let q__971: Query = from(t___7417.clone()).where_not(t___7421.clone());
        let mut t___7426: bool = Some(q__971.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE NOT (active = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___93 {}
        impl ClosureGroup___93 {
            fn fn__7416(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot".to_string());
            }
        }
        let closure_group = ClosureGroup___93 {};
        let fn__7416 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7416())
        };
        test___84.assert(t___7426, fn__7416.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn whereNotChained__1457() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let mut t___7400: SafeIdentifier = sid__443("users");
        let mut t___7401: SqlBuilder = SqlBuilder::new();
        t___7401.append_safe("age > ");
        t___7401.append_int32(18);
        let mut t___7404: SqlFragment = t___7401.accumulated();
        let mut t___7405: Query = from(t___7400.clone()).r#where(t___7404.clone());
        let mut t___7406: SqlBuilder = SqlBuilder::new();
        t___7406.append_safe("banned = ");
        t___7406.append_boolean(true);
        let q__973: Query = t___7405.where_not(t___7406.accumulated());
        let mut t___7414: bool = Some(q__973.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___94 {}
        impl ClosureGroup___94 {
            fn fn__7399(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot chained".to_string());
            }
        }
        let closure_group = ClosureGroup___94 {};
        let fn__7399 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7399())
        };
        test___85.assert(t___7414, fn__7399.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenIntegers__1460() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let mut t___7389: SafeIdentifier = sid__443("users");
        let mut t___7390: SafeIdentifier = sid__443("age");
        let mut t___7391: SqlInt32 = SqlInt32::new(18);
        let mut t___7392: SqlInt32 = SqlInt32::new(65);
        let q__975: Query = from(t___7389.clone()).where_between(t___7390.clone(), SqlPart::new(t___7391.clone()), SqlPart::new(t___7392.clone()));
        let mut t___7397: bool = Some(q__975.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age BETWEEN 18 AND 65");
        #[derive(Clone)]
        struct ClosureGroup___95 {}
        impl ClosureGroup___95 {
            fn fn__7388(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween ints".to_string());
            }
        }
        let closure_group = ClosureGroup___95 {};
        let fn__7388 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7388())
        };
        test___86.assert(t___7397, fn__7388.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenChained__1461() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let mut t___7373: SafeIdentifier = sid__443("users");
        let mut t___7374: SqlBuilder = SqlBuilder::new();
        t___7374.append_safe("active = ");
        t___7374.append_boolean(true);
        let mut t___7377: SqlFragment = t___7374.accumulated();
        let q__977: Query = from(t___7373.clone()).r#where(t___7377.clone()).where_between(sid__443("age"), SqlPart::new(SqlInt32::new(21)), SqlPart::new(SqlInt32::new(30)));
        let mut t___7386: bool = Some(q__977.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30");
        #[derive(Clone)]
        struct ClosureGroup___96 {}
        impl ClosureGroup___96 {
            fn fn__7372(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween chained".to_string());
            }
        }
        let closure_group = ClosureGroup___96 {};
        let fn__7372 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7372())
        };
        test___87.assert(t___7386, fn__7372.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeBasic__1463() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let mut t___7364: SafeIdentifier = sid__443("users");
        let mut t___7365: SafeIdentifier = sid__443("name");
        let q__979: Query = from(t___7364.clone()).where_like(t___7365.clone(), "John%");
        let mut t___7370: bool = Some(q__979.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE 'John%'");
        #[derive(Clone)]
        struct ClosureGroup___97 {}
        impl ClosureGroup___97 {
            fn fn__7363(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike".to_string());
            }
        }
        let closure_group = ClosureGroup___97 {};
        let fn__7363 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7363())
        };
        test___88.assert(t___7370, fn__7363.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn whereIlikeBasic__1464() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let mut t___7355: SafeIdentifier = sid__443("users");
        let mut t___7356: SafeIdentifier = sid__443("email");
        let q__981: Query = from(t___7355.clone()).where_i_like(t___7356.clone(), "%@gmail.com");
        let mut t___7361: bool = Some(q__981.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email ILIKE '%@gmail.com'");
        #[derive(Clone)]
        struct ClosureGroup___98 {}
        impl ClosureGroup___98 {
            fn fn__7354(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereILike".to_string());
            }
        }
        let closure_group = ClosureGroup___98 {};
        let fn__7354 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7354())
        };
        test___89.assert(t___7361, fn__7354.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWithInjectionAttempt__1465() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___90 = temper_std::testing::Test::new();
        let mut t___7341: SafeIdentifier = sid__443("users");
        let mut t___7342: SafeIdentifier = sid__443("name");
        let q__983: Query = from(t___7341.clone()).where_like(t___7342.clone(), "'; DROP TABLE users; --");
        let s__984: std::sync::Arc<String> = q__983.to_sql().to_string();
        let mut t___7347: bool = temper_core::string::index_of( & s__984, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___99 {
            s__984: std::sync::Arc<String>
        }
        impl ClosureGroup___99 {
            fn fn__7340(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like injection escaped: {}", self.s__984.clone()));
            }
        }
        let closure_group = ClosureGroup___99 {
            s__984: s__984.clone()
        };
        let fn__7340 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7340())
        };
        test___90.assert(t___7347, fn__7340.clone());
        let mut t___7351: bool = temper_core::string::index_of( & s__984, "LIKE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___100 {
            s__984: std::sync::Arc<String>
        }
        impl ClosureGroup___100 {
            fn fn__7339(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like structure intact: {}", self.s__984.clone()));
            }
        }
        let closure_group = ClosureGroup___100 {
            s__984: s__984.clone()
        };
        let fn__7339 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7339())
        };
        test___90.assert(t___7351, fn__7339.clone());
        test___90.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWildcardPatterns__1466() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___91 = temper_std::testing::Test::new();
        let mut t___7331: SafeIdentifier = sid__443("users");
        let mut t___7332: SafeIdentifier = sid__443("name");
        let q__986: Query = from(t___7331.clone()).where_like(t___7332.clone(), "%son%");
        let mut t___7337: bool = Some(q__986.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE '%son%'");
        #[derive(Clone)]
        struct ClosureGroup___101 {}
        impl ClosureGroup___101 {
            fn fn__7330(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike wildcard".to_string());
            }
        }
        let closure_group = ClosureGroup___101 {};
        let fn__7330 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7330())
        };
        test___91.assert(t___7337, fn__7330.clone());
        test___91.soft_fail_to_hard()
    }
    #[test]
    fn countAllProducesCount__1467() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___92 = temper_std::testing::Test::new();
        let f__988: SqlFragment = count_all();
        let mut t___7328: bool = Some(f__988.to_string().as_str()) == Some("COUNT(*)");
        #[derive(Clone)]
        struct ClosureGroup___102 {}
        impl ClosureGroup___102 {
            fn fn__7324(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countAll".to_string());
            }
        }
        let closure_group = ClosureGroup___102 {};
        let fn__7324 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7324())
        };
        test___92.assert(t___7328, fn__7324.clone());
        test___92.soft_fail_to_hard()
    }
    #[test]
    fn countColProducesCountField__1468() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___93 = temper_std::testing::Test::new();
        let f__990: SqlFragment = count_col(sid__443("id"));
        let mut t___7322: bool = Some(f__990.to_string().as_str()) == Some("COUNT(id)");
        #[derive(Clone)]
        struct ClosureGroup___103 {}
        impl ClosureGroup___103 {
            fn fn__7317(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countCol".to_string());
            }
        }
        let closure_group = ClosureGroup___103 {};
        let fn__7317 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7317())
        };
        test___93.assert(t___7322, fn__7317.clone());
        test___93.soft_fail_to_hard()
    }
    #[test]
    fn sumColProducesSumField__1469() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___94 = temper_std::testing::Test::new();
        let f__992: SqlFragment = sum_col(sid__443("amount"));
        let mut t___7315: bool = Some(f__992.to_string().as_str()) == Some("SUM(amount)");
        #[derive(Clone)]
        struct ClosureGroup___104 {}
        impl ClosureGroup___104 {
            fn fn__7310(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("sumCol".to_string());
            }
        }
        let closure_group = ClosureGroup___104 {};
        let fn__7310 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7310())
        };
        test___94.assert(t___7315, fn__7310.clone());
        test___94.soft_fail_to_hard()
    }
    #[test]
    fn avgColProducesAvgField__1470() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___95 = temper_std::testing::Test::new();
        let f__994: SqlFragment = avg_col(sid__443("price"));
        let mut t___7308: bool = Some(f__994.to_string().as_str()) == Some("AVG(price)");
        #[derive(Clone)]
        struct ClosureGroup___105 {}
        impl ClosureGroup___105 {
            fn fn__7303(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("avgCol".to_string());
            }
        }
        let closure_group = ClosureGroup___105 {};
        let fn__7303 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7303())
        };
        test___95.assert(t___7308, fn__7303.clone());
        test___95.soft_fail_to_hard()
    }
    #[test]
    fn minColProducesMinField__1471() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___96 = temper_std::testing::Test::new();
        let f__996: SqlFragment = min_col(sid__443("created_at"));
        let mut t___7301: bool = Some(f__996.to_string().as_str()) == Some("MIN(created_at)");
        #[derive(Clone)]
        struct ClosureGroup___106 {}
        impl ClosureGroup___106 {
            fn fn__7296(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("minCol".to_string());
            }
        }
        let closure_group = ClosureGroup___106 {};
        let fn__7296 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7296())
        };
        test___96.assert(t___7301, fn__7296.clone());
        test___96.soft_fail_to_hard()
    }
    #[test]
    fn maxColProducesMaxField__1472() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___97 = temper_std::testing::Test::new();
        let f__998: SqlFragment = max_col(sid__443("score"));
        let mut t___7294: bool = Some(f__998.to_string().as_str()) == Some("MAX(score)");
        #[derive(Clone)]
        struct ClosureGroup___107 {}
        impl ClosureGroup___107 {
            fn fn__7289(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("maxCol".to_string());
            }
        }
        let closure_group = ClosureGroup___107 {};
        let fn__7289 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7289())
        };
        test___97.assert(t___7294, fn__7289.clone());
        test___97.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithAggregate__1473() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___98 = temper_std::testing::Test::new();
        let mut t___7281: SafeIdentifier = sid__443("orders");
        let mut t___7282: SqlFragment = count_all();
        let q__1000: Query = from(t___7281.clone()).select_expr([t___7282.clone()]);
        let mut t___7287: bool = Some(q__1000.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM orders");
        #[derive(Clone)]
        struct ClosureGroup___108 {}
        impl ClosureGroup___108 {
            fn fn__7280(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr count".to_string());
            }
        }
        let closure_group = ClosureGroup___108 {};
        let fn__7280 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7280())
        };
        test___98.assert(t___7287, fn__7280.clone());
        test___98.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithMultipleExpressions__1474() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___99 = temper_std::testing::Test::new();
        let nameFrag__1002: SqlFragment = col(sid__443("users"), sid__443("name"));
        let mut t___7272: SafeIdentifier = sid__443("users");
        let mut t___7273: SqlFragment = count_all();
        let q__1003: Query = from(t___7272.clone()).select_expr([nameFrag__1002.clone(), t___7273.clone()]);
        let mut t___7278: bool = Some(q__1003.to_sql().to_string().as_str()) == Some("SELECT users.name, COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___109 {}
        impl ClosureGroup___109 {
            fn fn__7268(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr multi".to_string());
            }
        }
        let closure_group = ClosureGroup___109 {};
        let fn__7268 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7268())
        };
        test___99.assert(t___7278, fn__7268.clone());
        test___99.soft_fail_to_hard()
    }
    #[test]
    fn selectExprOverridesSelectedFields__1475() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___100 = temper_std::testing::Test::new();
        let mut t___7257: SafeIdentifier = sid__443("users");
        let mut t___7258: SafeIdentifier = sid__443("id");
        let mut t___7259: SafeIdentifier = sid__443("name");
        let q__1005: Query = from(t___7257.clone()).select([t___7258.clone(), t___7259.clone()]).select_expr([count_all()]);
        let mut t___7266: bool = Some(q__1005.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___110 {}
        impl ClosureGroup___110 {
            fn fn__7256(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr overrides select".to_string());
            }
        }
        let closure_group = ClosureGroup___110 {};
        let fn__7256 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7256())
        };
        test___100.assert(t___7266, fn__7256.clone());
        test___100.soft_fail_to_hard()
    }
    #[test]
    fn groupBySingleField__1476() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___101 = temper_std::testing::Test::new();
        let mut t___7243: SafeIdentifier = sid__443("orders");
        let mut t___7246: SqlFragment = col(sid__443("orders"), sid__443("status"));
        let mut t___7247: SqlFragment = count_all();
        let q__1007: Query = from(t___7243.clone()).select_expr([t___7246.clone(), t___7247.clone()]).group_by(sid__443("status"));
        let mut t___7254: bool = Some(q__1007.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status");
        #[derive(Clone)]
        struct ClosureGroup___111 {}
        impl ClosureGroup___111 {
            fn fn__7242(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy single".to_string());
            }
        }
        let closure_group = ClosureGroup___111 {};
        let fn__7242 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7242())
        };
        test___101.assert(t___7254, fn__7242.clone());
        test___101.soft_fail_to_hard()
    }
    #[test]
    fn groupByMultipleFields__1477() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___102 = temper_std::testing::Test::new();
        let mut t___7232: SafeIdentifier = sid__443("orders");
        let mut t___7233: SafeIdentifier = sid__443("status");
        let q__1009: Query = from(t___7232.clone()).group_by(t___7233.clone()).group_by(sid__443("category"));
        let mut t___7240: bool = Some(q__1009.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status, category");
        #[derive(Clone)]
        struct ClosureGroup___112 {}
        impl ClosureGroup___112 {
            fn fn__7231(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy multiple".to_string());
            }
        }
        let closure_group = ClosureGroup___112 {};
        let fn__7231 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7231())
        };
        test___102.assert(t___7240, fn__7231.clone());
        test___102.soft_fail_to_hard()
    }
    #[test]
    fn havingBasic__1478() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___103 = temper_std::testing::Test::new();
        let mut t___7213: SafeIdentifier = sid__443("orders");
        let mut t___7216: SqlFragment = col(sid__443("orders"), sid__443("status"));
        let mut t___7217: SqlFragment = count_all();
        let mut t___7220: Query = from(t___7213.clone()).select_expr([t___7216.clone(), t___7217.clone()]).group_by(sid__443("status"));
        let mut t___7221: SqlBuilder = SqlBuilder::new();
        t___7221.append_safe("COUNT(*) > ");
        t___7221.append_int32(5);
        let q__1011: Query = t___7220.having(t___7221.accumulated());
        let mut t___7229: bool = Some(q__1011.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status HAVING COUNT(*) > 5");
        #[derive(Clone)]
        struct ClosureGroup___113 {}
        impl ClosureGroup___113 {
            fn fn__7212(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("having basic".to_string());
            }
        }
        let closure_group = ClosureGroup___113 {};
        let fn__7212 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7212())
        };
        test___103.assert(t___7229, fn__7212.clone());
        test___103.soft_fail_to_hard()
    }
    #[test]
    fn orHaving__1480() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___104 = temper_std::testing::Test::new();
        let mut t___7194: SafeIdentifier = sid__443("orders");
        let mut t___7195: SafeIdentifier = sid__443("status");
        let mut t___7196: Query = from(t___7194.clone()).group_by(t___7195.clone());
        let mut t___7197: SqlBuilder = SqlBuilder::new();
        t___7197.append_safe("COUNT(*) > ");
        t___7197.append_int32(5);
        let mut t___7201: Query = t___7196.having(t___7197.accumulated());
        let mut t___7202: SqlBuilder = SqlBuilder::new();
        t___7202.append_safe("SUM(total) > ");
        t___7202.append_int32(1000);
        let q__1013: Query = t___7201.or_having(t___7202.accumulated());
        let mut t___7210: bool = Some(q__1013.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status HAVING COUNT(*) > 5 OR SUM(total) > 1000");
        #[derive(Clone)]
        struct ClosureGroup___114 {}
        impl ClosureGroup___114 {
            fn fn__7193(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orHaving".to_string());
            }
        }
        let closure_group = ClosureGroup___114 {};
        let fn__7193 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7193())
        };
        test___104.assert(t___7210, fn__7193.clone());
        test___104.soft_fail_to_hard()
    }
    #[test]
    fn distinctBasic__1483() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___105 = temper_std::testing::Test::new();
        let mut t___7184: SafeIdentifier = sid__443("users");
        let mut t___7185: SafeIdentifier = sid__443("name");
        let q__1015: Query = from(t___7184.clone()).select([t___7185.clone()]).distinct();
        let mut t___7191: bool = Some(q__1015.to_sql().to_string().as_str()) == Some("SELECT DISTINCT name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___115 {}
        impl ClosureGroup___115 {
            fn fn__7183(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct".to_string());
            }
        }
        let closure_group = ClosureGroup___115 {};
        let fn__7183 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7183())
        };
        test___105.assert(t___7191, fn__7183.clone());
        test___105.soft_fail_to_hard()
    }
    #[test]
    fn distinctWithWhere__1484() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___106 = temper_std::testing::Test::new();
        let mut t___7169: SafeIdentifier = sid__443("users");
        let mut t___7170: SafeIdentifier = sid__443("email");
        let mut t___7171: Query = from(t___7169.clone()).select([t___7170.clone()]);
        let mut t___7172: SqlBuilder = SqlBuilder::new();
        t___7172.append_safe("active = ");
        t___7172.append_boolean(true);
        let q__1017: Query = t___7171.r#where(t___7172.accumulated()).distinct();
        let mut t___7181: bool = Some(q__1017.to_sql().to_string().as_str()) == Some("SELECT DISTINCT email FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___116 {}
        impl ClosureGroup___116 {
            fn fn__7168(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct with where".to_string());
            }
        }
        let closure_group = ClosureGroup___116 {};
        let fn__7168 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7168())
        };
        test___106.assert(t___7181, fn__7168.clone());
        test___106.soft_fail_to_hard()
    }
    #[test]
    fn countSqlBare__1486() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___107 = temper_std::testing::Test::new();
        let q__1019: Query = from(sid__443("users"));
        let mut t___7166: bool = Some(q__1019.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___117 {}
        impl ClosureGroup___117 {
            fn fn__7161(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql bare".to_string());
            }
        }
        let closure_group = ClosureGroup___117 {};
        let fn__7161 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7161())
        };
        test___107.assert(t___7166, fn__7161.clone());
        test___107.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithWhere__1487() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___108 = temper_std::testing::Test::new();
        let mut t___7150: SafeIdentifier = sid__443("users");
        let mut t___7151: SqlBuilder = SqlBuilder::new();
        t___7151.append_safe("active = ");
        t___7151.append_boolean(true);
        let mut t___7154: SqlFragment = t___7151.accumulated();
        let q__1021: Query = from(t___7150.clone()).r#where(t___7154.clone());
        let mut t___7159: bool = Some(q__1021.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___118 {}
        impl ClosureGroup___118 {
            fn fn__7149(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with where".to_string());
            }
        }
        let closure_group = ClosureGroup___118 {};
        let fn__7149 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7149())
        };
        test___108.assert(t___7159, fn__7149.clone());
        test___108.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithJoin__1489() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___109 = temper_std::testing::Test::new();
        let mut t___7133: SafeIdentifier = sid__443("users");
        let mut t___7134: SafeIdentifier = sid__443("orders");
        let mut t___7135: SqlBuilder = SqlBuilder::new();
        t___7135.append_safe("users.id = orders.user_id");
        let mut t___7137: SqlFragment = t___7135.accumulated();
        let mut t___7138: Query = from(t___7133.clone()).inner_join(t___7134.clone(), t___7137.clone());
        let mut t___7139: SqlBuilder = SqlBuilder::new();
        t___7139.append_safe("orders.total > ");
        t___7139.append_int32(100);
        let q__1023: Query = t___7138.r#where(t___7139.accumulated());
        let mut t___7147: bool = Some(q__1023.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100");
        #[derive(Clone)]
        struct ClosureGroup___119 {}
        impl ClosureGroup___119 {
            fn fn__7132(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with join".to_string());
            }
        }
        let closure_group = ClosureGroup___119 {};
        let fn__7132 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7132())
        };
        test___109.assert(t___7147, fn__7132.clone());
        test___109.soft_fail_to_hard()
    }
    #[test]
    fn countSqlDropsOrderByLimitOffset__1492() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___110 = temper_std::testing::Test::new();
        let mut t___7119: SafeIdentifier;
        let mut t___7120: SqlBuilder;
        let mut t___7123: SqlFragment;
        let mut t___3568: Query;
        let mut t___3569: Query;
        let q__1025: Query;
        'ok___8862: {
            'orelse___1614: {
                t___7119 = sid__443("users");
                t___7120 = SqlBuilder::new();
                t___7120.append_safe("active = ");
                t___7120.append_boolean(true);
                t___7123 = t___7120.accumulated();
                t___3568 = match from(t___7119.clone()).r#where(t___7123.clone()).order_by(sid__443("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1614
                };
                t___3569 = match t___3568.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1614
                };
                q__1025 = t___3569.clone();
                break 'ok___8862;
            }
            q__1025 = panic!();
        }
        let s__1026: std::sync::Arc<String> = q__1025.count_sql().to_string();
        let mut t___7130: bool = Some(s__1026.as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___120 {
            s__1026: std::sync::Arc<String>
        }
        impl ClosureGroup___120 {
            fn fn__7118(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("countSql drops extras: {}", self.s__1026.clone()));
            }
        }
        let closure_group = ClosureGroup___120 {
            s__1026: s__1026.clone()
        };
        let fn__7118 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7118())
        };
        test___110.assert(t___7130, fn__7118.clone());
        test___110.soft_fail_to_hard()
    }
    #[test]
    fn fullAggregationQuery__1494() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___111 = temper_std::testing::Test::new();
        let mut t___7086: SafeIdentifier = sid__443("orders");
        let mut t___7089: SqlFragment = col(sid__443("orders"), sid__443("status"));
        let mut t___7090: SqlFragment = count_all();
        let mut t___7092: SqlFragment = sum_col(sid__443("total"));
        let mut t___7093: Query = from(t___7086.clone()).select_expr([t___7089.clone(), t___7090.clone(), t___7092.clone()]);
        let mut t___7094: SafeIdentifier = sid__443("users");
        let mut t___7095: SqlBuilder = SqlBuilder::new();
        t___7095.append_safe("orders.user_id = users.id");
        let mut t___7098: Query = t___7093.inner_join(t___7094.clone(), t___7095.accumulated());
        let mut t___7099: SqlBuilder = SqlBuilder::new();
        t___7099.append_safe("users.active = ");
        t___7099.append_boolean(true);
        let mut t___7105: Query = t___7098.r#where(t___7099.accumulated()).group_by(sid__443("status"));
        let mut t___7106: SqlBuilder = SqlBuilder::new();
        t___7106.append_safe("COUNT(*) > ");
        t___7106.append_int32(3);
        let q__1028: Query = t___7105.having(t___7106.accumulated()).order_by(sid__443("status"), true);
        let expected__1029: std::sync::Arc<String> = std::sync::Arc::new("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC".to_string());
        let mut t___7116: bool = Some(q__1028.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC");
        #[derive(Clone)]
        struct ClosureGroup___121 {}
        impl ClosureGroup___121 {
            fn fn__7085(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full aggregation".to_string());
            }
        }
        let closure_group = ClosureGroup___121 {};
        let fn__7085 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7085())
        };
        test___111.assert(t___7116, fn__7085.clone());
        test___111.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__1498() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___118 = temper_std::testing::Test::new();
        let mut t___3522: SafeIdentifier;
        let id__1067: SafeIdentifier;
        'ok___8863: {
            'orelse___1615: {
                t___3522 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___1615
                };
                id__1067 = t___3522.clone();
                break 'ok___8863;
            }
            id__1067 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7083: bool = Some(id__1067.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___122 {}
        impl ClosureGroup___122 {
            fn fn__7080(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___122 {};
        let fn__7080 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7080())
        };
        test___118.assert(t___7083, fn__7080.clone());
        test___118.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__1499() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___119 = temper_std::testing::Test::new();
        let didBubble__1069: bool;
        'ok___8864: {
            'orelse___1616: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___1616
                };
                didBubble__1069 = false;
                break 'ok___8864;
            }
            didBubble__1069 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___123 {}
        impl ClosureGroup___123 {
            fn fn__7077(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___123 {};
        let fn__7077 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7077())
        };
        test___119.assert(didBubble__1069, fn__7077.clone());
        test___119.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__1500() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___120 = temper_std::testing::Test::new();
        let didBubble__1071: bool;
        'ok___8865: {
            'orelse___1617: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___1617
                };
                didBubble__1071 = false;
                break 'ok___8865;
            }
            didBubble__1071 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___124 {}
        impl ClosureGroup___124 {
            fn fn__7074(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___124 {};
        let fn__7074 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7074())
        };
        test___120.assert(didBubble__1071, fn__7074.clone());
        test___120.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__1501() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___121 = temper_std::testing::Test::new();
        let cases__1073: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___125 {
            test___121: temper_std::testing::Test
        }
        impl ClosureGroup___125 {
            fn fn__7071(& self, c__1074: impl temper_core::ToArcString) {
                let c__1074 = c__1074.to_arc_string();
                let didBubble__1075: bool;
                'ok___8866: {
                    'orelse___1618: {
                        match safe_identifier(c__1074.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___1618
                        };
                        didBubble__1075 = false;
                        break 'ok___8866;
                    }
                    didBubble__1075 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___126 {
                    c__1074: std::sync::Arc<String>
                }
                impl ClosureGroup___126 {
                    fn fn__7068(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__1074.clone()));
                    }
                }
                let closure_group = ClosureGroup___126 {
                    c__1074: c__1074.clone()
                };
                let fn__7068 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__7068())
                };
                self.test___121.assert(didBubble__1075, fn__7068.clone());
            }
        }
        let closure_group = ClosureGroup___125 {
            test___121: test___121.clone()
        };
        let fn__7071 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1074: std::sync::Arc<String> | closure_group.fn__7071(c__1074))
        };
        temper_core::listed::list_for_each( & cases__1073, & ( * fn__7071.clone()));
        test___121.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__1502() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___122 = temper_std::testing::Test::new();
        let mut t___3499: SafeIdentifier;
        let mut t___3500: SafeIdentifier;
        let mut t___3501: SafeIdentifier;
        let mut t___3502: SafeIdentifier;
        let mut t___3505: SafeIdentifier;
        let mut t___3506: SafeIdentifier;
        let mut t___3510: FieldDef;
        'ok___8867: {
            'orelse___1619: {
                t___3499 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1619
                };
                t___3500 = t___3499.clone();
                break 'ok___8867;
            }
            t___3500 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___8868: {
            'orelse___1620: {
                t___3501 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1620
                };
                t___3502 = t___3501.clone();
                break 'ok___8868;
            }
            t___3502 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7058: StringField = StringField::new();
        let mut t___7059: FieldDef = FieldDef::new(t___3502.clone(), FieldType::new(t___7058.clone()), false);
        'ok___8869: {
            'orelse___1621: {
                t___3505 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1621
                };
                t___3506 = t___3505.clone();
                break 'ok___8869;
            }
            t___3506 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7060: IntField = IntField::new();
        let mut t___7061: FieldDef = FieldDef::new(t___3506.clone(), FieldType::new(t___7060.clone()), false);
        let td__1077: TableDef = TableDef::new(t___3500.clone(), [t___7059.clone(), t___7061.clone()]);
        let f__1078: FieldDef;
        'ok___8870: {
            'orelse___1622: {
                t___3510 = match td__1077.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1622
                };
                f__1078 = t___3510.clone();
                break 'ok___8870;
            }
            f__1078 = panic!();
        }
        let mut t___7066: bool = Some(f__1078.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___127 {}
        impl ClosureGroup___127 {
            fn fn__7057(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___127 {};
        let fn__7057 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7057())
        };
        test___122.assert(t___7066, fn__7057.clone());
        test___122.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__1503() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___123 = temper_std::testing::Test::new();
        let mut t___3490: SafeIdentifier;
        let mut t___3491: SafeIdentifier;
        let mut t___3492: SafeIdentifier;
        let mut t___3493: SafeIdentifier;
        'ok___8871: {
            'orelse___1623: {
                t___3490 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1623
                };
                t___3491 = t___3490.clone();
                break 'ok___8871;
            }
            t___3491 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___8872: {
            'orelse___1624: {
                t___3492 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1624
                };
                t___3493 = t___3492.clone();
                break 'ok___8872;
            }
            t___3493 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7052: StringField = StringField::new();
        let mut t___7053: FieldDef = FieldDef::new(t___3493.clone(), FieldType::new(t___7052.clone()), false);
        let td__1080: TableDef = TableDef::new(t___3491.clone(), [t___7053.clone()]);
        let didBubble__1081: bool;
        'ok___8873: {
            'orelse___1625: {
                match td__1080.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___1625
                };
                didBubble__1081 = false;
                break 'ok___8873;
            }
            didBubble__1081 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___128 {}
        impl ClosureGroup___128 {
            fn fn__7051(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___128 {};
        let fn__7051 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7051())
        };
        test___123.assert(didBubble__1081, fn__7051.clone());
        test___123.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__1504() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___124 = temper_std::testing::Test::new();
        let mut t___3478: SafeIdentifier;
        let mut t___3479: SafeIdentifier;
        let mut t___3482: SafeIdentifier;
        let mut t___3483: SafeIdentifier;
        'ok___8874: {
            'orelse___1626: {
                t___3478 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___1626
                };
                t___3479 = t___3478.clone();
                break 'ok___8874;
            }
            t___3479 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7040: StringField = StringField::new();
        let required__1083: FieldDef = FieldDef::new(t___3479.clone(), FieldType::new(t___7040.clone()), false);
        'ok___8875: {
            'orelse___1627: {
                t___3482 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___1627
                };
                t___3483 = t___3482.clone();
                break 'ok___8875;
            }
            t___3483 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___7042: StringField = StringField::new();
        let optional__1084: FieldDef = FieldDef::new(t___3483.clone(), FieldType::new(t___7042.clone()), true);
        let mut t___7046: bool = ! required__1083.nullable();
        #[derive(Clone)]
        struct ClosureGroup___129 {}
        impl ClosureGroup___129 {
            fn fn__7039(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___129 {};
        let fn__7039 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7039())
        };
        test___124.assert(t___7046, fn__7039.clone());
        let mut t___7048: bool = optional__1084.nullable();
        #[derive(Clone)]
        struct ClosureGroup___130 {}
        impl ClosureGroup___130 {
            fn fn__7038(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___130 {};
        let fn__7038 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7038())
        };
        test___124.assert(t___7048, fn__7038.clone());
        test___124.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__1505() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___128 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___131 {}
        impl ClosureGroup___131 {
            fn build__1210(& self, name__1212: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1212 = name__1212.to_arc_string();
                let mut t___7020: SqlBuilder = SqlBuilder::new();
                t___7020.append_safe("select * from hi where name = ");
                t___7020.append_string(name__1212.clone());
                return t___7020.accumulated().to_string();
            }
            fn buildWrong__1211(& self, name__1214: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1214 = name__1214.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__1214.clone()));
            }
        }
        let closure_group = ClosureGroup___131 {};
        let build__1210 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1212: std::sync::Arc<String> | closure_group.build__1210(name__1212))
        };
        let buildWrong__1211 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1214: std::sync::Arc<String> | closure_group.buildWrong__1211(name__1214))
        };
        let actual___1507: std::sync::Arc<String> = build__1210(std::sync::Arc::new("world".to_string()));
        let mut t___7030: bool = Some(actual___1507.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___132 {
            actual___1507: std::sync::Arc<String>
        }
        impl ClosureGroup___132 {
            fn fn__7027(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___1507.clone()));
            }
        }
        let closure_group = ClosureGroup___132 {
            actual___1507: actual___1507.clone()
        };
        let fn__7027 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7027())
        };
        test___128.assert(t___7030, fn__7027.clone());
        let bobbyTables__1216: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___1509: std::sync::Arc<String> = build__1210(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___7034: bool = Some(actual___1509.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___133 {
            actual___1509: std::sync::Arc<String>
        }
        impl ClosureGroup___133 {
            fn fn__7026(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___1509.clone()));
            }
        }
        let closure_group = ClosureGroup___133 {
            actual___1509: actual___1509.clone()
        };
        let fn__7026 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7026())
        };
        test___128.assert(t___7034, fn__7026.clone());
        #[derive(Clone)]
        struct ClosureGroup___134 {}
        impl ClosureGroup___134 {
            fn fn__7025(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___134 {};
        let fn__7025 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__7025())
        };
        test___128.assert(true, fn__7025.clone());
        test___128.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__1513() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___129 = temper_std::testing::Test::new();
        let mut t___6988: SqlBuilder = SqlBuilder::new();
        t___6988.append_safe("v = ");
        t___6988.append_string("");
        let actual___1514: std::sync::Arc<String> = t___6988.accumulated().to_string();
        let mut t___6994: bool = Some(actual___1514.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___135 {
            actual___1514: std::sync::Arc<String>
        }
        impl ClosureGroup___135 {
            fn fn__6987(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___1514.clone()));
            }
        }
        let closure_group = ClosureGroup___135 {
            actual___1514: actual___1514.clone()
        };
        let fn__6987 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6987())
        };
        test___129.assert(t___6994, fn__6987.clone());
        let mut t___6996: SqlBuilder = SqlBuilder::new();
        t___6996.append_safe("v = ");
        t___6996.append_string("a''b");
        let actual___1517: std::sync::Arc<String> = t___6996.accumulated().to_string();
        let mut t___7002: bool = Some(actual___1517.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___136 {
            actual___1517: std::sync::Arc<String>
        }
        impl ClosureGroup___136 {
            fn fn__6986(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___1517.clone()));
            }
        }
        let closure_group = ClosureGroup___136 {
            actual___1517: actual___1517.clone()
        };
        let fn__6986 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6986())
        };
        test___129.assert(t___7002, fn__6986.clone());
        let mut t___7004: SqlBuilder = SqlBuilder::new();
        t___7004.append_safe("v = ");
        t___7004.append_string("Hello 世界");
        let actual___1520: std::sync::Arc<String> = t___7004.accumulated().to_string();
        let mut t___7010: bool = Some(actual___1520.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___137 {
            actual___1520: std::sync::Arc<String>
        }
        impl ClosureGroup___137 {
            fn fn__6985(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___1520.clone()));
            }
        }
        let closure_group = ClosureGroup___137 {
            actual___1520: actual___1520.clone()
        };
        let fn__6985 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6985())
        };
        test___129.assert(t___7010, fn__6985.clone());
        let mut t___7012: SqlBuilder = SqlBuilder::new();
        t___7012.append_safe("v = ");
        t___7012.append_string("Line1\x0aLine2");
        let actual___1523: std::sync::Arc<String> = t___7012.accumulated().to_string();
        let mut t___7018: bool = Some(actual___1523.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___138 {
            actual___1523: std::sync::Arc<String>
        }
        impl ClosureGroup___138 {
            fn fn__6984(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___1523.clone()));
            }
        }
        let closure_group = ClosureGroup___138 {
            actual___1523: actual___1523.clone()
        };
        let fn__6984 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6984())
        };
        test___129.assert(t___7018, fn__6984.clone());
        test___129.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__1526() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___130 = temper_std::testing::Test::new();
        let mut t___3423: temper_std::temporal::Date;
        let mut t___6959: SqlBuilder = SqlBuilder::new();
        t___6959.append_safe("select ");
        t___6959.append_int32(42);
        t___6959.append_safe(", ");
        t___6959.append_int64(43);
        t___6959.append_safe(", ");
        t___6959.append_float64(19.99f64);
        t___6959.append_safe(", ");
        t___6959.append_boolean(true);
        t___6959.append_safe(", ");
        t___6959.append_boolean(false);
        let actual___1527: std::sync::Arc<String> = t___6959.accumulated().to_string();
        let mut t___6973: bool = Some(actual___1527.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___139 {
            actual___1527: std::sync::Arc<String>
        }
        impl ClosureGroup___139 {
            fn fn__6958(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___1527.clone()));
            }
        }
        let closure_group = ClosureGroup___139 {
            actual___1527: actual___1527.clone()
        };
        let fn__6958 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6958())
        };
        test___130.assert(t___6973, fn__6958.clone());
        let date__1219: temper_std::temporal::Date;
        'ok___8876: {
            'orelse___1628: {
                t___3423 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1628
                };
                date__1219 = t___3423.clone();
                break 'ok___8876;
            }
            date__1219 = panic!();
        }
        let mut t___6975: SqlBuilder = SqlBuilder::new();
        t___6975.append_safe("insert into t values (");
        t___6975.append_date(date__1219.clone());
        t___6975.append_safe(")");
        let actual___1530: std::sync::Arc<String> = t___6975.accumulated().to_string();
        let mut t___6982: bool = Some(actual___1530.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___140 {
            actual___1530: std::sync::Arc<String>
        }
        impl ClosureGroup___140 {
            fn fn__6957(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___1530.clone()));
            }
        }
        let closure_group = ClosureGroup___140 {
            actual___1530: actual___1530.clone()
        };
        let fn__6957 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6957())
        };
        test___130.assert(t___6982, fn__6957.clone());
        test___130.soft_fail_to_hard()
    }
    #[test]
    fn lists__1533() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___131 = temper_std::testing::Test::new();
        let mut t___3395: temper_std::temporal::Date;
        let mut t___3396: temper_std::temporal::Date;
        let mut t___3397: temper_std::temporal::Date;
        let mut t___3398: temper_std::temporal::Date;
        let mut t___6903: SqlBuilder = SqlBuilder::new();
        t___6903.append_safe("v IN (");
        t___6903.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___6903.append_safe(")");
        let actual___1534: std::sync::Arc<String> = t___6903.accumulated().to_string();
        let mut t___6910: bool = Some(actual___1534.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___141 {
            actual___1534: std::sync::Arc<String>
        }
        impl ClosureGroup___141 {
            fn fn__6902(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___1534.clone()));
            }
        }
        let closure_group = ClosureGroup___141 {
            actual___1534: actual___1534.clone()
        };
        let fn__6902 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6902())
        };
        test___131.assert(t___6910, fn__6902.clone());
        let mut t___6912: SqlBuilder = SqlBuilder::new();
        t___6912.append_safe("v IN (");
        t___6912.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___6912.append_safe(")");
        let actual___1537: std::sync::Arc<String> = t___6912.accumulated().to_string();
        let mut t___6919: bool = Some(actual___1537.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___142 {
            actual___1537: std::sync::Arc<String>
        }
        impl ClosureGroup___142 {
            fn fn__6901(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___1537.clone()));
            }
        }
        let closure_group = ClosureGroup___142 {
            actual___1537: actual___1537.clone()
        };
        let fn__6901 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6901())
        };
        test___131.assert(t___6919, fn__6901.clone());
        let mut t___6921: SqlBuilder = SqlBuilder::new();
        t___6921.append_safe("v IN (");
        t___6921.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___6921.append_safe(")");
        let actual___1540: std::sync::Arc<String> = t___6921.accumulated().to_string();
        let mut t___6928: bool = Some(actual___1540.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___143 {
            actual___1540: std::sync::Arc<String>
        }
        impl ClosureGroup___143 {
            fn fn__6900(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___1540.clone()));
            }
        }
        let closure_group = ClosureGroup___143 {
            actual___1540: actual___1540.clone()
        };
        let fn__6900 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6900())
        };
        test___131.assert(t___6928, fn__6900.clone());
        let mut t___6930: SqlBuilder = SqlBuilder::new();
        t___6930.append_safe("v IN (");
        t___6930.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___6930.append_safe(")");
        let actual___1543: std::sync::Arc<String> = t___6930.accumulated().to_string();
        let mut t___6937: bool = Some(actual___1543.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___144 {
            actual___1543: std::sync::Arc<String>
        }
        impl ClosureGroup___144 {
            fn fn__6899(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___1543.clone()));
            }
        }
        let closure_group = ClosureGroup___144 {
            actual___1543: actual___1543.clone()
        };
        let fn__6899 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6899())
        };
        test___131.assert(t___6937, fn__6899.clone());
        let mut t___6939: SqlBuilder = SqlBuilder::new();
        t___6939.append_safe("v IN (");
        t___6939.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___6939.append_safe(")");
        let actual___1546: std::sync::Arc<String> = t___6939.accumulated().to_string();
        let mut t___6946: bool = Some(actual___1546.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___145 {
            actual___1546: std::sync::Arc<String>
        }
        impl ClosureGroup___145 {
            fn fn__6898(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___1546.clone()));
            }
        }
        let closure_group = ClosureGroup___145 {
            actual___1546: actual___1546.clone()
        };
        let fn__6898 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6898())
        };
        test___131.assert(t___6946, fn__6898.clone());
        'ok___8877: {
            'orelse___1629: {
                t___3395 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___1629
                };
                t___3396 = t___3395.clone();
                break 'ok___8877;
            }
            t___3396 = panic!();
        }
        'ok___8878: {
            'orelse___1630: {
                t___3397 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1630
                };
                t___3398 = t___3397.clone();
                break 'ok___8878;
            }
            t___3398 = panic!();
        }
        let dates__1221: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___3396.clone(), t___3398.clone()]);
        let mut t___6948: SqlBuilder = SqlBuilder::new();
        t___6948.append_safe("v IN (");
        t___6948.append_date_list(temper_core::ToListed::to_listed(dates__1221.clone()));
        t___6948.append_safe(")");
        let actual___1549: std::sync::Arc<String> = t___6948.accumulated().to_string();
        let mut t___6955: bool = Some(actual___1549.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___146 {
            actual___1549: std::sync::Arc<String>
        }
        impl ClosureGroup___146 {
            fn fn__6897(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___1549.clone()));
            }
        }
        let closure_group = ClosureGroup___146 {
            actual___1549: actual___1549.clone()
        };
        let fn__6897 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6897())
        };
        test___131.assert(t___6955, fn__6897.clone());
        test___131.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__1552() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___132 = temper_std::testing::Test::new();
        let nan__1223: f64;
        nan__1223 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___6889: SqlBuilder = SqlBuilder::new();
        t___6889.append_safe("v = ");
        t___6889.append_float64(nan__1223);
        let actual___1553: std::sync::Arc<String> = t___6889.accumulated().to_string();
        let mut t___6895: bool = Some(actual___1553.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___147 {
            actual___1553: std::sync::Arc<String>
        }
        impl ClosureGroup___147 {
            fn fn__6888(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___1553.clone()));
            }
        }
        let closure_group = ClosureGroup___147 {
            actual___1553: actual___1553.clone()
        };
        let fn__6888 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6888())
        };
        test___132.assert(t___6895, fn__6888.clone());
        test___132.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__1556() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___133 = temper_std::testing::Test::new();
        let inf__1225: f64;
        inf__1225 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___6880: SqlBuilder = SqlBuilder::new();
        t___6880.append_safe("v = ");
        t___6880.append_float64(inf__1225);
        let actual___1557: std::sync::Arc<String> = t___6880.accumulated().to_string();
        let mut t___6886: bool = Some(actual___1557.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___148 {
            actual___1557: std::sync::Arc<String>
        }
        impl ClosureGroup___148 {
            fn fn__6879(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___1557.clone()));
            }
        }
        let closure_group = ClosureGroup___148 {
            actual___1557: actual___1557.clone()
        };
        let fn__6879 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6879())
        };
        test___133.assert(t___6886, fn__6879.clone());
        test___133.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__1560() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___134 = temper_std::testing::Test::new();
        let ninf__1227: f64;
        ninf__1227 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___6871: SqlBuilder = SqlBuilder::new();
        t___6871.append_safe("v = ");
        t___6871.append_float64(ninf__1227);
        let actual___1561: std::sync::Arc<String> = t___6871.accumulated().to_string();
        let mut t___6877: bool = Some(actual___1561.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___149 {
            actual___1561: std::sync::Arc<String>
        }
        impl ClosureGroup___149 {
            fn fn__6870(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___1561.clone()));
            }
        }
        let closure_group = ClosureGroup___149 {
            actual___1561: actual___1561.clone()
        };
        let fn__6870 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6870())
        };
        test___134.assert(t___6877, fn__6870.clone());
        test___134.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__1564() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___135 = temper_std::testing::Test::new();
        let mut t___6846: SqlBuilder = SqlBuilder::new();
        t___6846.append_safe("v = ");
        t___6846.append_float64(3.14f64);
        let actual___1565: std::sync::Arc<String> = t___6846.accumulated().to_string();
        let mut t___6852: bool = Some(actual___1565.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___150 {
            actual___1565: std::sync::Arc<String>
        }
        impl ClosureGroup___150 {
            fn fn__6845(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___1565.clone()));
            }
        }
        let closure_group = ClosureGroup___150 {
            actual___1565: actual___1565.clone()
        };
        let fn__6845 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6845())
        };
        test___135.assert(t___6852, fn__6845.clone());
        let mut t___6854: SqlBuilder = SqlBuilder::new();
        t___6854.append_safe("v = ");
        t___6854.append_float64(0.0f64);
        let actual___1568: std::sync::Arc<String> = t___6854.accumulated().to_string();
        let mut t___6860: bool = Some(actual___1568.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___151 {
            actual___1568: std::sync::Arc<String>
        }
        impl ClosureGroup___151 {
            fn fn__6844(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___1568.clone()));
            }
        }
        let closure_group = ClosureGroup___151 {
            actual___1568: actual___1568.clone()
        };
        let fn__6844 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6844())
        };
        test___135.assert(t___6860, fn__6844.clone());
        let mut t___6862: SqlBuilder = SqlBuilder::new();
        t___6862.append_safe("v = ");
        t___6862.append_float64(-42.5f64);
        let actual___1571: std::sync::Arc<String> = t___6862.accumulated().to_string();
        let mut t___6868: bool = Some(actual___1571.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___152 {
            actual___1571: std::sync::Arc<String>
        }
        impl ClosureGroup___152 {
            fn fn__6843(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___1571.clone()));
            }
        }
        let closure_group = ClosureGroup___152 {
            actual___1571: actual___1571.clone()
        };
        let fn__6843 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6843())
        };
        test___135.assert(t___6868, fn__6843.clone());
        test___135.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__1574() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___136 = temper_std::testing::Test::new();
        let mut t___3291: temper_std::temporal::Date;
        let d__1230: temper_std::temporal::Date;
        'ok___8879: {
            'orelse___1631: {
                t___3291 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___1631
                };
                d__1230 = t___3291.clone();
                break 'ok___8879;
            }
            d__1230 = panic!();
        }
        let mut t___6835: SqlBuilder = SqlBuilder::new();
        t___6835.append_safe("v = ");
        t___6835.append_date(d__1230.clone());
        let actual___1575: std::sync::Arc<String> = t___6835.accumulated().to_string();
        let mut t___6841: bool = Some(actual___1575.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___153 {
            actual___1575: std::sync::Arc<String>
        }
        impl ClosureGroup___153 {
            fn fn__6834(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___1575.clone()));
            }
        }
        let closure_group = ClosureGroup___153 {
            actual___1575: actual___1575.clone()
        };
        let fn__6834 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6834())
        };
        test___136.assert(t___6841, fn__6834.clone());
        test___136.soft_fail_to_hard()
    }
    #[test]
    fn nesting__1578() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___137 = temper_std::testing::Test::new();
        let name__1232: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___6803: SqlBuilder = SqlBuilder::new();
        t___6803.append_safe("where p.last_name = ");
        t___6803.append_string("Someone");
        let condition__1233: SqlFragment = t___6803.accumulated();
        let mut t___6807: SqlBuilder = SqlBuilder::new();
        t___6807.append_safe("select p.id from person p ");
        t___6807.append_fragment(condition__1233.clone());
        let actual___1580: std::sync::Arc<String> = t___6807.accumulated().to_string();
        let mut t___6813: bool = Some(actual___1580.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___154 {
            actual___1580: std::sync::Arc<String>
        }
        impl ClosureGroup___154 {
            fn fn__6802(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1580.clone()));
            }
        }
        let closure_group = ClosureGroup___154 {
            actual___1580: actual___1580.clone()
        };
        let fn__6802 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6802())
        };
        test___137.assert(t___6813, fn__6802.clone());
        let mut t___6815: SqlBuilder = SqlBuilder::new();
        t___6815.append_safe("select p.id from person p ");
        t___6815.append_part(SqlPart::new(condition__1233.to_source()));
        let actual___1583: std::sync::Arc<String> = t___6815.accumulated().to_string();
        let mut t___6822: bool = Some(actual___1583.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___155 {
            actual___1583: std::sync::Arc<String>
        }
        impl ClosureGroup___155 {
            fn fn__6801(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1583.clone()));
            }
        }
        let closure_group = ClosureGroup___155 {
            actual___1583: actual___1583.clone()
        };
        let fn__6801 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6801())
        };
        test___137.assert(t___6822, fn__6801.clone());
        let parts__1234: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___6826: SqlBuilder = SqlBuilder::new();
        t___6826.append_safe("select ");
        t___6826.append_part_list(parts__1234.clone());
        let actual___1586: std::sync::Arc<String> = t___6826.accumulated().to_string();
        let mut t___6832: bool = Some(actual___1586.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___156 {
            actual___1586: std::sync::Arc<String>
        }
        impl ClosureGroup___156 {
            fn fn__6800(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___1586.clone()));
            }
        }
        let closure_group = ClosureGroup___156 {
            actual___1586: actual___1586.clone()
        };
        let fn__6800 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__6800())
        };
        test___137.assert(t___6832, fn__6800.clone());
        test___137.soft_fail_to_hard()
    }
    use super::*;
}
