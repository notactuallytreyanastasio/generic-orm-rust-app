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
    pub fn new(field__646: impl temper_core::ToArcString, message__647: impl temper_core::ToArcString) -> ChangesetError {
        let field__646 = field__646.to_arc_string();
        let message__647 = message__647.to_arc_string();
        let field;
        let message;
        field = field__646.clone();
        message = message__647.clone();
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
struct NumberValidationOptsStruct {
    greater_than: Option<f64>, less_than: Option<f64>, greater_than_or_equal: Option<f64>, less_than_or_equal: Option<f64>, equal_to: Option<f64>
}
#[derive(Clone)]
pub struct NumberValidationOpts(std::sync::Arc<NumberValidationOptsStruct>);
#[derive(Clone)]
pub struct NumberValidationOptsBuilder {
    pub greater_than: Option<f64>, pub less_than: Option<f64>, pub greater_than_or_equal: Option<f64>, pub less_than_or_equal: Option<f64>, pub equal_to: Option<f64>
}
impl NumberValidationOptsBuilder {
    pub fn build(self) -> NumberValidationOpts {
        NumberValidationOpts::new(self.greater_than, self.less_than, self.greater_than_or_equal, self.less_than_or_equal, self.equal_to)
    }
}
impl NumberValidationOpts {
    pub fn new(greaterThan__654: Option<f64>, lessThan__655: Option<f64>, greaterThanOrEqual__656: Option<f64>, lessThanOrEqual__657: Option<f64>, equalTo__658: Option<f64>) -> NumberValidationOpts {
        let greater_than;
        let less_than;
        let greater_than_or_equal;
        let less_than_or_equal;
        let equal_to;
        greater_than = greaterThan__654;
        less_than = lessThan__655;
        greater_than_or_equal = greaterThanOrEqual__656;
        less_than_or_equal = lessThanOrEqual__657;
        equal_to = equalTo__658;
        let selfish = NumberValidationOpts(std::sync::Arc::new(NumberValidationOptsStruct {
                    greater_than, less_than, greater_than_or_equal, less_than_or_equal, equal_to
        }));
        return selfish;
    }
    pub fn greater_than(& self) -> Option<f64> {
        return self.0.greater_than;
    }
    pub fn less_than(& self) -> Option<f64> {
        return self.0.less_than;
    }
    pub fn greater_than_or_equal(& self) -> Option<f64> {
        return self.0.greater_than_or_equal;
    }
    pub fn less_than_or_equal(& self) -> Option<f64> {
        return self.0.less_than_or_equal;
    }
    pub fn equal_to(& self) -> Option<f64> {
        return self.0.equal_to;
    }
}
temper_core::impl_any_value_trait!(NumberValidationOpts, []);
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
    fn cast(& self, allowedFields__668: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__671: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__674: SafeIdentifier, min__675: i32, max__676: i32) -> Changeset;
    fn validate_int(& self, field__679: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__682: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__685: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__688: SafeIdentifier) -> Changeset;
    fn put_change(& self, field__691: SafeIdentifier, value__692: std::sync::Arc<String>) -> Changeset;
    fn get_change(& self, field__695: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>>;
    fn delete_change(& self, field__698: SafeIdentifier) -> Changeset;
    fn validate_inclusion(& self, field__701: SafeIdentifier, allowed__702: temper_core::List<std::sync::Arc<String>>) -> Changeset;
    fn validate_exclusion(& self, field__705: SafeIdentifier, disallowed__706: temper_core::List<std::sync::Arc<String>>) -> Changeset;
    fn validate_number(& self, field__709: SafeIdentifier, opts__710: NumberValidationOpts) -> Changeset;
    fn validate_acceptance(& self, field__713: SafeIdentifier) -> Changeset;
    fn validate_confirmation(& self, field__716: SafeIdentifier, confirmationField__717: SafeIdentifier) -> Changeset;
    fn validate_contains(& self, field__720: SafeIdentifier, substring__721: std::sync::Arc<String>) -> Changeset;
    fn validate_starts_with(& self, field__724: SafeIdentifier, prefix__725: std::sync::Arc<String>) -> Changeset;
    fn validate_ends_with(& self, field__728: SafeIdentifier, suffix__729: std::sync::Arc<String>) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__734: i32) -> temper_core::Result<SqlFragment>;
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
    fn put_change(& self, arg1: SafeIdentifier, arg2: std::sync::Arc<String>) -> Changeset {
        ChangesetTrait::put_change( & ( * self.0), arg1, arg2)
    }
    fn get_change(& self, arg1: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>> {
        ChangesetTrait::get_change( & ( * self.0), arg1)
    }
    fn delete_change(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::delete_change( & ( * self.0), arg1)
    }
    fn validate_inclusion(& self, arg1: SafeIdentifier, arg2: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        ChangesetTrait::validate_inclusion( & ( * self.0), arg1, arg2)
    }
    fn validate_exclusion(& self, arg1: SafeIdentifier, arg2: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        ChangesetTrait::validate_exclusion( & ( * self.0), arg1, arg2)
    }
    fn validate_number(& self, arg1: SafeIdentifier, arg2: NumberValidationOpts) -> Changeset {
        ChangesetTrait::validate_number( & ( * self.0), arg1, arg2)
    }
    fn validate_acceptance(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_acceptance( & ( * self.0), arg1)
    }
    fn validate_confirmation(& self, arg1: SafeIdentifier, arg2: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_confirmation( & ( * self.0), arg1, arg2)
    }
    fn validate_contains(& self, arg1: SafeIdentifier, arg2: std::sync::Arc<String>) -> Changeset {
        ChangesetTrait::validate_contains( & ( * self.0), arg1, arg2)
    }
    fn validate_starts_with(& self, arg1: SafeIdentifier, arg2: std::sync::Arc<String>) -> Changeset {
        ChangesetTrait::validate_starts_with( & ( * self.0), arg1, arg2)
    }
    fn validate_ends_with(& self, arg1: SafeIdentifier, arg2: std::sync::Arc<String>) -> Changeset {
        ChangesetTrait::validate_ends_with( & ( * self.0), arg1, arg2)
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
    pub fn cast(& self, allowedFields__750: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__750 = allowedFields__750.to_list();
        let mb__752: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__239: ChangesetImpl, mb__752: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__14193(& self, f__753: SafeIdentifier) {
                let mut t___14191: std::sync::Arc<String>;
                let mut t___14188: std::sync::Arc<String> = f__753.sql_value();
                let val__754: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__239.0.params, t___14188.clone(), std::sync::Arc::new("".to_string()));
                if ! val__754.is_empty() {
                    t___14191 = f__753.sql_value();
                    temper_core::MapBuilder::set( & self.mb__752, t___14191.clone(), val__754.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__239: self.clone(), mb__752: mb__752.clone()
        };
        let fn__14193 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__753: SafeIdentifier | closure_group.fn__14193(f__753))
        };
        temper_core::listed::list_for_each( & allowedFields__750, & ( * fn__14193.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__752), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__756: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__756 = fields__756.to_list();
        let return__405: Changeset;
        let mut t___14186: temper_core::List<ChangesetError>;
        let mut t___8102: TableDef;
        let mut t___8103: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8104: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__757: {
            if ! self.0.is_valid {
                return__405 = Changeset::new(self.clone());
                break 'fn__757;
            }
            let eb__758: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__759: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__240: ChangesetImpl, eb__758: temper_core::ListBuilder<ChangesetError>, valid__759: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__14182(& self, f__760: SafeIdentifier) {
                    let mut t___14180: ChangesetError;
                    let mut t___14177: std::sync::Arc<String> = f__760.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__240.0.changes, t___14177.clone()) {
                        t___14180 = ChangesetError::new(f__760.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__758, t___14180.clone(), None);
                        {
                            * self.valid__759.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__240: self.clone(), eb__758: eb__758.clone(), valid__759: valid__759.clone()
            };
            let fn__14182 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__760: SafeIdentifier | closure_group.fn__14182(f__760))
            };
            temper_core::listed::list_for_each( & fields__756, & ( * fn__14182.clone()));
            t___8102 = self.0.table_def.clone();
            t___8103 = self.0.params.clone();
            t___8104 = self.0.changes.clone();
            t___14186 = temper_core::ListedTrait::to_list( & eb__758);
            return__405 = Changeset::new(ChangesetImpl::new(t___8102.clone(), t___8103.clone(), t___8104.clone(), t___14186.clone(), temper_core::read_locked( & valid__759)));
        }
        return return__405.clone();
    }
    pub fn validate_length(& self, field__762: SafeIdentifier, min__763: i32, max__764: i32) -> Changeset {
        let return__406: Changeset;
        let mut t___14164: std::sync::Arc<String>;
        let mut t___14175: temper_core::List<ChangesetError>;
        let mut t___8085: bool;
        let mut t___8091: TableDef;
        let mut t___8092: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8093: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__765: {
            if ! self.0.is_valid {
                return__406 = Changeset::new(self.clone());
                break 'fn__765;
            }
            t___14164 = field__762.sql_value();
            let val__766: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14164.clone(), std::sync::Arc::new("".to_string()));
            let len__767: i32 = temper_core::string::count_between( & val__766, 0usize, val__766.len());
            if Some(len__767) < Some(min__763) {
                t___8085 = true;
            } else {
                t___8085 = Some(len__767) > Some(max__764);
            }
            if t___8085 {
                let msg__768: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__763, max__764));
                let eb__769: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__769, ChangesetError::new(field__762.sql_value(), msg__768.clone()), None);
                t___8091 = self.0.table_def.clone();
                t___8092 = self.0.params.clone();
                t___8093 = self.0.changes.clone();
                t___14175 = temper_core::ListedTrait::to_list( & eb__769);
                return__406 = Changeset::new(ChangesetImpl::new(t___8091.clone(), t___8092.clone(), t___8093.clone(), t___14175.clone(), false));
                break 'fn__765;
            }
            return__406 = Changeset::new(self.clone());
        }
        return return__406.clone();
    }
    pub fn validate_int(& self, field__771: SafeIdentifier) -> Changeset {
        let return__407: Changeset;
        let mut t___14155: std::sync::Arc<String>;
        let mut t___14162: temper_core::List<ChangesetError>;
        let mut t___8076: TableDef;
        let mut t___8077: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8078: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__772: {
            if ! self.0.is_valid {
                return__407 = Changeset::new(self.clone());
                break 'fn__772;
            }
            t___14155 = field__771.sql_value();
            let val__773: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14155.clone(), std::sync::Arc::new("".to_string()));
            if val__773.is_empty() {
                return__407 = Changeset::new(self.clone());
                break 'fn__772;
            }
            let parseOk__774: bool;
            'ok___14298: {
                'orelse___2407: {
                    match temper_core::string::to_int( & val__773, None) {
                        Ok(x) => x,
                        _ => break 'orelse___2407
                    };
                    parseOk__774 = true;
                    break 'ok___14298;
                }
                parseOk__774 = false;
            }
            if ! parseOk__774 {
                let eb__775: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__775, ChangesetError::new(field__771.sql_value(), "must be an integer"), None);
                t___8076 = self.0.table_def.clone();
                t___8077 = self.0.params.clone();
                t___8078 = self.0.changes.clone();
                t___14162 = temper_core::ListedTrait::to_list( & eb__775);
                return__407 = Changeset::new(ChangesetImpl::new(t___8076.clone(), t___8077.clone(), t___8078.clone(), t___14162.clone(), false));
                break 'fn__772;
            }
            return__407 = Changeset::new(self.clone());
        }
        return return__407.clone();
    }
    pub fn validate_int64(& self, field__777: SafeIdentifier) -> Changeset {
        let return__408: Changeset;
        let mut t___14146: std::sync::Arc<String>;
        let mut t___14153: temper_core::List<ChangesetError>;
        let mut t___8063: TableDef;
        let mut t___8064: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8065: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__778: {
            if ! self.0.is_valid {
                return__408 = Changeset::new(self.clone());
                break 'fn__778;
            }
            t___14146 = field__777.sql_value();
            let val__779: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14146.clone(), std::sync::Arc::new("".to_string()));
            if val__779.is_empty() {
                return__408 = Changeset::new(self.clone());
                break 'fn__778;
            }
            let parseOk__780: bool;
            'ok___14300: {
                'orelse___2408: {
                    match temper_core::string::to_int64( & val__779, None) {
                        Ok(x) => x,
                        _ => break 'orelse___2408
                    };
                    parseOk__780 = true;
                    break 'ok___14300;
                }
                parseOk__780 = false;
            }
            if ! parseOk__780 {
                let eb__781: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__781, ChangesetError::new(field__777.sql_value(), "must be a 64-bit integer"), None);
                t___8063 = self.0.table_def.clone();
                t___8064 = self.0.params.clone();
                t___8065 = self.0.changes.clone();
                t___14153 = temper_core::ListedTrait::to_list( & eb__781);
                return__408 = Changeset::new(ChangesetImpl::new(t___8063.clone(), t___8064.clone(), t___8065.clone(), t___14153.clone(), false));
                break 'fn__778;
            }
            return__408 = Changeset::new(self.clone());
        }
        return return__408.clone();
    }
    pub fn validate_float(& self, field__783: SafeIdentifier) -> Changeset {
        let return__409: Changeset;
        let mut t___14137: std::sync::Arc<String>;
        let mut t___14144: temper_core::List<ChangesetError>;
        let mut t___8050: TableDef;
        let mut t___8051: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8052: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__784: {
            if ! self.0.is_valid {
                return__409 = Changeset::new(self.clone());
                break 'fn__784;
            }
            t___14137 = field__783.sql_value();
            let val__785: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14137.clone(), std::sync::Arc::new("".to_string()));
            if val__785.is_empty() {
                return__409 = Changeset::new(self.clone());
                break 'fn__784;
            }
            let parseOk__786: bool;
            'ok___14302: {
                'orelse___2409: {
                    match temper_core::string::to_float64( & val__785) {
                        Ok(x) => x,
                        _ => break 'orelse___2409
                    };
                    parseOk__786 = true;
                    break 'ok___14302;
                }
                parseOk__786 = false;
            }
            if ! parseOk__786 {
                let eb__787: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__787, ChangesetError::new(field__783.sql_value(), "must be a number"), None);
                t___8050 = self.0.table_def.clone();
                t___8051 = self.0.params.clone();
                t___8052 = self.0.changes.clone();
                t___14144 = temper_core::ListedTrait::to_list( & eb__787);
                return__409 = Changeset::new(ChangesetImpl::new(t___8050.clone(), t___8051.clone(), t___8052.clone(), t___14144.clone(), false));
                break 'fn__784;
            }
            return__409 = Changeset::new(self.clone());
        }
        return return__409.clone();
    }
    pub fn validate_bool(& self, field__789: SafeIdentifier) -> Changeset {
        let return__410: Changeset;
        let mut t___14128: std::sync::Arc<String>;
        let mut t___14135: temper_core::List<ChangesetError>;
        let mut t___8025: bool;
        let mut t___8026: bool;
        let mut t___8028: bool;
        let mut t___8029: bool;
        let mut t___8031: bool;
        let mut t___8037: TableDef;
        let mut t___8038: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8039: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__790: {
            if ! self.0.is_valid {
                return__410 = Changeset::new(self.clone());
                break 'fn__790;
            }
            t___14128 = field__789.sql_value();
            let val__791: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14128.clone(), std::sync::Arc::new("".to_string()));
            if val__791.is_empty() {
                return__410 = Changeset::new(self.clone());
                break 'fn__790;
            }
            let isTrue__792: bool;
            if Some(val__791.as_str()) == Some("true") {
                isTrue__792 = true;
            } else {
                if Some(val__791.as_str()) == Some("1") {
                    t___8026 = true;
                } else {
                    if Some(val__791.as_str()) == Some("yes") {
                        t___8025 = true;
                    } else {
                        t___8025 = Some(val__791.as_str()) == Some("on");
                    }
                    t___8026 = t___8025;
                }
                isTrue__792 = t___8026;
            }
            let isFalse__793: bool;
            if Some(val__791.as_str()) == Some("false") {
                isFalse__793 = true;
            } else {
                if Some(val__791.as_str()) == Some("0") {
                    t___8029 = true;
                } else {
                    if Some(val__791.as_str()) == Some("no") {
                        t___8028 = true;
                    } else {
                        t___8028 = Some(val__791.as_str()) == Some("off");
                    }
                    t___8029 = t___8028;
                }
                isFalse__793 = t___8029;
            }
            if ! isTrue__792 {
                t___8031 = ! isFalse__793;
            } else {
                t___8031 = false;
            }
            if t___8031 {
                let eb__794: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__794, ChangesetError::new(field__789.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___8037 = self.0.table_def.clone();
                t___8038 = self.0.params.clone();
                t___8039 = self.0.changes.clone();
                t___14135 = temper_core::ListedTrait::to_list( & eb__794);
                return__410 = Changeset::new(ChangesetImpl::new(t___8037.clone(), t___8038.clone(), t___8039.clone(), t___14135.clone(), false));
                break 'fn__790;
            }
            return__410 = Changeset::new(self.clone());
        }
        return return__410.clone();
    }
    pub fn put_change(& self, field__796: SafeIdentifier, value__797: impl temper_core::ToArcString) -> Changeset {
        let value__797 = value__797.to_arc_string();
        let mut t___14116: i32;
        let mb__799: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        let pairs__800: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        let mut i__801: i32 = 0;
        'loop___14393: loop {
            t___14116 = temper_core::ListedTrait::len( & pairs__800);
            if ! (Some(i__801) < Some(t___14116)) {
                break;
            }
            temper_core::MapBuilder::set( & mb__799, temper_core::ListedTrait::get( & pairs__800, i__801).key(), temper_core::ListedTrait::get( & pairs__800, i__801).value());
            i__801 = i__801.wrapping_add(1);
        }
        temper_core::MapBuilder::set( & mb__799, field__796.sql_value(), value__797.clone());
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__799), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn get_change(& self, field__803: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>> {
        let mut t___14110: std::sync::Arc<String> = field__803.sql_value();
        if ! temper_core::MappedTrait::has( & self.0.changes, t___14110.clone()) {
            return Err(temper_core::Error::new());
        }
        let mut t___14112: std::sync::Arc<String> = field__803.sql_value();
        return Ok(temper_core::MappedTrait::get_or( & self.0.changes, t___14112.clone(), std::sync::Arc::new("".to_string())));
    }
    pub fn delete_change(& self, field__806: SafeIdentifier) -> Changeset {
        let mut t___14097: i32;
        let mb__808: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        let pairs__809: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        let mut i__810: i32 = 0;
        'loop___14394: loop {
            t___14097 = temper_core::ListedTrait::len( & pairs__809);
            if ! (Some(i__810) < Some(t___14097)) {
                break;
            }
            if Some(temper_core::ListedTrait::get( & pairs__809, i__810).key().as_str()) != Some(field__806.sql_value().as_str()) {
                temper_core::MapBuilder::set( & mb__808, temper_core::ListedTrait::get( & pairs__809, i__810).key(), temper_core::ListedTrait::get( & pairs__809, i__810).value());
            }
            i__810 = i__810.wrapping_add(1);
        }
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__808), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_inclusion(& self, field__812: SafeIdentifier, allowed__813: impl temper_core::ToList<std::sync::Arc<String>>) -> Changeset {
        let allowed__813 = allowed__813.to_list();
        let return__414: Changeset;
        let mut t___14083: std::sync::Arc<String>;
        let mut t___14085: std::sync::Arc<String>;
        let mut t___14093: temper_core::List<ChangesetError>;
        let mut t___7987: TableDef;
        let mut t___7988: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7989: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__814: {
            if ! self.0.is_valid {
                return__414 = Changeset::new(self.clone());
                break 'fn__814;
            }
            t___14083 = field__812.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___14083.clone()) {
                return__414 = Changeset::new(self.clone());
                break 'fn__814;
            }
            t___14085 = field__812.sql_value();
            let val__815: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14085.clone(), std::sync::Arc::new("".to_string()));
            let mut found__816: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___3 {
                val__815: std::sync::Arc<String>, found__816: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___3 {
                fn fn__14082(& self, a__817: impl temper_core::ToArcString) {
                    let a__817 = a__817.to_arc_string();
                    if Some(a__817.as_str()) == Some(self.val__815.as_str()) {
                        {
                            * self.found__816.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___3 {
                val__815: val__815.clone(), found__816: found__816.clone()
            };
            let fn__14082 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | a__817: std::sync::Arc<String> | closure_group.fn__14082(a__817))
            };
            temper_core::listed::list_for_each( & allowed__813, & ( * fn__14082.clone()));
            if ! temper_core::read_locked( & found__816) {
                let eb__818: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__818, ChangesetError::new(field__812.sql_value(), "is not included in the list"), None);
                t___7987 = self.0.table_def.clone();
                t___7988 = self.0.params.clone();
                t___7989 = self.0.changes.clone();
                t___14093 = temper_core::ListedTrait::to_list( & eb__818);
                return__414 = Changeset::new(ChangesetImpl::new(t___7987.clone(), t___7988.clone(), t___7989.clone(), t___14093.clone(), false));
                break 'fn__814;
            }
            return__414 = Changeset::new(self.clone());
        }
        return return__414.clone();
    }
    pub fn validate_exclusion(& self, field__820: SafeIdentifier, disallowed__821: impl temper_core::ToList<std::sync::Arc<String>>) -> Changeset {
        let disallowed__821 = disallowed__821.to_list();
        let return__415: Changeset;
        let mut t___14070: std::sync::Arc<String>;
        let mut t___14072: std::sync::Arc<String>;
        let mut t___14080: temper_core::List<ChangesetError>;
        let mut t___7973: TableDef;
        let mut t___7974: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7975: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__822: {
            if ! self.0.is_valid {
                return__415 = Changeset::new(self.clone());
                break 'fn__822;
            }
            t___14070 = field__820.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___14070.clone()) {
                return__415 = Changeset::new(self.clone());
                break 'fn__822;
            }
            t___14072 = field__820.sql_value();
            let val__823: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14072.clone(), std::sync::Arc::new("".to_string()));
            let mut found__824: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___4 {
                val__823: std::sync::Arc<String>, found__824: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___4 {
                fn fn__14069(& self, d__825: impl temper_core::ToArcString) {
                    let d__825 = d__825.to_arc_string();
                    if Some(d__825.as_str()) == Some(self.val__823.as_str()) {
                        {
                            * self.found__824.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___4 {
                val__823: val__823.clone(), found__824: found__824.clone()
            };
            let fn__14069 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | d__825: std::sync::Arc<String> | closure_group.fn__14069(d__825))
            };
            temper_core::listed::list_for_each( & disallowed__821, & ( * fn__14069.clone()));
            if temper_core::read_locked( & found__824) {
                let eb__826: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__826, ChangesetError::new(field__820.sql_value(), "is reserved"), None);
                t___7973 = self.0.table_def.clone();
                t___7974 = self.0.params.clone();
                t___7975 = self.0.changes.clone();
                t___14080 = temper_core::ListedTrait::to_list( & eb__826);
                return__415 = Changeset::new(ChangesetImpl::new(t___7973.clone(), t___7974.clone(), t___7975.clone(), t___14080.clone(), false));
                break 'fn__822;
            }
            return__415 = Changeset::new(self.clone());
        }
        return return__415.clone();
    }
    pub fn validate_number(& self, field__828: SafeIdentifier, opts__829: NumberValidationOpts) -> Changeset {
        let return__416: Changeset;
        let mut t___14019: std::sync::Arc<String>;
        let mut t___14021: std::sync::Arc<String>;
        let mut t___14027: temper_core::List<ChangesetError>;
        let mut t___14035: temper_core::List<ChangesetError>;
        let mut t___14043: temper_core::List<ChangesetError>;
        let mut t___14051: temper_core::List<ChangesetError>;
        let mut t___14059: temper_core::List<ChangesetError>;
        let mut t___14067: temper_core::List<ChangesetError>;
        let mut t___7906: TableDef;
        let mut t___7907: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7908: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7910: f64;
        let mut t___7919: TableDef;
        let mut t___7920: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7921: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7929: TableDef;
        let mut t___7930: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7931: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7939: TableDef;
        let mut t___7940: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7941: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7949: TableDef;
        let mut t___7950: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7951: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7959: TableDef;
        let mut t___7960: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7961: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__830: {
            if ! self.0.is_valid {
                return__416 = Changeset::new(self.clone());
                break 'fn__830;
            }
            t___14019 = field__828.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___14019.clone()) {
                return__416 = Changeset::new(self.clone());
                break 'fn__830;
            }
            t___14021 = field__828.sql_value();
            let val__831: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14021.clone(), std::sync::Arc::new("".to_string()));
            let parseOk__832: bool;
            'ok___14307: {
                'orelse___2410: {
                    match temper_core::string::to_float64( & val__831) {
                        Ok(x) => x,
                        _ => break 'orelse___2410
                    };
                    parseOk__832 = true;
                    break 'ok___14307;
                }
                parseOk__832 = false;
            }
            if ! parseOk__832 {
                let eb__833: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__833, ChangesetError::new(field__828.sql_value(), "must be a number"), None);
                t___7906 = self.0.table_def.clone();
                t___7907 = self.0.params.clone();
                t___7908 = self.0.changes.clone();
                t___14027 = temper_core::ListedTrait::to_list( & eb__833);
                return__416 = Changeset::new(ChangesetImpl::new(t___7906.clone(), t___7907.clone(), t___7908.clone(), t___14027.clone(), false));
                break 'fn__830;
            }
            let num__834: f64;
            'ok___14308: {
                'orelse___2411: {
                    t___7910 = match temper_core::string::to_float64( & val__831) {
                        Ok(x) => x,
                        _ => break 'orelse___2411
                    };
                    num__834 = t___7910;
                    break 'ok___14308;
                }
                num__834 = 0.0f64;
            }
            let gt__835: Option<f64> = opts__829.greater_than();
            if ! gt__835.is_none() {
                let gt___2466: f64 = gt__835.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__834), Some(gt___2466)) > 0) {
                    let eb__836: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__836, ChangesetError::new(field__828.sql_value(), std::sync::Arc::new(format!("must be greater than {}", temper_core::float64::to_string(gt___2466)))), None);
                    t___7919 = self.0.table_def.clone();
                    t___7920 = self.0.params.clone();
                    t___7921 = self.0.changes.clone();
                    t___14035 = temper_core::ListedTrait::to_list( & eb__836);
                    return__416 = Changeset::new(ChangesetImpl::new(t___7919.clone(), t___7920.clone(), t___7921.clone(), t___14035.clone(), false));
                    break 'fn__830;
                }
            }
            let lt__837: Option<f64> = opts__829.less_than();
            if ! lt__837.is_none() {
                let lt___2467: f64 = lt__837.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__834), Some(lt___2467)) < 0) {
                    let eb__838: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__838, ChangesetError::new(field__828.sql_value(), std::sync::Arc::new(format!("must be less than {}", temper_core::float64::to_string(lt___2467)))), None);
                    t___7929 = self.0.table_def.clone();
                    t___7930 = self.0.params.clone();
                    t___7931 = self.0.changes.clone();
                    t___14043 = temper_core::ListedTrait::to_list( & eb__838);
                    return__416 = Changeset::new(ChangesetImpl::new(t___7929.clone(), t___7930.clone(), t___7931.clone(), t___14043.clone(), false));
                    break 'fn__830;
                }
            }
            let gte__839: Option<f64> = opts__829.greater_than_or_equal();
            if ! gte__839.is_none() {
                let gte___2468: f64 = gte__839.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__834), Some(gte___2468)) >= 0) {
                    let eb__840: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__840, ChangesetError::new(field__828.sql_value(), std::sync::Arc::new(format!("must be greater than or equal to {}", temper_core::float64::to_string(gte___2468)))), None);
                    t___7939 = self.0.table_def.clone();
                    t___7940 = self.0.params.clone();
                    t___7941 = self.0.changes.clone();
                    t___14051 = temper_core::ListedTrait::to_list( & eb__840);
                    return__416 = Changeset::new(ChangesetImpl::new(t___7939.clone(), t___7940.clone(), t___7941.clone(), t___14051.clone(), false));
                    break 'fn__830;
                }
            }
            let lte__841: Option<f64> = opts__829.less_than_or_equal();
            if ! lte__841.is_none() {
                let lte___2469: f64 = lte__841.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__834), Some(lte___2469)) <= 0) {
                    let eb__842: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__842, ChangesetError::new(field__828.sql_value(), std::sync::Arc::new(format!("must be less than or equal to {}", temper_core::float64::to_string(lte___2469)))), None);
                    t___7949 = self.0.table_def.clone();
                    t___7950 = self.0.params.clone();
                    t___7951 = self.0.changes.clone();
                    t___14059 = temper_core::ListedTrait::to_list( & eb__842);
                    return__416 = Changeset::new(ChangesetImpl::new(t___7949.clone(), t___7950.clone(), t___7951.clone(), t___14059.clone(), false));
                    break 'fn__830;
                }
            }
            let eq__843: Option<f64> = opts__829.equal_to();
            if ! eq__843.is_none() {
                let eq___2470: f64 = eq__843.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__834), Some(eq___2470)) == 0) {
                    let eb__844: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__844, ChangesetError::new(field__828.sql_value(), std::sync::Arc::new(format!("must be equal to {}", temper_core::float64::to_string(eq___2470)))), None);
                    t___7959 = self.0.table_def.clone();
                    t___7960 = self.0.params.clone();
                    t___7961 = self.0.changes.clone();
                    t___14067 = temper_core::ListedTrait::to_list( & eb__844);
                    return__416 = Changeset::new(ChangesetImpl::new(t___7959.clone(), t___7960.clone(), t___7961.clone(), t___14067.clone(), false));
                    break 'fn__830;
                }
            }
            return__416 = Changeset::new(self.clone());
        }
        return return__416.clone();
    }
    pub fn validate_acceptance(& self, field__846: SafeIdentifier) -> Changeset {
        let return__417: Changeset;
        let mut t___14009: std::sync::Arc<String>;
        let mut t___14011: std::sync::Arc<String>;
        let mut t___14017: temper_core::List<ChangesetError>;
        let mut t___7884: bool;
        let mut t___7885: bool;
        let mut t___7892: TableDef;
        let mut t___7893: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7894: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__847: {
            if ! self.0.is_valid {
                return__417 = Changeset::new(self.clone());
                break 'fn__847;
            }
            t___14009 = field__846.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___14009.clone()) {
                return__417 = Changeset::new(self.clone());
                break 'fn__847;
            }
            t___14011 = field__846.sql_value();
            let val__848: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14011.clone(), std::sync::Arc::new("".to_string()));
            let accepted__849: bool;
            if Some(val__848.as_str()) == Some("true") {
                accepted__849 = true;
            } else {
                if Some(val__848.as_str()) == Some("1") {
                    t___7885 = true;
                } else {
                    if Some(val__848.as_str()) == Some("yes") {
                        t___7884 = true;
                    } else {
                        t___7884 = Some(val__848.as_str()) == Some("on");
                    }
                    t___7885 = t___7884;
                }
                accepted__849 = t___7885;
            }
            if ! accepted__849 {
                let eb__850: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__850, ChangesetError::new(field__846.sql_value(), "must be accepted"), None);
                t___7892 = self.0.table_def.clone();
                t___7893 = self.0.params.clone();
                t___7894 = self.0.changes.clone();
                t___14017 = temper_core::ListedTrait::to_list( & eb__850);
                return__417 = Changeset::new(ChangesetImpl::new(t___7892.clone(), t___7893.clone(), t___7894.clone(), t___14017.clone(), false));
                break 'fn__847;
            }
            return__417 = Changeset::new(self.clone());
        }
        return return__417.clone();
    }
    pub fn validate_confirmation(& self, field__852: SafeIdentifier, confirmationField__853: SafeIdentifier) -> Changeset {
        let return__418: Changeset;
        let mut t___13997: std::sync::Arc<String>;
        let mut t___13999: std::sync::Arc<String>;
        let mut t___14001: std::sync::Arc<String>;
        let mut t___14007: temper_core::List<ChangesetError>;
        let mut t___7876: TableDef;
        let mut t___7877: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7878: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__854: {
            if ! self.0.is_valid {
                return__418 = Changeset::new(self.clone());
                break 'fn__854;
            }
            t___13997 = field__852.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___13997.clone()) {
                return__418 = Changeset::new(self.clone());
                break 'fn__854;
            }
            t___13999 = field__852.sql_value();
            let val__855: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___13999.clone(), std::sync::Arc::new("".to_string()));
            t___14001 = confirmationField__853.sql_value();
            let conf__856: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___14001.clone(), std::sync::Arc::new("".to_string()));
            if Some(val__855.as_str()) != Some(conf__856.as_str()) {
                let eb__857: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__857, ChangesetError::new(confirmationField__853.sql_value(), "does not match"), None);
                t___7876 = self.0.table_def.clone();
                t___7877 = self.0.params.clone();
                t___7878 = self.0.changes.clone();
                t___14007 = temper_core::ListedTrait::to_list( & eb__857);
                return__418 = Changeset::new(ChangesetImpl::new(t___7876.clone(), t___7877.clone(), t___7878.clone(), t___14007.clone(), false));
                break 'fn__854;
            }
            return__418 = Changeset::new(self.clone());
        }
        return return__418.clone();
    }
    pub fn validate_contains(& self, field__859: SafeIdentifier, substring__860: impl temper_core::ToArcString) -> Changeset {
        let substring__860 = substring__860.to_arc_string();
        let return__419: Changeset;
        let mut t___13985: std::sync::Arc<String>;
        let mut t___13987: std::sync::Arc<String>;
        let mut t___13995: temper_core::List<ChangesetError>;
        let mut t___7861: TableDef;
        let mut t___7862: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7863: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__861: {
            if ! self.0.is_valid {
                return__419 = Changeset::new(self.clone());
                break 'fn__861;
            }
            t___13985 = field__859.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___13985.clone()) {
                return__419 = Changeset::new(self.clone());
                break 'fn__861;
            }
            t___13987 = field__859.sql_value();
            let val__862: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___13987.clone(), std::sync::Arc::new("".to_string()));
            if ! temper_core::string::index_of( & val__862, substring__860.clone(), None).is_some() {
                let eb__863: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__863, ChangesetError::new(field__859.sql_value(), "must contain the given substring"), None);
                t___7861 = self.0.table_def.clone();
                t___7862 = self.0.params.clone();
                t___7863 = self.0.changes.clone();
                t___13995 = temper_core::ListedTrait::to_list( & eb__863);
                return__419 = Changeset::new(ChangesetImpl::new(t___7861.clone(), t___7862.clone(), t___7863.clone(), t___13995.clone(), false));
                break 'fn__861;
            }
            return__419 = Changeset::new(self.clone());
        }
        return return__419.clone();
    }
    pub fn validate_starts_with(& self, field__865: SafeIdentifier, prefix__866: impl temper_core::ToArcString) -> Changeset {
        let prefix__866 = prefix__866.to_arc_string();
        let return__420: Changeset;
        let mut t___13972: std::sync::Arc<String>;
        let mut t___13974: std::sync::Arc<String>;
        let mut t___13978: i32;
        let mut t___13983: temper_core::List<ChangesetError>;
        let mut t___7845: TableDef;
        let mut t___7846: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7847: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__867: {
            if ! self.0.is_valid {
                return__420 = Changeset::new(self.clone());
                break 'fn__867;
            }
            t___13972 = field__865.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___13972.clone()) {
                return__420 = Changeset::new(self.clone());
                break 'fn__867;
            }
            t___13974 = field__865.sql_value();
            let val__868: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___13974.clone(), std::sync::Arc::new("".to_string()));
            let idx__869: Option<usize> = temper_core::string::index_of( & val__868, prefix__866.clone(), None);
            let starts__870: bool;
            if idx__869.is_some() {
                t___13978 = temper_core::string::count_between( & val__868, 0usize, temper_core::string::cast_as_index(idx__869).unwrap());
                starts__870 = Some(t___13978) == Some(0);
            } else {
                starts__870 = false;
            }
            if ! starts__870 {
                let eb__871: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__871, ChangesetError::new(field__865.sql_value(), "must start with the given prefix"), None);
                t___7845 = self.0.table_def.clone();
                t___7846 = self.0.params.clone();
                t___7847 = self.0.changes.clone();
                t___13983 = temper_core::ListedTrait::to_list( & eb__871);
                return__420 = Changeset::new(ChangesetImpl::new(t___7845.clone(), t___7846.clone(), t___7847.clone(), t___13983.clone(), false));
                break 'fn__867;
            }
            return__420 = Changeset::new(self.clone());
        }
        return return__420.clone();
    }
    pub fn validate_ends_with(& self, field__873: SafeIdentifier, suffix__874: impl temper_core::ToArcString) -> Changeset {
        let suffix__874 = suffix__874.to_arc_string();
        let return__421: Changeset;
        let mut t___13944: std::sync::Arc<String>;
        let mut t___13946: std::sync::Arc<String>;
        let mut t___13951: usize;
        let mut t___13957: temper_core::List<ChangesetError>;
        let mut t___13959: usize;
        let mut t___13960: bool;
        let mut t___13964: usize;
        let mut t___13965: usize;
        let mut t___13970: temper_core::List<ChangesetError>;
        let mut t___7810: TableDef;
        let mut t___7811: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7812: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7816: bool;
        let mut t___7827: TableDef;
        let mut t___7828: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___7829: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__875: {
            if ! self.0.is_valid {
                return__421 = Changeset::new(self.clone());
                break 'fn__875;
            }
            t___13944 = field__873.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___13944.clone()) {
                return__421 = Changeset::new(self.clone());
                break 'fn__875;
            }
            t___13946 = field__873.sql_value();
            let val__876: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___13946.clone(), std::sync::Arc::new("".to_string()));
            let valLen__877: i32 = temper_core::string::count_between( & val__876, 0usize, val__876.len());
            t___13951 = suffix__874.len();
            let suffixLen__878: i32 = temper_core::string::count_between( & suffix__874, 0usize, t___13951);
            if Some(valLen__877) < Some(suffixLen__878) {
                let eb__879: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__879, ChangesetError::new(field__873.sql_value(), "must end with the given suffix"), None);
                t___7810 = self.0.table_def.clone();
                t___7811 = self.0.params.clone();
                t___7812 = self.0.changes.clone();
                t___13957 = temper_core::ListedTrait::to_list( & eb__879);
                return__421 = Changeset::new(ChangesetImpl::new(t___7810.clone(), t___7811.clone(), t___7812.clone(), t___13957.clone(), false));
                break 'fn__875;
            }
            let skipCount__880: i32 = valLen__877.wrapping_sub(suffixLen__878);
            let mut strIdx__881: usize = 0usize;
            let mut i__882: i32 = 0;
            'loop___14400: while Some(i__882) < Some(skipCount__880) {
                t___13959 = temper_core::string::next( & val__876, strIdx__881);
                strIdx__881 = t___13959;
                i__882 = i__882.wrapping_add(1);
            }
            let mut sufIdx__883: usize = 0usize;
            let mut matches__884: bool = true;
            'loop___14401: loop {
                if matches__884 {
                    t___13960 = temper_core::string::has_index( & suffix__874, sufIdx__883);
                    t___7816 = t___13960;
                } else {
                    t___7816 = false;
                }
                if ! t___7816 {
                    break;
                }
                if ! temper_core::string::has_index( & val__876, strIdx__881) {
                    matches__884 = false;
                } else {
                    if Some(temper_core::string::get( & val__876, strIdx__881)) != Some(temper_core::string::get( & suffix__874, sufIdx__883)) {
                        matches__884 = false;
                    } else {
                        t___13964 = temper_core::string::next( & val__876, strIdx__881);
                        strIdx__881 = t___13964;
                        t___13965 = temper_core::string::next( & suffix__874, sufIdx__883);
                        sufIdx__883 = t___13965;
                    }
                }
            }
            if ! matches__884 {
                let eb__885: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__885, ChangesetError::new(field__873.sql_value(), "must end with the given suffix"), None);
                t___7827 = self.0.table_def.clone();
                t___7828 = self.0.params.clone();
                t___7829 = self.0.changes.clone();
                t___13970 = temper_core::ListedTrait::to_list( & eb__885);
                return__421 = Changeset::new(ChangesetImpl::new(t___7827.clone(), t___7828.clone(), t___7829.clone(), t___13970.clone(), false));
                break 'fn__875;
            }
            return__421 = Changeset::new(self.clone());
        }
        return return__421.clone();
    }
    fn parse_bool_sql_part(& self, val__887: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__887 = val__887.to_arc_string();
        let return__422: SqlBoolean;
        let mut t___7787: bool;
        let mut t___7788: bool;
        let mut t___7789: bool;
        let mut t___7791: bool;
        let mut t___7792: bool;
        let mut t___7793: bool;
        'fn__888: {
            if Some(val__887.as_str()) == Some("true") {
                t___7789 = true;
            } else {
                if Some(val__887.as_str()) == Some("1") {
                    t___7788 = true;
                } else {
                    if Some(val__887.as_str()) == Some("yes") {
                        t___7787 = true;
                    } else {
                        t___7787 = Some(val__887.as_str()) == Some("on");
                    }
                    t___7788 = t___7787;
                }
                t___7789 = t___7788;
            }
            if t___7789 {
                return__422 = SqlBoolean::new(true);
                break 'fn__888;
            }
            if Some(val__887.as_str()) == Some("false") {
                t___7793 = true;
            } else {
                if Some(val__887.as_str()) == Some("0") {
                    t___7792 = true;
                } else {
                    if Some(val__887.as_str()) == Some("no") {
                        t___7791 = true;
                    } else {
                        t___7791 = Some(val__887.as_str()) == Some("off");
                    }
                    t___7792 = t___7791;
                }
                t___7793 = t___7792;
            }
            if t___7793 {
                return__422 = SqlBoolean::new(false);
                break 'fn__888;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__422.clone());
    }
    fn value_to_sql_part(& self, fieldDef__890: FieldDef, val__891: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__891 = val__891.to_arc_string();
        let return__423: SqlPart;
        let mut t___7774: i32;
        let mut t___7777: i64;
        let mut t___7780: f64;
        let mut t___7785: temper_std::temporal::Date;
        'fn__892: {
            let ft__893: FieldType = fieldDef__890.field_type();
            if temper_core::is::<StringField>(ft__893.clone()) {
                return__423 = SqlPart::new(SqlString::new(val__891.clone()));
                break 'fn__892;
            }
            if temper_core::is::<IntField>(ft__893.clone()) {
                t___7774 = temper_core::string::to_int( & val__891, None) ? ;
                return__423 = SqlPart::new(SqlInt32::new(t___7774));
                break 'fn__892;
            }
            if temper_core::is::<Int64Field>(ft__893.clone()) {
                t___7777 = temper_core::string::to_int64( & val__891, None) ? ;
                return__423 = SqlPart::new(SqlInt64::new(t___7777));
                break 'fn__892;
            }
            if temper_core::is::<FloatField>(ft__893.clone()) {
                t___7780 = temper_core::string::to_float64( & val__891) ? ;
                return__423 = SqlPart::new(SqlFloat64::new(t___7780));
                break 'fn__892;
            }
            if temper_core::is::<BoolField>(ft__893.clone()) {
                return__423 = SqlPart::new(self.parse_bool_sql_part(val__891.clone()) ? );
                break 'fn__892;
            }
            if temper_core::is::<DateField>(ft__893.clone()) {
                t___7785 = temper_std::temporal::Date::from_iso_string(val__891.clone()) ? ;
                return__423 = SqlPart::new(SqlDate::new(t___7785.clone()));
                break 'fn__892;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__423.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___13892: i32;
        let mut t___13897: std::sync::Arc<String>;
        let mut t___13898: bool;
        let mut t___13903: i32;
        let mut t___13905: std::sync::Arc<String>;
        let mut t___13909: std::sync::Arc<String>;
        let mut t___13924: i32;
        let mut t___7738: bool;
        let mut t___7746: FieldDef;
        let mut t___7751: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__896: i32 = 0;
        'loop___14402: loop {
            t___13892 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__896) < Some(t___13892)) {
                break;
            }
            let f__897: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__896);
            if ! f__897.nullable() {
                t___13897 = f__897.name().sql_value();
                t___13898 = temper_core::MappedTrait::has( & self.0.changes, t___13897.clone());
                t___7738 = ! t___13898;
            } else {
                t___7738 = false;
            }
            if t___7738 {
                return Err(temper_core::Error::new());
            }
            i__896 = i__896.wrapping_add(1);
        }
        let pairs__898: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__898)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__899: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__900: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__901: i32 = 0;
        'loop___14403: loop {
            t___13903 = temper_core::ListedTrait::len( & pairs__898);
            if ! (Some(i__901) < Some(t___13903)) {
                break;
            }
            let pair__902: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__898, i__901);
            t___13905 = pair__902.key();
            t___7746 = self.0.table_def.field(t___13905.clone()) ? ;
            let fd__903: FieldDef = t___7746.clone();
            temper_core::listed::add( & colNames__899, fd__903.name().sql_value(), None);
            t___13909 = pair__902.value();
            t___7751 = self.value_to_sql_part(fd__903.clone(), t___13909.clone()) ? ;
            temper_core::listed::add( & valParts__900, t___7751.clone(), None);
            i__901 = i__901.wrapping_add(1);
        }
        let b__904: SqlBuilder = SqlBuilder::new();
        b__904.append_safe("INSERT INTO ");
        b__904.append_safe(self.0.table_def.table_name().sql_value());
        b__904.append_safe(" (");
        let mut t___13917: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__899);
        #[derive(Clone)]
        struct ClosureGroup___5 {}
        impl ClosureGroup___5 {
            fn fn__13890(& self, c__905: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__905 = c__905.to_arc_string();
                return c__905.clone();
            }
        }
        let closure_group = ClosureGroup___5 {};
        let fn__13890 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__905: std::sync::Arc<String> | closure_group.fn__13890(c__905))
        };
        b__904.append_safe(temper_core::listed::join( & t___13917, std::sync::Arc::new(", ".to_string()), & ( * fn__13890.clone())));
        b__904.append_safe(") VALUES (");
        b__904.append_part(temper_core::ListedTrait::get( & valParts__900, 0));
        let mut j__906: i32 = 1;
        'loop___14404: loop {
            t___13924 = temper_core::ListedTrait::len( & valParts__900);
            if ! (Some(j__906) < Some(t___13924)) {
                break;
            }
            b__904.append_safe(", ");
            b__904.append_part(temper_core::ListedTrait::get( & valParts__900, j__906));
            j__906 = j__906.wrapping_add(1);
        }
        b__904.append_safe(")");
        return Ok(b__904.accumulated());
    }
    pub fn to_update_sql(& self, id__908: i32) -> temper_core::Result<SqlFragment> {
        let mut t___13877: i32;
        let mut t___13880: std::sync::Arc<String>;
        let mut t___13885: std::sync::Arc<String>;
        let mut t___7719: FieldDef;
        let mut t___7725: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__910: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__910)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__911: SqlBuilder = SqlBuilder::new();
        b__911.append_safe("UPDATE ");
        b__911.append_safe(self.0.table_def.table_name().sql_value());
        b__911.append_safe(" SET ");
        let mut i__912: i32 = 0;
        'loop___14405: loop {
            t___13877 = temper_core::ListedTrait::len( & pairs__910);
            if ! (Some(i__912) < Some(t___13877)) {
                break;
            }
            if Some(i__912) > Some(0) {
                b__911.append_safe(", ");
            }
            let pair__913: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__910, i__912);
            t___13880 = pair__913.key();
            t___7719 = self.0.table_def.field(t___13880.clone()) ? ;
            let fd__914: FieldDef = t___7719.clone();
            b__911.append_safe(fd__914.name().sql_value());
            b__911.append_safe(" = ");
            t___13885 = pair__913.value();
            t___7725 = self.value_to_sql_part(fd__914.clone(), t___13885.clone()) ? ;
            b__911.append_part(t___7725.clone());
            i__912 = i__912.wrapping_add(1);
        }
        b__911.append_safe(" WHERE id = ");
        b__911.append_int32(id__908);
        return Ok(b__911.accumulated());
    }
    pub fn new(_tableDef__916: TableDef, _params__917: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__918: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__919: impl temper_core::ToList<ChangesetError>, _isValid__920: bool) -> ChangesetImpl {
        let _errors__919 = _errors__919.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__916.clone();
        params = _params__917.clone();
        changes = _changes__918.clone();
        errors = _errors__919.clone();
        is_valid = _isValid__920;
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
    fn cast(& self, allowedFields__750: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__750)
    }
    fn validate_required(& self, fields__756: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__756)
    }
    fn validate_length(& self, field__762: SafeIdentifier, min__763: i32, max__764: i32) -> Changeset {
        self.validate_length(field__762, min__763, max__764)
    }
    fn validate_int(& self, field__771: SafeIdentifier) -> Changeset {
        self.validate_int(field__771)
    }
    fn validate_int64(& self, field__777: SafeIdentifier) -> Changeset {
        self.validate_int64(field__777)
    }
    fn validate_float(& self, field__783: SafeIdentifier) -> Changeset {
        self.validate_float(field__783)
    }
    fn validate_bool(& self, field__789: SafeIdentifier) -> Changeset {
        self.validate_bool(field__789)
    }
    fn put_change(& self, field__796: SafeIdentifier, value__797: std::sync::Arc<String>) -> Changeset {
        self.put_change(field__796, value__797)
    }
    fn get_change(& self, field__803: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>> {
        self.get_change(field__803)
    }
    fn delete_change(& self, field__806: SafeIdentifier) -> Changeset {
        self.delete_change(field__806)
    }
    fn validate_inclusion(& self, field__812: SafeIdentifier, allowed__813: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        self.validate_inclusion(field__812, allowed__813)
    }
    fn validate_exclusion(& self, field__820: SafeIdentifier, disallowed__821: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        self.validate_exclusion(field__820, disallowed__821)
    }
    fn validate_number(& self, field__828: SafeIdentifier, opts__829: NumberValidationOpts) -> Changeset {
        self.validate_number(field__828, opts__829)
    }
    fn validate_acceptance(& self, field__846: SafeIdentifier) -> Changeset {
        self.validate_acceptance(field__846)
    }
    fn validate_confirmation(& self, field__852: SafeIdentifier, confirmationField__853: SafeIdentifier) -> Changeset {
        self.validate_confirmation(field__852, confirmationField__853)
    }
    fn validate_contains(& self, field__859: SafeIdentifier, substring__860: std::sync::Arc<String>) -> Changeset {
        self.validate_contains(field__859, substring__860)
    }
    fn validate_starts_with(& self, field__865: SafeIdentifier, prefix__866: std::sync::Arc<String>) -> Changeset {
        self.validate_starts_with(field__865, prefix__866)
    }
    fn validate_ends_with(& self, field__873: SafeIdentifier, suffix__874: std::sync::Arc<String>) -> Changeset {
        self.validate_ends_with(field__873, suffix__874)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__908: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__908)
    }
}
temper_core::impl_any_value_trait!(ChangesetImpl, [Changeset]);
pub enum JoinTypeEnum {
    InnerJoin(InnerJoin), LeftJoin(LeftJoin), RightJoin(RightJoin), FullJoin(FullJoin), CrossJoin(CrossJoin)
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
struct CrossJoinStruct {}
#[derive(Clone)]
pub struct CrossJoin(std::sync::Arc<CrossJoinStruct>);
impl CrossJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("CROSS JOIN".to_string());
    }
    pub fn new() -> CrossJoin {
        let selfish = CrossJoin(std::sync::Arc::new(CrossJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for CrossJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::CrossJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(CrossJoin, [JoinType]);
struct JoinClauseStruct {
    join_type: JoinType, table: SafeIdentifier, on_condition: Option<SqlFragment>
}
#[derive(Clone)]
pub struct JoinClause(std::sync::Arc<JoinClauseStruct>);
#[derive(Clone)]
pub struct JoinClauseBuilder {
    pub join_type: JoinType, pub table: SafeIdentifier, pub on_condition: Option<SqlFragment>
}
impl JoinClauseBuilder {
    pub fn build(self) -> JoinClause {
        JoinClause::new(self.join_type, self.table, self.on_condition)
    }
}
impl JoinClause {
    pub fn new(joinType__1145: JoinType, table__1146: SafeIdentifier, onCondition__1147: Option<SqlFragment>) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__1145.clone();
        table = table__1146.clone();
        on_condition = onCondition__1147.clone();
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
    pub fn on_condition(& self) -> Option<SqlFragment> {
        return self.0.on_condition.clone();
    }
}
temper_core::impl_any_value_trait!(JoinClause, []);
pub enum NullsPositionEnum {
    NullsFirst(NullsFirst), NullsLast(NullsLast)
}
pub trait NullsPositionTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> NullsPositionEnum;
    fn clone_boxed(& self) -> NullsPosition;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct NullsPosition(std::sync::Arc<dyn NullsPositionTrait>);
impl NullsPosition {
    pub fn new(selfish: impl NullsPositionTrait + 'static) -> NullsPosition {
        NullsPosition(std::sync::Arc::new(selfish))
    }
}
impl NullsPositionTrait for NullsPosition {
    fn as_enum(& self) -> NullsPositionEnum {
        NullsPositionTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> NullsPosition {
        NullsPositionTrait::clone_boxed( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        NullsPositionTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(NullsPosition);
impl std::ops::Deref for NullsPosition {
    type Target = dyn NullsPositionTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct NullsFirstStruct {}
#[derive(Clone)]
pub struct NullsFirst(std::sync::Arc<NullsFirstStruct>);
impl NullsFirst {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" NULLS FIRST".to_string());
    }
    pub fn new() -> NullsFirst {
        let selfish = NullsFirst(std::sync::Arc::new(NullsFirstStruct {}));
        return selfish;
    }
}
impl NullsPositionTrait for NullsFirst {
    fn as_enum(& self) -> NullsPositionEnum {
        NullsPositionEnum::NullsFirst(self.clone())
    }
    fn clone_boxed(& self) -> NullsPosition {
        NullsPosition::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(NullsFirst, [NullsPosition]);
struct NullsLastStruct {}
#[derive(Clone)]
pub struct NullsLast(std::sync::Arc<NullsLastStruct>);
impl NullsLast {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" NULLS LAST".to_string());
    }
    pub fn new() -> NullsLast {
        let selfish = NullsLast(std::sync::Arc::new(NullsLastStruct {}));
        return selfish;
    }
}
impl NullsPositionTrait for NullsLast {
    fn as_enum(& self) -> NullsPositionEnum {
        NullsPositionEnum::NullsLast(self.clone())
    }
    fn clone_boxed(& self) -> NullsPosition {
        NullsPosition::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(NullsLast, [NullsPosition]);
struct OrderClauseStruct {
    field: SafeIdentifier, ascending: bool, nulls_pos: Option<NullsPosition>
}
#[derive(Clone)]
pub struct OrderClause(std::sync::Arc<OrderClauseStruct>);
#[derive(Clone)]
pub struct OrderClauseBuilder {
    pub field: SafeIdentifier, pub ascending: bool, pub nulls_pos: Option<NullsPosition>
}
impl OrderClauseBuilder {
    pub fn build(self) -> OrderClause {
        OrderClause::new(self.field, self.ascending, self.nulls_pos)
    }
}
impl OrderClause {
    pub fn new(field__1160: SafeIdentifier, ascending__1161: bool, nullsPos__1162: Option<NullsPosition>) -> OrderClause {
        let field;
        let ascending;
        let nulls_pos;
        field = field__1160.clone();
        ascending = ascending__1161;
        nulls_pos = nullsPos__1162.clone();
        let selfish = OrderClause(std::sync::Arc::new(OrderClauseStruct {
                    field, ascending, nulls_pos
        }));
        return selfish;
    }
    pub fn field(& self) -> SafeIdentifier {
        return self.0.field.clone();
    }
    pub fn ascending(& self) -> bool {
        return self.0.ascending;
    }
    pub fn nulls_pos(& self) -> Option<NullsPosition> {
        return self.0.nulls_pos.clone();
    }
}
temper_core::impl_any_value_trait!(OrderClause, []);
pub enum LockModeEnum {
    ForUpdate(ForUpdate), ForShare(ForShare)
}
pub trait LockModeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> LockModeEnum;
    fn clone_boxed(& self) -> LockMode;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct LockMode(std::sync::Arc<dyn LockModeTrait>);
impl LockMode {
    pub fn new(selfish: impl LockModeTrait + 'static) -> LockMode {
        LockMode(std::sync::Arc::new(selfish))
    }
}
impl LockModeTrait for LockMode {
    fn as_enum(& self) -> LockModeEnum {
        LockModeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> LockMode {
        LockModeTrait::clone_boxed( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        LockModeTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(LockMode);
impl std::ops::Deref for LockMode {
    type Target = dyn LockModeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ForUpdateStruct {}
#[derive(Clone)]
pub struct ForUpdate(std::sync::Arc<ForUpdateStruct>);
impl ForUpdate {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" FOR UPDATE".to_string());
    }
    pub fn new() -> ForUpdate {
        let selfish = ForUpdate(std::sync::Arc::new(ForUpdateStruct {}));
        return selfish;
    }
}
impl LockModeTrait for ForUpdate {
    fn as_enum(& self) -> LockModeEnum {
        LockModeEnum::ForUpdate(self.clone())
    }
    fn clone_boxed(& self) -> LockMode {
        LockMode::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(ForUpdate, [LockMode]);
struct ForShareStruct {}
#[derive(Clone)]
pub struct ForShare(std::sync::Arc<ForShareStruct>);
impl ForShare {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" FOR SHARE".to_string());
    }
    pub fn new() -> ForShare {
        let selfish = ForShare(std::sync::Arc::new(ForShareStruct {}));
        return selfish;
    }
}
impl LockModeTrait for ForShare {
    fn as_enum(& self) -> LockModeEnum {
        LockModeEnum::ForShare(self.clone())
    }
    fn clone_boxed(& self) -> LockMode {
        LockMode::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(ForShare, [LockMode]);
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
    pub fn new(_condition__1181: SqlFragment) -> AndCondition {
        let condition;
        condition = _condition__1181.clone();
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
    pub fn new(_condition__1188: SqlFragment) -> OrCondition {
        let condition;
        condition = _condition__1188.clone();
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
    table_name: SafeIdentifier, conditions: temper_core::List<WhereClause>, selected_fields: temper_core::List<SafeIdentifier>, order_clauses: temper_core::List<OrderClause>, limit_val: Option<i32>, offset_val: Option<i32>, join_clauses: temper_core::List<JoinClause>, group_by_fields: temper_core::List<SafeIdentifier>, having_conditions: temper_core::List<WhereClause>, is_distinct: bool, select_exprs: temper_core::List<SqlFragment>, lock_mode: Option<LockMode>
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<QueryStruct>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<WhereClause>, pub selected_fields: temper_core::List<SafeIdentifier>, pub order_clauses: temper_core::List<OrderClause>, pub limit_val: Option<i32>, pub offset_val: Option<i32>, pub join_clauses: temper_core::List<JoinClause>, pub group_by_fields: temper_core::List<SafeIdentifier>, pub having_conditions: temper_core::List<WhereClause>, pub is_distinct: bool, pub select_exprs: temper_core::List<SqlFragment>, pub lock_mode: Option<LockMode>
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.table_name, self.conditions, self.selected_fields, self.order_clauses, self.limit_val, self.offset_val, self.join_clauses, self.group_by_fields, self.having_conditions, self.is_distinct, self.select_exprs, self.lock_mode)
    }
}
impl Query {
    pub fn r#where(& self, condition__1202: SqlFragment) -> Query {
        let nb__1204: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1204, WhereClause::new(AndCondition::new(condition__1202.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1204), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn or_where(& self, condition__1206: SqlFragment) -> Query {
        let nb__1208: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1208, WhereClause::new(OrCondition::new(condition__1206.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1208), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn where_null(& self, field__1210: SafeIdentifier) -> Query {
        let b__1212: SqlBuilder = SqlBuilder::new();
        b__1212.append_safe(field__1210.sql_value());
        b__1212.append_safe(" IS NULL");
        let mut t___12997: SqlFragment = b__1212.accumulated();
        return self.r#where(t___12997.clone());
    }
    pub fn where_not_null(& self, field__1214: SafeIdentifier) -> Query {
        let b__1216: SqlBuilder = SqlBuilder::new();
        b__1216.append_safe(field__1214.sql_value());
        b__1216.append_safe(" IS NOT NULL");
        let mut t___12991: SqlFragment = b__1216.accumulated();
        return self.r#where(t___12991.clone());
    }
    pub fn where_in(& self, field__1218: SafeIdentifier, values__1219: impl temper_core::ToList<SqlPart>) -> Query {
        let values__1219 = values__1219.to_list();
        let return__495: Query;
        let mut t___12972: SqlFragment;
        let mut t___12980: i32;
        let mut t___12985: SqlFragment;
        'fn__1220: {
            if temper_core::ListedTrait::is_empty( & values__1219) {
                let b__1221: SqlBuilder = SqlBuilder::new();
                b__1221.append_safe("1 = 0");
                t___12972 = b__1221.accumulated();
                return__495 = self.r#where(t___12972.clone());
                break 'fn__1220;
            }
            let b__1222: SqlBuilder = SqlBuilder::new();
            b__1222.append_safe(field__1218.sql_value());
            b__1222.append_safe(" IN (");
            b__1222.append_part(temper_core::ListedTrait::get( & values__1219, 0));
            let mut i__1223: i32 = 1;
            'loop___14416: loop {
                t___12980 = temper_core::ListedTrait::len( & values__1219);
                if ! (Some(i__1223) < Some(t___12980)) {
                    break;
                }
                b__1222.append_safe(", ");
                b__1222.append_part(temper_core::ListedTrait::get( & values__1219, i__1223));
                i__1223 = i__1223.wrapping_add(1);
            }
            b__1222.append_safe(")");
            t___12985 = b__1222.accumulated();
            return__495 = self.r#where(t___12985.clone());
        }
        return return__495.clone();
    }
    pub fn where_in_subquery(& self, field__1225: SafeIdentifier, sub__1226: Query) -> Query {
        let b__1228: SqlBuilder = SqlBuilder::new();
        b__1228.append_safe(field__1225.sql_value());
        b__1228.append_safe(" IN (");
        b__1228.append_fragment(sub__1226.to_sql());
        b__1228.append_safe(")");
        let mut t___12967: SqlFragment = b__1228.accumulated();
        return self.r#where(t___12967.clone());
    }
    pub fn where_not(& self, condition__1230: SqlFragment) -> Query {
        let b__1232: SqlBuilder = SqlBuilder::new();
        b__1232.append_safe("NOT (");
        b__1232.append_fragment(condition__1230.clone());
        b__1232.append_safe(")");
        let mut t___12958: SqlFragment = b__1232.accumulated();
        return self.r#where(t___12958.clone());
    }
    pub fn where_between(& self, field__1234: SafeIdentifier, low__1235: SqlPart, high__1236: SqlPart) -> Query {
        let b__1238: SqlBuilder = SqlBuilder::new();
        b__1238.append_safe(field__1234.sql_value());
        b__1238.append_safe(" BETWEEN ");
        b__1238.append_part(low__1235.clone());
        b__1238.append_safe(" AND ");
        b__1238.append_part(high__1236.clone());
        let mut t___12952: SqlFragment = b__1238.accumulated();
        return self.r#where(t___12952.clone());
    }
    pub fn where_like(& self, field__1240: SafeIdentifier, pattern__1241: impl temper_core::ToArcString) -> Query {
        let pattern__1241 = pattern__1241.to_arc_string();
        let b__1243: SqlBuilder = SqlBuilder::new();
        b__1243.append_safe(field__1240.sql_value());
        b__1243.append_safe(" LIKE ");
        b__1243.append_string(pattern__1241.clone());
        let mut t___12943: SqlFragment = b__1243.accumulated();
        return self.r#where(t___12943.clone());
    }
    pub fn where_i_like(& self, field__1245: SafeIdentifier, pattern__1246: impl temper_core::ToArcString) -> Query {
        let pattern__1246 = pattern__1246.to_arc_string();
        let b__1248: SqlBuilder = SqlBuilder::new();
        b__1248.append_safe(field__1245.sql_value());
        b__1248.append_safe(" ILIKE ");
        b__1248.append_string(pattern__1246.clone());
        let mut t___12936: SqlFragment = b__1248.accumulated();
        return self.r#where(t___12936.clone());
    }
    pub fn select(& self, fields__1250: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__1250 = fields__1250.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__1250.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn select_expr(& self, exprs__1253: impl temper_core::ToList<SqlFragment>) -> Query {
        let exprs__1253 = exprs__1253.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, exprs__1253.clone(), self.0.lock_mode.clone());
    }
    pub fn order_by(& self, field__1256: SafeIdentifier, ascending__1257: bool) -> Query {
        let nb__1259: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__1259, OrderClause::new(field__1256.clone(), ascending__1257, None), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__1259), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn order_by_nulls(& self, field__1261: SafeIdentifier, ascending__1262: bool, nulls__1263: NullsPosition) -> Query {
        let nb__1265: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__1265, OrderClause::new(field__1261.clone(), ascending__1262, Some(nulls__1263.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__1265), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn limit(& self, n__1267: i32) -> temper_core::Result<Query> {
        if Some(n__1267) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__1267), self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone()));
    }
    pub fn offset(& self, n__1270: i32) -> temper_core::Result<Query> {
        if Some(n__1270) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__1270), self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone()));
    }
    pub fn join(& self, joinType__1273: JoinType, table__1274: SafeIdentifier, onCondition__1275: SqlFragment) -> Query {
        let nb__1277: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__1277, JoinClause::new(joinType__1273.clone(), table__1274.clone(), Some(onCondition__1275.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__1277), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn inner_join(& self, table__1279: SafeIdentifier, onCondition__1280: SqlFragment) -> Query {
        let mut t___12898: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___12898.clone()), table__1279.clone(), onCondition__1280.clone());
    }
    pub fn left_join(& self, table__1283: SafeIdentifier, onCondition__1284: SqlFragment) -> Query {
        let mut t___12896: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___12896.clone()), table__1283.clone(), onCondition__1284.clone());
    }
    pub fn right_join(& self, table__1287: SafeIdentifier, onCondition__1288: SqlFragment) -> Query {
        let mut t___12894: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___12894.clone()), table__1287.clone(), onCondition__1288.clone());
    }
    pub fn full_join(& self, table__1291: SafeIdentifier, onCondition__1292: SqlFragment) -> Query {
        let mut t___12892: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___12892.clone()), table__1291.clone(), onCondition__1292.clone());
    }
    pub fn cross_join(& self, table__1295: SafeIdentifier) -> Query {
        let nb__1297: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__1297, JoinClause::new(JoinType::new(CrossJoin::new()), table__1295.clone(), None), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__1297), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn group_by(& self, field__1299: SafeIdentifier) -> Query {
        let nb__1301: temper_core::ListBuilder<SafeIdentifier> = temper_core::ListedTrait::to_list_builder( & self.0.group_by_fields);
        temper_core::listed::add( & nb__1301, field__1299.clone(), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1301), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn having(& self, condition__1303: SqlFragment) -> Query {
        let nb__1305: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__1305, WhereClause::new(AndCondition::new(condition__1303.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__1305), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn or_having(& self, condition__1307: SqlFragment) -> Query {
        let nb__1309: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__1309, WhereClause::new(OrCondition::new(condition__1307.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__1309), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn distinct(& self) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), true, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn lock(& self, mode__1313: LockMode) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), Some(mode__1313.clone()));
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___12783: i32;
        let mut t___12802: i32;
        let mut t___12821: i32;
        let b__1317: SqlBuilder = SqlBuilder::new();
        if self.0.is_distinct {
            b__1317.append_safe("SELECT DISTINCT ");
        } else {
            b__1317.append_safe("SELECT ");
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.select_exprs) {
            b__1317.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, 0));
            let mut i__1318: i32 = 1;
            'loop___14453: loop {
                t___12783 = temper_core::ListedTrait::len( & self.0.select_exprs);
                if ! (Some(i__1318) < Some(t___12783)) {
                    break;
                }
                b__1317.append_safe(", ");
                b__1317.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, i__1318));
                i__1318 = i__1318.wrapping_add(1);
            }
        } else {
            if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
                b__1317.append_safe("*");
            } else {
                #[derive(Clone)]
                struct ClosureGroup___6 {}
                impl ClosureGroup___6 {
                    fn fn__12776(& self, f__1319: SafeIdentifier) -> std::sync::Arc<String> {
                        return f__1319.sql_value();
                    }
                }
                let closure_group = ClosureGroup___6 {};
                let fn__12776 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | f__1319: SafeIdentifier | closure_group.fn__12776(f__1319))
                };
                b__1317.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__12776.clone())));
            }
        }
        b__1317.append_safe(" FROM ");
        b__1317.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___7 {
            b__1317: SqlBuilder
        }
        impl ClosureGroup___7 {
            fn fn__12775(& self, jc__1320: JoinClause) {
                self.b__1317.append_safe(" ");
                let mut t___12763: std::sync::Arc<String> = jc__1320.join_type().keyword();
                self.b__1317.append_safe(t___12763.clone());
                self.b__1317.append_safe(" ");
                let mut t___12767: std::sync::Arc<String> = jc__1320.table().sql_value();
                self.b__1317.append_safe(t___12767.clone());
                let oc__1321: Option<SqlFragment> = jc__1320.on_condition();
                if ! oc__1321.is_none() {
                    let oc___2478: SqlFragment = oc__1321.clone().unwrap();
                    self.b__1317.append_safe(" ON ");
                    self.b__1317.append_fragment(oc___2478.clone());
                }
            }
        }
        let closure_group = ClosureGroup___7 {
            b__1317: b__1317.clone()
        };
        let fn__12775 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__1320: JoinClause | closure_group.fn__12775(jc__1320))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__12775.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__1317.append_safe(" WHERE ");
            b__1317.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__1322: i32 = 1;
            'loop___14455: loop {
                t___12802 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__1322) < Some(t___12802)) {
                    break;
                }
                b__1317.append_safe(" ");
                b__1317.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1322).keyword());
                b__1317.append_safe(" ");
                b__1317.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1322).condition());
                i__1322 = i__1322.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__1317.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___8 {}
            impl ClosureGroup___8 {
                fn fn__12774(& self, f__1323: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__1323.sql_value();
                }
            }
            let closure_group = ClosureGroup___8 {};
            let fn__12774 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__1323: SafeIdentifier | closure_group.fn__12774(f__1323))
            };
            b__1317.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__12774.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__1317.append_safe(" HAVING ");
            b__1317.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__1324: i32 = 1;
            'loop___14456: loop {
                t___12821 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__1324) < Some(t___12821)) {
                    break;
                }
                b__1317.append_safe(" ");
                b__1317.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__1324).keyword());
                b__1317.append_safe(" ");
                b__1317.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__1324).condition());
                i__1324 = i__1324.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__1317.append_safe(" ORDER BY ");
            let mut first__1325: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___9 {
                first__1325: std::sync::Arc<std::sync::RwLock<bool>>, b__1317: SqlBuilder
            }
            impl ClosureGroup___9 {
                fn fn__12773(& self, orc__1326: OrderClause) {
                    let mut t___12758: std::sync::Arc<String>;
                    let mut t___6760: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__1325) {
                        self.b__1317.append_safe(", ");
                    }
                    {
                        * self.first__1325.write().unwrap() = false;
                    }
                    let mut t___12753: std::sync::Arc<String> = orc__1326.field().sql_value();
                    self.b__1317.append_safe(t___12753.clone());
                    if orc__1326.ascending() {
                        t___6760 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___6760 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__1317.append_safe(t___6760.clone());
                    let np__1327: Option<NullsPosition> = orc__1326.nulls_pos();
                    if ! np__1327.is_none() {
                        t___12758 = np__1327.clone().unwrap().keyword();
                        self.b__1317.append_safe(t___12758.clone());
                    }
                }
            }
            let closure_group = ClosureGroup___9 {
                first__1325: first__1325.clone(), b__1317: b__1317.clone()
            };
            let fn__12773 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | orc__1326: OrderClause | closure_group.fn__12773(orc__1326))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__12773.clone()));
        }
        let lv__1328: Option<i32> = self.0.limit_val;
        if ! lv__1328.is_none() {
            let lv___2480: i32 = lv__1328.unwrap();
            b__1317.append_safe(" LIMIT ");
            b__1317.append_int32(lv___2480);
        }
        let ov__1329: Option<i32> = self.0.offset_val;
        if ! ov__1329.is_none() {
            let ov___2481: i32 = ov__1329.unwrap();
            b__1317.append_safe(" OFFSET ");
            b__1317.append_int32(ov___2481);
        }
        let lm__1330: Option<LockMode> = self.0.lock_mode.clone();
        if ! lm__1330.is_none() {
            b__1317.append_safe(lm__1330.clone().unwrap().keyword());
        }
        return b__1317.accumulated();
    }
    pub fn count_sql(& self) -> SqlFragment {
        let mut t___12722: i32;
        let mut t___12741: i32;
        let b__1333: SqlBuilder = SqlBuilder::new();
        b__1333.append_safe("SELECT COUNT(*) FROM ");
        b__1333.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___10 {
            b__1333: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__12710(& self, jc__1334: JoinClause) {
                self.b__1333.append_safe(" ");
                let mut t___12700: std::sync::Arc<String> = jc__1334.join_type().keyword();
                self.b__1333.append_safe(t___12700.clone());
                self.b__1333.append_safe(" ");
                let mut t___12704: std::sync::Arc<String> = jc__1334.table().sql_value();
                self.b__1333.append_safe(t___12704.clone());
                let oc2__1335: Option<SqlFragment> = jc__1334.on_condition();
                if ! oc2__1335.is_none() {
                    let oc2___2483: SqlFragment = oc2__1335.clone().unwrap();
                    self.b__1333.append_safe(" ON ");
                    self.b__1333.append_fragment(oc2___2483.clone());
                }
            }
        }
        let closure_group = ClosureGroup___10 {
            b__1333: b__1333.clone()
        };
        let fn__12710 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__1334: JoinClause | closure_group.fn__12710(jc__1334))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__12710.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__1333.append_safe(" WHERE ");
            b__1333.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__1336: i32 = 1;
            'loop___14462: loop {
                t___12722 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__1336) < Some(t___12722)) {
                    break;
                }
                b__1333.append_safe(" ");
                b__1333.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1336).keyword());
                b__1333.append_safe(" ");
                b__1333.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1336).condition());
                i__1336 = i__1336.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__1333.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___11 {}
            impl ClosureGroup___11 {
                fn fn__12709(& self, f__1337: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__1337.sql_value();
                }
            }
            let closure_group = ClosureGroup___11 {};
            let fn__12709 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__1337: SafeIdentifier | closure_group.fn__12709(f__1337))
            };
            b__1333.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__12709.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__1333.append_safe(" HAVING ");
            b__1333.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__1338: i32 = 1;
            'loop___14463: loop {
                t___12741 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__1338) < Some(t___12741)) {
                    break;
                }
                b__1333.append_safe(" ");
                b__1333.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__1338).keyword());
                b__1333.append_safe(" ");
                b__1333.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__1338).condition());
                i__1338 = i__1338.wrapping_add(1);
            }
        }
        return b__1333.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__1340: i32) -> temper_core::Result<SqlFragment> {
        let return__520: SqlFragment;
        let mut t___6710: Query;
        if Some(defaultLimit__1340) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__520 = self.to_sql();
        } else {
            t___6710 = self.limit(defaultLimit__1340) ? ;
            return__520 = t___6710.to_sql();
        }
        return Ok(return__520.clone());
    }
    pub fn new(tableName__1343: SafeIdentifier, conditions__1344: impl temper_core::ToList<WhereClause>, selectedFields__1345: impl temper_core::ToList<SafeIdentifier>, orderClauses__1346: impl temper_core::ToList<OrderClause>, limitVal__1347: Option<i32>, offsetVal__1348: Option<i32>, joinClauses__1349: impl temper_core::ToList<JoinClause>, groupByFields__1350: impl temper_core::ToList<SafeIdentifier>, havingConditions__1351: impl temper_core::ToList<WhereClause>, isDistinct__1352: bool, selectExprs__1353: impl temper_core::ToList<SqlFragment>, lockMode__1354: Option<LockMode>) -> Query {
        let conditions__1344 = conditions__1344.to_list();
        let selectedFields__1345 = selectedFields__1345.to_list();
        let orderClauses__1346 = orderClauses__1346.to_list();
        let joinClauses__1349 = joinClauses__1349.to_list();
        let groupByFields__1350 = groupByFields__1350.to_list();
        let havingConditions__1351 = havingConditions__1351.to_list();
        let selectExprs__1353 = selectExprs__1353.to_list();
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
        let lock_mode;
        table_name = tableName__1343.clone();
        conditions = conditions__1344.clone();
        selected_fields = selectedFields__1345.clone();
        order_clauses = orderClauses__1346.clone();
        limit_val = limitVal__1347;
        offset_val = offsetVal__1348;
        join_clauses = joinClauses__1349.clone();
        group_by_fields = groupByFields__1350.clone();
        having_conditions = havingConditions__1351.clone();
        is_distinct = isDistinct__1352;
        select_exprs = selectExprs__1353.clone();
        lock_mode = lockMode__1354.clone();
        let selfish = Query(std::sync::Arc::new(QueryStruct {
                    table_name, conditions, selected_fields, order_clauses, limit_val, offset_val, join_clauses, group_by_fields, having_conditions, is_distinct, select_exprs, lock_mode
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
    pub fn lock_mode(& self) -> Option<LockMode> {
        return self.0.lock_mode.clone();
    }
}
temper_core::impl_any_value_trait!(Query, []);
struct SetClauseStruct {
    field: SafeIdentifier, value: SqlPart
}
#[derive(Clone)]
pub struct SetClause(std::sync::Arc<SetClauseStruct>);
#[derive(Clone)]
pub struct SetClauseBuilder {
    pub field: SafeIdentifier, pub value: SqlPart
}
impl SetClauseBuilder {
    pub fn build(self) -> SetClause {
        SetClause::new(self.field, self.value)
    }
}
impl SetClause {
    pub fn new(field__1404: SafeIdentifier, value__1405: SqlPart) -> SetClause {
        let field;
        let value;
        field = field__1404.clone();
        value = value__1405.clone();
        let selfish = SetClause(std::sync::Arc::new(SetClauseStruct {
                    field, value
        }));
        return selfish;
    }
    pub fn field(& self) -> SafeIdentifier {
        return self.0.field.clone();
    }
    pub fn value(& self) -> SqlPart {
        return self.0.value.clone();
    }
}
temper_core::impl_any_value_trait!(SetClause, []);
struct UpdateQueryStruct {
    table_name: SafeIdentifier, set_clauses: temper_core::List<SetClause>, conditions: temper_core::List<WhereClause>, limit_val: Option<i32>
}
#[derive(Clone)]
pub struct UpdateQuery(std::sync::Arc<UpdateQueryStruct>);
#[derive(Clone)]
pub struct UpdateQueryBuilder {
    pub table_name: SafeIdentifier, pub set_clauses: temper_core::List<SetClause>, pub conditions: temper_core::List<WhereClause>, pub limit_val: Option<i32>
}
impl UpdateQueryBuilder {
    pub fn build(self) -> UpdateQuery {
        UpdateQuery::new(self.table_name, self.set_clauses, self.conditions, self.limit_val)
    }
}
impl UpdateQuery {
    pub fn set(& self, field__1411: SafeIdentifier, value__1412: SqlPart) -> UpdateQuery {
        let nb__1414: temper_core::ListBuilder<SetClause> = temper_core::ListedTrait::to_list_builder( & self.0.set_clauses);
        temper_core::listed::add( & nb__1414, SetClause::new(field__1411.clone(), value__1412.clone()), None);
        return UpdateQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1414), self.0.conditions.clone(), self.0.limit_val);
    }
    pub fn r#where(& self, condition__1416: SqlFragment) -> UpdateQuery {
        let nb__1418: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1418, WhereClause::new(AndCondition::new(condition__1416.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1418), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1420: SqlFragment) -> UpdateQuery {
        let nb__1422: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1422, WhereClause::new(OrCondition::new(condition__1420.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1422), self.0.limit_val);
    }
    pub fn limit(& self, n__1424: i32) -> temper_core::Result<UpdateQuery> {
        if Some(n__1424) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), self.0.conditions.clone(), Some(n__1424)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___12557: i32;
        let mut t___12571: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        if temper_core::ListedTrait::is_empty( & self.0.set_clauses) {
            return Err(temper_core::Error::new());
        }
        let b__1428: SqlBuilder = SqlBuilder::new();
        b__1428.append_safe("UPDATE ");
        b__1428.append_safe(self.0.table_name.sql_value());
        b__1428.append_safe(" SET ");
        b__1428.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, 0).field().sql_value());
        b__1428.append_safe(" = ");
        b__1428.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, 0).value());
        let mut i__1429: i32 = 1;
        'loop___14473: loop {
            t___12557 = temper_core::ListedTrait::len( & self.0.set_clauses);
            if ! (Some(i__1429) < Some(t___12557)) {
                break;
            }
            b__1428.append_safe(", ");
            b__1428.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, i__1429).field().sql_value());
            b__1428.append_safe(" = ");
            b__1428.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, i__1429).value());
            i__1429 = i__1429.wrapping_add(1);
        }
        b__1428.append_safe(" WHERE ");
        b__1428.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
        let mut i__1430: i32 = 1;
        'loop___14474: loop {
            t___12571 = temper_core::ListedTrait::len( & self.0.conditions);
            if ! (Some(i__1430) < Some(t___12571)) {
                break;
            }
            b__1428.append_safe(" ");
            b__1428.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1430).keyword());
            b__1428.append_safe(" ");
            b__1428.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1430).condition());
            i__1430 = i__1430.wrapping_add(1);
        }
        let lv__1431: Option<i32> = self.0.limit_val;
        if ! lv__1431.is_none() {
            let lv___2484: i32 = lv__1431.unwrap();
            b__1428.append_safe(" LIMIT ");
            b__1428.append_int32(lv___2484);
        }
        return Ok(b__1428.accumulated());
    }
    pub fn new(tableName__1433: SafeIdentifier, setClauses__1434: impl temper_core::ToList<SetClause>, conditions__1435: impl temper_core::ToList<WhereClause>, limitVal__1436: Option<i32>) -> UpdateQuery {
        let setClauses__1434 = setClauses__1434.to_list();
        let conditions__1435 = conditions__1435.to_list();
        let table_name;
        let set_clauses;
        let conditions;
        let limit_val;
        table_name = tableName__1433.clone();
        set_clauses = setClauses__1434.clone();
        conditions = conditions__1435.clone();
        limit_val = limitVal__1436;
        let selfish = UpdateQuery(std::sync::Arc::new(UpdateQueryStruct {
                    table_name, set_clauses, conditions, limit_val
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn set_clauses(& self) -> temper_core::List<SetClause> {
        return self.0.set_clauses.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.conditions.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
}
temper_core::impl_any_value_trait!(UpdateQuery, []);
struct DeleteQueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<WhereClause>, limit_val: Option<i32>
}
#[derive(Clone)]
pub struct DeleteQuery(std::sync::Arc<DeleteQueryStruct>);
#[derive(Clone)]
pub struct DeleteQueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<WhereClause>, pub limit_val: Option<i32>
}
impl DeleteQueryBuilder {
    pub fn build(self) -> DeleteQuery {
        DeleteQuery::new(self.table_name, self.conditions, self.limit_val)
    }
}
impl DeleteQuery {
    pub fn r#where(& self, condition__1441: SqlFragment) -> DeleteQuery {
        let nb__1443: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1443, WhereClause::new(AndCondition::new(condition__1441.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1443), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1445: SqlFragment) -> DeleteQuery {
        let nb__1447: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1447, WhereClause::new(OrCondition::new(condition__1445.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1447), self.0.limit_val);
    }
    pub fn limit(& self, n__1449: i32) -> temper_core::Result<DeleteQuery> {
        if Some(n__1449) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(DeleteQuery::new(self.0.table_name.clone(), self.0.conditions.clone(), Some(n__1449)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___12517: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        let b__1453: SqlBuilder = SqlBuilder::new();
        b__1453.append_safe("DELETE FROM ");
        b__1453.append_safe(self.0.table_name.sql_value());
        b__1453.append_safe(" WHERE ");
        b__1453.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
        let mut i__1454: i32 = 1;
        'loop___14480: loop {
            t___12517 = temper_core::ListedTrait::len( & self.0.conditions);
            if ! (Some(i__1454) < Some(t___12517)) {
                break;
            }
            b__1453.append_safe(" ");
            b__1453.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1454).keyword());
            b__1453.append_safe(" ");
            b__1453.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1454).condition());
            i__1454 = i__1454.wrapping_add(1);
        }
        let lv__1455: Option<i32> = self.0.limit_val;
        if ! lv__1455.is_none() {
            let lv___2485: i32 = lv__1455.unwrap();
            b__1453.append_safe(" LIMIT ");
            b__1453.append_int32(lv___2485);
        }
        return Ok(b__1453.accumulated());
    }
    pub fn new(tableName__1457: SafeIdentifier, conditions__1458: impl temper_core::ToList<WhereClause>, limitVal__1459: Option<i32>) -> DeleteQuery {
        let conditions__1458 = conditions__1458.to_list();
        let table_name;
        let conditions;
        let limit_val;
        table_name = tableName__1457.clone();
        conditions = conditions__1458.clone();
        limit_val = limitVal__1459;
        let selfish = DeleteQuery(std::sync::Arc::new(DeleteQueryStruct {
                    table_name, conditions, limit_val
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.conditions.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
}
temper_core::impl_any_value_trait!(DeleteQuery, []);
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
    pub fn new(_value__1694: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__1694 = _value__1694.to_arc_string();
        let value;
        value = _value__1694.clone();
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
    pub fn new(name__1712: SafeIdentifier, fieldType__1713: FieldType, nullable__1714: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__1712.clone();
        field_type = fieldType__1713.clone();
        nullable = nullable__1714;
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
    pub fn field(& self, name__1718: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__1718 = name__1718.to_arc_string();
        let return__584: FieldDef;
        'fn__1719: {
            let this__8420: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__8421: i32 = temper_core::ListedTrait::len( & this__8420);
            let mut i__8422: i32 = 0;
            'loop___14484: while Some(i__8422) < Some(n__8421) {
                let el__8423: FieldDef = temper_core::ListedTrait::get( & this__8420, i__8422);
                i__8422 = i__8422.wrapping_add(1);
                let f__1720: FieldDef = el__8423.clone();
                if Some(f__1720.name().sql_value().as_str()) == Some(name__1718.as_str()) {
                    return__584 = f__1720.clone();
                    break 'fn__1719;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__584.clone());
    }
    pub fn new(tableName__1722: SafeIdentifier, fields__1723: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__1723 = fields__1723.to_list();
        let table_name;
        let fields;
        table_name = tableName__1722.clone();
        fields = fields__1723.clone();
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
    pub fn append_safe(& self, sqlSource__1745: impl temper_core::ToArcString) {
        let sqlSource__1745 = sqlSource__1745.to_arc_string();
        let mut t___14251: SqlSource = SqlSource::new(sqlSource__1745.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___14251.clone()), None);
    }
    pub fn append_fragment(& self, fragment__1748: SqlFragment) {
        let mut t___14249: temper_core::List<SqlPart> = fragment__1748.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___14249.clone()), None);
    }
    pub fn append_part(& self, part__1751: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__1751.clone(), None);
    }
    pub fn append_part_list(& self, values__1754: impl temper_core::ToList<SqlPart>) {
        let values__1754 = values__1754.to_list();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__325: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__14245(& self, x__1756: SqlPart) {
                self.this__325.append_part(x__1756.clone());
            }
        }
        let closure_group = ClosureGroup___12 {
            this__325: self.clone()
        };
        let fn__14245 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1756: SqlPart | closure_group.fn__14245(x__1756))
        };
        self.append_list(temper_core::ToListed::to_listed(values__1754.clone()), fn__14245.clone());
    }
    pub fn append_boolean(& self, value__1758: bool) {
        let mut t___14242: SqlBoolean = SqlBoolean::new(value__1758);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___14242.clone()), None);
    }
    pub fn append_boolean_list(& self, values__1761: impl temper_core::ToListed<bool>) {
        let values__1761 = values__1761.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__327: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__14239(& self, x__1763: bool) {
                self.this__327.append_boolean(x__1763);
            }
        }
        let closure_group = ClosureGroup___13 {
            this__327: self.clone()
        };
        let fn__14239 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1763: bool | closure_group.fn__14239(x__1763))
        };
        self.append_list(values__1761.clone(), fn__14239.clone());
    }
    pub fn append_date(& self, value__1765: temper_std::temporal::Date) {
        let mut t___14236: SqlDate = SqlDate::new(value__1765.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___14236.clone()), None);
    }
    pub fn append_date_list(& self, values__1768: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__1768 = values__1768.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            this__329: SqlBuilder
        }
        impl ClosureGroup___14 {
            fn fn__14233(& self, x__1770: temper_std::temporal::Date) {
                self.this__329.append_date(x__1770.clone());
            }
        }
        let closure_group = ClosureGroup___14 {
            this__329: self.clone()
        };
        let fn__14233 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1770: temper_std::temporal::Date | closure_group.fn__14233(x__1770))
        };
        self.append_list(values__1768.clone(), fn__14233.clone());
    }
    pub fn append_float64(& self, value__1772: f64) {
        let mut t___14230: SqlFloat64 = SqlFloat64::new(value__1772);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___14230.clone()), None);
    }
    pub fn append_float64_list(& self, values__1775: impl temper_core::ToListed<f64>) {
        let values__1775 = values__1775.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___15 {
            this__331: SqlBuilder
        }
        impl ClosureGroup___15 {
            fn fn__14227(& self, x__1777: f64) {
                self.this__331.append_float64(x__1777);
            }
        }
        let closure_group = ClosureGroup___15 {
            this__331: self.clone()
        };
        let fn__14227 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1777: f64 | closure_group.fn__14227(x__1777))
        };
        self.append_list(values__1775.clone(), fn__14227.clone());
    }
    pub fn append_int32(& self, value__1779: i32) {
        let mut t___14224: SqlInt32 = SqlInt32::new(value__1779);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___14224.clone()), None);
    }
    pub fn append_int32_list(& self, values__1782: impl temper_core::ToListed<i32>) {
        let values__1782 = values__1782.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___16 {
            this__333: SqlBuilder
        }
        impl ClosureGroup___16 {
            fn fn__14221(& self, x__1784: i32) {
                self.this__333.append_int32(x__1784);
            }
        }
        let closure_group = ClosureGroup___16 {
            this__333: self.clone()
        };
        let fn__14221 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1784: i32 | closure_group.fn__14221(x__1784))
        };
        self.append_list(values__1782.clone(), fn__14221.clone());
    }
    pub fn append_int64(& self, value__1786: i64) {
        let mut t___14218: SqlInt64 = SqlInt64::new(value__1786);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___14218.clone()), None);
    }
    pub fn append_int64_list(& self, values__1789: impl temper_core::ToListed<i64>) {
        let values__1789 = values__1789.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___17 {
            this__335: SqlBuilder
        }
        impl ClosureGroup___17 {
            fn fn__14215(& self, x__1791: i64) {
                self.this__335.append_int64(x__1791);
            }
        }
        let closure_group = ClosureGroup___17 {
            this__335: self.clone()
        };
        let fn__14215 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1791: i64 | closure_group.fn__14215(x__1791))
        };
        self.append_list(values__1789.clone(), fn__14215.clone());
    }
    pub fn append_string(& self, value__1793: impl temper_core::ToArcString) {
        let value__1793 = value__1793.to_arc_string();
        let mut t___14212: SqlString = SqlString::new(value__1793.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___14212.clone()), None);
    }
    pub fn append_string_list(& self, values__1796: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__1796 = values__1796.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___18 {
            this__337: SqlBuilder
        }
        impl ClosureGroup___18 {
            fn fn__14209(& self, x__1798: impl temper_core::ToArcString) {
                let x__1798 = x__1798.to_arc_string();
                self.this__337.append_string(x__1798.clone());
            }
        }
        let closure_group = ClosureGroup___18 {
            this__337: self.clone()
        };
        let fn__14209 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1798: std::sync::Arc<String> | closure_group.fn__14209(x__1798))
        };
        self.append_list(values__1796.clone(), fn__14209.clone());
    }
    fn append_list<T>(& self, values__1800: impl temper_core::ToListed<T>, appendValue__1801: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__1800 = values__1800.to_listed();
        let mut t___14204: i32;
        let mut t___14206: T;
        let mut i__1803: i32 = 0;
        'loop___14485: loop {
            t___14204 = temper_core::ListedTrait::len( & ( * values__1800));
            if ! (Some(i__1803) < Some(t___14204)) {
                break;
            }
            if Some(i__1803) > Some(0) {
                self.append_safe(", ");
            }
            t___14206 = temper_core::ListedTrait::get( & ( * values__1800), i__1803);
            appendValue__1801(t___14206.clone());
            i__1803 = i__1803.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___14201: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___14201.clone();
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
        let mut t___14275: i32;
        let builder__1815: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__1816: i32 = 0;
        'loop___14486: loop {
            t___14275 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__1816) < Some(t___14275)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__1816).format_to(builder__1815.clone());
            i__1816 = i__1816.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__1815);
    }
    pub fn new(parts__1818: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__1818 = parts__1818.to_list();
        let parts;
        parts = parts__1818.clone();
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
    fn format_to(& self, builder__1820: std::sync::Arc<std::sync::RwLock<String>>);
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
    pub fn format_to(& self, builder__1824: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1824, self.0.source.clone());
    }
    pub fn new(source__1827: impl temper_core::ToArcString) -> SqlSource {
        let source__1827 = source__1827.to_arc_string();
        let source;
        source = source__1827.clone();
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
    fn format_to(& self, builder__1824: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1824)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__1830: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___8157: std::sync::Arc<String>;
        if self.0.value {
            t___8157 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___8157 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__1830, t___8157.clone());
    }
    pub fn new(value__1833: bool) -> SqlBoolean {
        let value;
        value = value__1833;
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
    fn format_to(& self, builder__1830: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1830)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__1836: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1836, "'");
        let mut t___14256: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___19 {
            builder__1836: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___19 {
            fn fn__14254(& self, c__1838: i32) {
                if Some(c__1838) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1836, "''");
                } else {
                    'ok___14322: {
                        'orelse___2406: {
                            match temper_core::string::builder::append_code_point( & self.builder__1836, c__1838) {
                                Ok(x) => x,
                                _ => break 'orelse___2406
                            };
                            break 'ok___14322;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___19 {
            builder__1836: builder__1836.clone()
        };
        let fn__14254 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1838: i32 | closure_group.fn__14254(c__1838))
        };
        temper_core::string::for_each( & t___14256, & ( * fn__14254.clone()));
        temper_core::string::builder::append( & builder__1836, "'");
    }
    pub fn new(value__1840: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__1840.clone();
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
    fn format_to(& self, builder__1836: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1836)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__1843: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___8146: bool;
        let mut t___8147: bool;
        let s__1845: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__1845.as_str()) == Some("NaN") {
            t___8147 = true;
        } else {
            if Some(s__1845.as_str()) == Some("Infinity") {
                t___8146 = true;
            } else {
                t___8146 = Some(s__1845.as_str()) == Some("-Infinity");
            }
            t___8147 = t___8146;
        }
        if t___8147 {
            temper_core::string::builder::append( & builder__1843, "NULL");
        } else {
            temper_core::string::builder::append( & builder__1843, s__1845.clone());
        }
    }
    pub fn new(value__1847: f64) -> SqlFloat64 {
        let value;
        value = value__1847;
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
    fn format_to(& self, builder__1843: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1843)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__1850: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___14265: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1850, t___14265.clone());
    }
    pub fn new(value__1853: i32) -> SqlInt32 {
        let value;
        value = value__1853;
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
    fn format_to(& self, builder__1850: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1850)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__1856: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___14263: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1856, t___14263.clone());
    }
    pub fn new(value__1859: i64) -> SqlInt64 {
        let value;
        value = value__1859;
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
    fn format_to(& self, builder__1856: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1856)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__1862: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1862, "'");
        #[derive(Clone)]
        struct ClosureGroup___20 {
            builder__1862: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___20 {
            fn fn__14268(& self, c__1864: i32) {
                if Some(c__1864) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1862, "''");
                } else {
                    'ok___14326: {
                        'orelse___2405: {
                            match temper_core::string::builder::append_code_point( & self.builder__1862, c__1864) {
                                Ok(x) => x,
                                _ => break 'orelse___2405
                            };
                            break 'ok___14326;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___20 {
            builder__1862: builder__1862.clone()
        };
        let fn__14268 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1864: i32 | closure_group.fn__14268(c__1864))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__14268.clone()));
        temper_core::string::builder::append( & builder__1862, "'");
    }
    pub fn new(value__1866: impl temper_core::ToArcString) -> SqlString {
        let value__1866 = value__1866.to_arc_string();
        let value;
        value = value__1866.clone();
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
    fn format_to(& self, builder__1862: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1862)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__921: TableDef, params__922: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___13867: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__921.clone(), params__922.clone(), t___13867.clone(), [], true));
}
fn isIdentStart__639(c__1695: i32) -> bool {
    let return__564: bool;
    let mut t___7693: bool;
    let mut t___7694: bool;
    if Some(c__1695) >= Some(97) {
        t___7693 = Some(c__1695) <= Some(122);
    } else {
        t___7693 = false;
    }
    if t___7693 {
        return__564 = true;
    } else {
        if Some(c__1695) >= Some(65) {
            t___7694 = Some(c__1695) <= Some(90);
        } else {
            t___7694 = false;
        }
        if t___7694 {
            return__564 = true;
        } else {
            return__564 = Some(c__1695) == Some(95);
        }
    }
    return return__564;
}
fn isIdentPart__640(c__1697: i32) -> bool {
    let return__565: bool;
    if isIdentStart__639(c__1697) {
        return__565 = true;
    } else {
        if Some(c__1697) >= Some(48) {
            return__565 = Some(c__1697) <= Some(57);
        } else {
            return__565 = false;
        }
    }
    return return__565;
}
pub fn safe_identifier(name__1699: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__1699 = name__1699.to_arc_string();
    let mut t___13865: usize;
    if name__1699.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__1701: usize = 0usize;
    if ! isIdentStart__639(temper_core::string::get( & name__1699, idx__1701)) {
        return Err(temper_core::Error::new());
    }
    let mut t___13862: usize = temper_core::string::next( & name__1699, idx__1701);
    idx__1701 = t___13862;
    'loop___14487: loop {
        if ! temper_core::string::has_index( & name__1699, idx__1701) {
            break;
        }
        if ! isIdentPart__640(temper_core::string::get( & name__1699, idx__1701)) {
            return Err(temper_core::Error::new());
        }
        t___13865 = temper_core::string::next( & name__1699, idx__1701);
        idx__1701 = t___13865;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__1699.clone())));
}
fn csid__636(name__924: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__924 = name__924.to_arc_string();
    let return__427: SafeIdentifier;
    let mut t___7681: SafeIdentifier;
    'ok___14328: {
        'orelse___2412: {
            t___7681 = match safe_identifier(name__924.clone()) {
                Ok(x) => x,
                _ => break 'orelse___2412
            };
            return__427 = t___7681.clone();
            break 'ok___14328;
        }
        return__427 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__427.clone();
}
fn userTable__637() -> TableDef {
    return TableDef::new(csid__636("users"), [FieldDef::new(csid__636("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__636("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__636("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__636("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__636("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__1120: TableDef, id__1121: i32) -> SqlFragment {
    let b__1123: SqlBuilder = SqlBuilder::new();
    b__1123.append_safe("DELETE FROM ");
    b__1123.append_safe(tableDef__1120.table_name().sql_value());
    b__1123.append_safe(" WHERE id = ");
    b__1123.append_int32(id__1121);
    return b__1123.accumulated();
}
pub fn from(tableName__1355: SafeIdentifier) -> Query {
    return Query::new(tableName__1355.clone(), [], [], [], None, None, [], [], [], false, [], None);
}
pub fn col(table__1357: SafeIdentifier, column__1358: SafeIdentifier) -> SqlFragment {
    let b__1360: SqlBuilder = SqlBuilder::new();
    b__1360.append_safe(table__1357.sql_value());
    b__1360.append_safe(".");
    b__1360.append_safe(column__1358.sql_value());
    return b__1360.accumulated();
}
pub fn count_all() -> SqlFragment {
    let b__1362: SqlBuilder = SqlBuilder::new();
    b__1362.append_safe("COUNT(*)");
    return b__1362.accumulated();
}
pub fn count_col(field__1363: SafeIdentifier) -> SqlFragment {
    let b__1365: SqlBuilder = SqlBuilder::new();
    b__1365.append_safe("COUNT(");
    b__1365.append_safe(field__1363.sql_value());
    b__1365.append_safe(")");
    return b__1365.accumulated();
}
pub fn sum_col(field__1366: SafeIdentifier) -> SqlFragment {
    let b__1368: SqlBuilder = SqlBuilder::new();
    b__1368.append_safe("SUM(");
    b__1368.append_safe(field__1366.sql_value());
    b__1368.append_safe(")");
    return b__1368.accumulated();
}
pub fn avg_col(field__1369: SafeIdentifier) -> SqlFragment {
    let b__1371: SqlBuilder = SqlBuilder::new();
    b__1371.append_safe("AVG(");
    b__1371.append_safe(field__1369.sql_value());
    b__1371.append_safe(")");
    return b__1371.accumulated();
}
pub fn min_col(field__1372: SafeIdentifier) -> SqlFragment {
    let b__1374: SqlBuilder = SqlBuilder::new();
    b__1374.append_safe("MIN(");
    b__1374.append_safe(field__1372.sql_value());
    b__1374.append_safe(")");
    return b__1374.accumulated();
}
pub fn max_col(field__1375: SafeIdentifier) -> SqlFragment {
    let b__1377: SqlBuilder = SqlBuilder::new();
    b__1377.append_safe("MAX(");
    b__1377.append_safe(field__1375.sql_value());
    b__1377.append_safe(")");
    return b__1377.accumulated();
}
pub fn union_sql(a__1378: Query, b__1379: Query) -> SqlFragment {
    let sb__1381: SqlBuilder = SqlBuilder::new();
    sb__1381.append_safe("(");
    sb__1381.append_fragment(a__1378.to_sql());
    sb__1381.append_safe(") UNION (");
    sb__1381.append_fragment(b__1379.to_sql());
    sb__1381.append_safe(")");
    return sb__1381.accumulated();
}
pub fn union_all_sql(a__1382: Query, b__1383: Query) -> SqlFragment {
    let sb__1385: SqlBuilder = SqlBuilder::new();
    sb__1385.append_safe("(");
    sb__1385.append_fragment(a__1382.to_sql());
    sb__1385.append_safe(") UNION ALL (");
    sb__1385.append_fragment(b__1383.to_sql());
    sb__1385.append_safe(")");
    return sb__1385.accumulated();
}
pub fn intersect_sql(a__1386: Query, b__1387: Query) -> SqlFragment {
    let sb__1389: SqlBuilder = SqlBuilder::new();
    sb__1389.append_safe("(");
    sb__1389.append_fragment(a__1386.to_sql());
    sb__1389.append_safe(") INTERSECT (");
    sb__1389.append_fragment(b__1387.to_sql());
    sb__1389.append_safe(")");
    return sb__1389.accumulated();
}
pub fn except_sql(a__1390: Query, b__1391: Query) -> SqlFragment {
    let sb__1393: SqlBuilder = SqlBuilder::new();
    sb__1393.append_safe("(");
    sb__1393.append_fragment(a__1390.to_sql());
    sb__1393.append_safe(") EXCEPT (");
    sb__1393.append_fragment(b__1391.to_sql());
    sb__1393.append_safe(")");
    return sb__1393.accumulated();
}
pub fn subquery(q__1394: Query, alias__1395: SafeIdentifier) -> SqlFragment {
    let b__1397: SqlBuilder = SqlBuilder::new();
    b__1397.append_safe("(");
    b__1397.append_fragment(q__1394.to_sql());
    b__1397.append_safe(") AS ");
    b__1397.append_safe(alias__1395.sql_value());
    return b__1397.accumulated();
}
pub fn exists_sql(q__1398: Query) -> SqlFragment {
    let b__1400: SqlBuilder = SqlBuilder::new();
    b__1400.append_safe("EXISTS (");
    b__1400.append_fragment(q__1398.to_sql());
    b__1400.append_safe(")");
    return b__1400.accumulated();
}
pub fn update(tableName__1460: SafeIdentifier) -> UpdateQuery {
    return UpdateQuery::new(tableName__1460.clone(), [], [], None);
}
pub fn delete_from(tableName__1462: SafeIdentifier) -> DeleteQuery {
    return DeleteQuery::new(tableName__1462.clone(), [], None);
}
fn sid__638(name__1464: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__1464 = name__1464.to_arc_string();
    let return__557: SafeIdentifier;
    let mut t___6485: SafeIdentifier;
    'ok___14340: {
        'orelse___2423: {
            t___6485 = match safe_identifier(name__1464.clone()) {
                Ok(x) => x,
                _ => break 'orelse___2423
            };
            return__557 = t___6485.clone();
            break 'ok___14340;
        }
        return__557 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__557.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__2009() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__928: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___13823: TableDef = userTable__637();
        let mut t___13824: SafeIdentifier = csid__636("name");
        let mut t___13825: SafeIdentifier = csid__636("email");
        let cs__929: Changeset = changeset(t___13823.clone(), params__928.clone()).cast(std::sync::Arc::new(vec![t___13824.clone(), t___13825.clone()]));
        let mut t___13828: bool = temper_core::MappedTrait::has( & cs__929.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__13818(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__13818 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13818())
        };
        test___31.assert(t___13828, fn__13818.clone());
        let mut t___13832: bool = temper_core::MappedTrait::has( & cs__929.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__13817(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__13817 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13817())
        };
        test___31.assert(t___13832, fn__13817.clone());
        let mut t___13838: bool = ! temper_core::MappedTrait::has( & cs__929.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__13816(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__13816 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13816())
        };
        test___31.assert(t___13838, fn__13816.clone());
        let mut t___13840: bool = cs__929.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__13815(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__13815 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13815())
        };
        test___31.assert(t___13840, fn__13815.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__2010() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__931: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___13801: TableDef = userTable__637();
        let mut t___13802: SafeIdentifier = csid__636("name");
        let cs__932: Changeset = changeset(t___13801.clone(), params__931.clone()).cast(std::sync::Arc::new(vec![t___13802.clone()])).cast(std::sync::Arc::new(vec![csid__636("email")]));
        let mut t___13809: bool = ! temper_core::MappedTrait::has( & cs__932.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__13797(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__13797 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13797())
        };
        test___32.assert(t___13809, fn__13797.clone());
        let mut t___13812: bool = temper_core::MappedTrait::has( & cs__932.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__13796(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__13796 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13796())
        };
        test___32.assert(t___13812, fn__13796.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__2011() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__934: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___13783: TableDef = userTable__637();
        let mut t___13784: SafeIdentifier = csid__636("name");
        let mut t___13785: SafeIdentifier = csid__636("email");
        let cs__935: Changeset = changeset(t___13783.clone(), params__934.clone()).cast(std::sync::Arc::new(vec![t___13784.clone(), t___13785.clone()]));
        let mut t___13790: bool = ! temper_core::MappedTrait::has( & cs__935.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__13779(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__13779 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13779())
        };
        test___33.assert(t___13790, fn__13779.clone());
        let mut t___13793: bool = temper_core::MappedTrait::has( & cs__935.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__13778(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__13778 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13778())
        };
        test___33.assert(t___13793, fn__13778.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__2012() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__937: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___13765: TableDef = userTable__637();
        let mut t___13766: SafeIdentifier = csid__636("name");
        let cs__938: Changeset = changeset(t___13765.clone(), params__937.clone()).cast(std::sync::Arc::new(vec![t___13766.clone()])).validate_required(std::sync::Arc::new(vec![csid__636("name")]));
        let mut t___13770: bool = cs__938.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__13762(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__13762 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13762())
        };
        test___34.assert(t___13770, fn__13762.clone());
        let mut t___13776: bool = Some(temper_core::ListedTrait::len( & cs__938.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__13761(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__13761 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13761())
        };
        test___34.assert(t___13776, fn__13761.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__2013() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let params__940: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___13741: TableDef = userTable__637();
        let mut t___13742: SafeIdentifier = csid__636("name");
        let cs__941: Changeset = changeset(t___13741.clone(), params__940.clone()).cast(std::sync::Arc::new(vec![t___13742.clone()])).validate_required(std::sync::Arc::new(vec![csid__636("name")]));
        let mut t___13748: bool = ! cs__941.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__13739(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__13739 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13739())
        };
        test___35.assert(t___13748, fn__13739.clone());
        let mut t___13753: bool = Some(temper_core::ListedTrait::len( & cs__941.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__13738(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__13738 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13738())
        };
        test___35.assert(t___13753, fn__13738.clone());
        let mut t___13759: bool = Some(temper_core::ListedTrait::get( & cs__941.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__13737(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__13737 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13737())
        };
        test___35.assert(t___13759, fn__13737.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__2014() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let params__943: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___13729: TableDef = userTable__637();
        let mut t___13730: SafeIdentifier = csid__636("name");
        let cs__944: Changeset = changeset(t___13729.clone(), params__943.clone()).cast(std::sync::Arc::new(vec![t___13730.clone()])).validate_length(csid__636("name"), 2, 50);
        let mut t___13734: bool = cs__944.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__13726(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__13726 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13726())
        };
        test___36.assert(t___13734, fn__13726.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__2015() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        let params__946: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___13717: TableDef = userTable__637();
        let mut t___13718: SafeIdentifier = csid__636("name");
        let cs__947: Changeset = changeset(t___13717.clone(), params__946.clone()).cast(std::sync::Arc::new(vec![t___13718.clone()])).validate_length(csid__636("name"), 2, 50);
        let mut t___13724: bool = ! cs__947.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__13714(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__13714 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13714())
        };
        test___37.assert(t___13724, fn__13714.clone());
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__2016() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        let params__949: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___13705: TableDef = userTable__637();
        let mut t___13706: SafeIdentifier = csid__636("name");
        let cs__950: Changeset = changeset(t___13705.clone(), params__949.clone()).cast(std::sync::Arc::new(vec![t___13706.clone()])).validate_length(csid__636("name"), 2, 10);
        let mut t___13712: bool = ! cs__950.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__13702(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__13702 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13702())
        };
        test___38.assert(t___13712, fn__13702.clone());
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__2017() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        let params__952: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___13694: TableDef = userTable__637();
        let mut t___13695: SafeIdentifier = csid__636("age");
        let cs__953: Changeset = changeset(t___13694.clone(), params__952.clone()).cast(std::sync::Arc::new(vec![t___13695.clone()])).validate_int(csid__636("age"));
        let mut t___13699: bool = cs__953.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__13691(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__13691 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13691())
        };
        test___39.assert(t___13699, fn__13691.clone());
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__2018() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let params__955: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___13682: TableDef = userTable__637();
        let mut t___13683: SafeIdentifier = csid__636("age");
        let cs__956: Changeset = changeset(t___13682.clone(), params__955.clone()).cast(std::sync::Arc::new(vec![t___13683.clone()])).validate_int(csid__636("age"));
        let mut t___13689: bool = ! cs__956.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___38 {}
        impl ClosureGroup___38 {
            fn fn__13679(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___38 {};
        let fn__13679 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13679())
        };
        test___40.assert(t___13689, fn__13679.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__2019() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let params__958: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___13671: TableDef = userTable__637();
        let mut t___13672: SafeIdentifier = csid__636("score");
        let cs__959: Changeset = changeset(t___13671.clone(), params__958.clone()).cast(std::sync::Arc::new(vec![t___13672.clone()])).validate_float(csid__636("score"));
        let mut t___13676: bool = cs__959.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__13668(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__13668 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13668())
        };
        test___41.assert(t___13676, fn__13668.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__2020() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let params__961: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___13660: TableDef = userTable__637();
        let mut t___13661: SafeIdentifier = csid__636("age");
        let cs__962: Changeset = changeset(t___13660.clone(), params__961.clone()).cast(std::sync::Arc::new(vec![t___13661.clone()])).validate_int64(csid__636("age"));
        let mut t___13665: bool = cs__962.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___40 {}
        impl ClosureGroup___40 {
            fn fn__13657(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___40 {};
        let fn__13657 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13657())
        };
        test___42.assert(t___13665, fn__13657.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__2021() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let params__964: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___13648: TableDef = userTable__637();
        let mut t___13649: SafeIdentifier = csid__636("age");
        let cs__965: Changeset = changeset(t___13648.clone(), params__964.clone()).cast(std::sync::Arc::new(vec![t___13649.clone()])).validate_int64(csid__636("age"));
        let mut t___13655: bool = ! cs__965.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___41 {}
        impl ClosureGroup___41 {
            fn fn__13645(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___41 {};
        let fn__13645 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13645())
        };
        test___43.assert(t___13655, fn__13645.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__2022() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            test___44: temper_std::testing::Test
        }
        impl ClosureGroup___42 {
            fn fn__13642(& self, v__967: impl temper_core::ToArcString) {
                let v__967 = v__967.to_arc_string();
                let params__968: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__967.clone())]);
                let mut t___13634: TableDef = userTable__637();
                let mut t___13635: SafeIdentifier = csid__636("active");
                let cs__969: Changeset = changeset(t___13634.clone(), params__968.clone()).cast(std::sync::Arc::new(vec![t___13635.clone()])).validate_bool(csid__636("active"));
                let mut t___13639: bool = cs__969.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___43 {
                    v__967: std::sync::Arc<String>
                }
                impl ClosureGroup___43 {
                    fn fn__13631(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__967.clone()));
                    }
                }
                let closure_group = ClosureGroup___43 {
                    v__967: v__967.clone()
                };
                let fn__13631 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__13631())
                };
                self.test___44.assert(t___13639, fn__13631.clone());
            }
        }
        let closure_group = ClosureGroup___42 {
            test___44: test___44.clone()
        };
        let fn__13642 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__967: std::sync::Arc<String> | closure_group.fn__13642(v__967))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__13642.clone()));
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__2023() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            test___45: temper_std::testing::Test
        }
        impl ClosureGroup___44 {
            fn fn__13628(& self, v__971: impl temper_core::ToArcString) {
                let v__971 = v__971.to_arc_string();
                let params__972: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__971.clone())]);
                let mut t___13620: TableDef = userTable__637();
                let mut t___13621: SafeIdentifier = csid__636("active");
                let cs__973: Changeset = changeset(t___13620.clone(), params__972.clone()).cast(std::sync::Arc::new(vec![t___13621.clone()])).validate_bool(csid__636("active"));
                let mut t___13625: bool = cs__973.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___45 {
                    v__971: std::sync::Arc<String>
                }
                impl ClosureGroup___45 {
                    fn fn__13617(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__971.clone()));
                    }
                }
                let closure_group = ClosureGroup___45 {
                    v__971: v__971.clone()
                };
                let fn__13617 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__13617())
                };
                self.test___45.assert(t___13625, fn__13617.clone());
            }
        }
        let closure_group = ClosureGroup___44 {
            test___45: test___45.clone()
        };
        let fn__13628 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__971: std::sync::Arc<String> | closure_group.fn__13628(v__971))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__13628.clone()));
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__2024() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            test___46: temper_std::testing::Test
        }
        impl ClosureGroup___46 {
            fn fn__13614(& self, v__975: impl temper_core::ToArcString) {
                let v__975 = v__975.to_arc_string();
                let params__976: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__975.clone())]);
                let mut t___13605: TableDef = userTable__637();
                let mut t___13606: SafeIdentifier = csid__636("active");
                let cs__977: Changeset = changeset(t___13605.clone(), params__976.clone()).cast(std::sync::Arc::new(vec![t___13606.clone()])).validate_bool(csid__636("active"));
                let mut t___13612: bool = ! cs__977.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___47 {
                    v__975: std::sync::Arc<String>
                }
                impl ClosureGroup___47 {
                    fn fn__13602(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__975.clone()));
                    }
                }
                let closure_group = ClosureGroup___47 {
                    v__975: v__975.clone()
                };
                let fn__13602 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__13602())
                };
                self.test___46.assert(t___13612, fn__13602.clone());
            }
        }
        let closure_group = ClosureGroup___46 {
            test___46: test___46.clone()
        };
        let fn__13614 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__975: std::sync::Arc<String> | closure_group.fn__13614(v__975))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__13614.clone()));
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__2025() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let mut t___7482: SqlFragment;
        let params__979: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___13590: TableDef = userTable__637();
        let mut t___13591: SafeIdentifier = csid__636("name");
        let mut t___13592: SafeIdentifier = csid__636("email");
        let cs__980: Changeset = changeset(t___13590.clone(), params__979.clone()).cast(std::sync::Arc::new(vec![t___13591.clone(), t___13592.clone()])).validate_required(std::sync::Arc::new(vec![csid__636("name"), csid__636("email")]));
        let sqlFrag__981: SqlFragment;
        'ok___14330: {
            'orelse___2413: {
                t___7482 = match cs__980.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2413
                };
                sqlFrag__981 = t___7482.clone();
                break 'ok___14330;
            }
            sqlFrag__981 = panic!();
        }
        let s__982: std::sync::Arc<String> = sqlFrag__981.to_string();
        let mut t___13599: bool = temper_core::string::index_of( & s__982, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___48 {
            s__982: std::sync::Arc<String>
        }
        impl ClosureGroup___48 {
            fn fn__13586(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__982.clone()));
            }
        }
        let closure_group = ClosureGroup___48 {
            s__982: s__982.clone()
        };
        let fn__13586 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13586())
        };
        test___47.assert(t___13599, fn__13586.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__2026() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___7461: SqlFragment;
        let params__984: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___13570: TableDef = userTable__637();
        let mut t___13571: SafeIdentifier = csid__636("name");
        let mut t___13572: SafeIdentifier = csid__636("email");
        let cs__985: Changeset = changeset(t___13570.clone(), params__984.clone()).cast(std::sync::Arc::new(vec![t___13571.clone(), t___13572.clone()])).validate_required(std::sync::Arc::new(vec![csid__636("name"), csid__636("email")]));
        let sqlFrag__986: SqlFragment;
        'ok___14331: {
            'orelse___2414: {
                t___7461 = match cs__985.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2414
                };
                sqlFrag__986 = t___7461.clone();
                break 'ok___14331;
            }
            sqlFrag__986 = panic!();
        }
        let s__987: std::sync::Arc<String> = sqlFrag__986.to_string();
        let mut t___13579: bool = temper_core::string::index_of( & s__987, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___49 {
            s__987: std::sync::Arc<String>
        }
        impl ClosureGroup___49 {
            fn fn__13566(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__987.clone()));
            }
        }
        let closure_group = ClosureGroup___49 {
            s__987: s__987.clone()
        };
        let fn__13566 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13566())
        };
        test___48.assert(t___13579, fn__13566.clone());
        let mut t___13583: bool = temper_core::string::index_of( & s__987, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___50 {
            s__987: std::sync::Arc<String>
        }
        impl ClosureGroup___50 {
            fn fn__13565(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__987.clone()));
            }
        }
        let closure_group = ClosureGroup___50 {
            s__987: s__987.clone()
        };
        let fn__13565 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13565())
        };
        test___48.assert(t___13583, fn__13565.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__2027() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___7444: SqlFragment;
        let params__989: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___13552: TableDef = userTable__637();
        let mut t___13553: SafeIdentifier = csid__636("name");
        let mut t___13554: SafeIdentifier = csid__636("email");
        let mut t___13555: SafeIdentifier = csid__636("age");
        let cs__990: Changeset = changeset(t___13552.clone(), params__989.clone()).cast(std::sync::Arc::new(vec![t___13553.clone(), t___13554.clone(), t___13555.clone()])).validate_required(std::sync::Arc::new(vec![csid__636("name"), csid__636("email")]));
        let sqlFrag__991: SqlFragment;
        'ok___14332: {
            'orelse___2415: {
                t___7444 = match cs__990.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2415
                };
                sqlFrag__991 = t___7444.clone();
                break 'ok___14332;
            }
            sqlFrag__991 = panic!();
        }
        let s__992: std::sync::Arc<String> = sqlFrag__991.to_string();
        let mut t___13562: bool = temper_core::string::index_of( & s__992, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___51 {
            s__992: std::sync::Arc<String>
        }
        impl ClosureGroup___51 {
            fn fn__13547(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__992.clone()));
            }
        }
        let closure_group = ClosureGroup___51 {
            s__992: s__992.clone()
        };
        let fn__13547 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13547())
        };
        test___49.assert(t___13562, fn__13547.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__2028() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let params__994: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___13540: TableDef = userTable__637();
        let mut t___13541: SafeIdentifier = csid__636("name");
        let cs__995: Changeset = changeset(t___13540.clone(), params__994.clone()).cast(std::sync::Arc::new(vec![t___13541.clone()])).validate_required(std::sync::Arc::new(vec![csid__636("name")]));
        let didBubble__996: bool;
        'ok___14333: {
            'orelse___2416: {
                match cs__995.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2416
                };
                didBubble__996 = false;
                break 'ok___14333;
            }
            didBubble__996 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__13538(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__13538 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13538())
        };
        test___50.assert(didBubble__996, fn__13538.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__2029() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let strictTable__998: TableDef = TableDef::new(csid__636("posts"), [FieldDef::new(csid__636("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__636("body"), FieldType::new(StringField::new()), true)]);
        let params__999: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___13531: SafeIdentifier = csid__636("body");
        let cs__1000: Changeset = changeset(strictTable__998.clone(), params__999.clone()).cast(std::sync::Arc::new(vec![t___13531.clone()]));
        let mut t___13533: bool = cs__1000.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___53 {}
        impl ClosureGroup___53 {
            fn fn__13520(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___53 {};
        let fn__13520 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13520())
        };
        test___51.assert(t___13533, fn__13520.clone());
        let didBubble__1001: bool;
        'ok___14334: {
            'orelse___2417: {
                match cs__1000.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2417
                };
                didBubble__1001 = false;
                break 'ok___14334;
            }
            didBubble__1001 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__13519(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__13519 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13519())
        };
        test___51.assert(didBubble__1001, fn__13519.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__2030() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___7404: SqlFragment;
        let params__1003: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___13510: TableDef = userTable__637();
        let mut t___13511: SafeIdentifier = csid__636("name");
        let cs__1004: Changeset = changeset(t___13510.clone(), params__1003.clone()).cast(std::sync::Arc::new(vec![t___13511.clone()])).validate_required(std::sync::Arc::new(vec![csid__636("name")]));
        let sqlFrag__1005: SqlFragment;
        'ok___14335: {
            'orelse___2418: {
                t___7404 = match cs__1004.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___2418
                };
                sqlFrag__1005 = t___7404.clone();
                break 'ok___14335;
            }
            sqlFrag__1005 = panic!();
        }
        let s__1006: std::sync::Arc<String> = sqlFrag__1005.to_string();
        let mut t___13517: bool = Some(s__1006.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___55 {
            s__1006: std::sync::Arc<String>
        }
        impl ClosureGroup___55 {
            fn fn__13507(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1006.clone()));
            }
        }
        let closure_group = ClosureGroup___55 {
            s__1006: s__1006.clone()
        };
        let fn__13507 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13507())
        };
        test___52.assert(t___13517, fn__13507.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__2031() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let params__1008: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___13500: TableDef = userTable__637();
        let mut t___13501: SafeIdentifier = csid__636("name");
        let cs__1009: Changeset = changeset(t___13500.clone(), params__1008.clone()).cast(std::sync::Arc::new(vec![t___13501.clone()])).validate_required(std::sync::Arc::new(vec![csid__636("name")]));
        let didBubble__1010: bool;
        'ok___14336: {
            'orelse___2419: {
                match cs__1009.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2419
                };
                didBubble__1010 = false;
                break 'ok___14336;
            }
            didBubble__1010 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__13498(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__13498 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13498())
        };
        test___53.assert(didBubble__1010, fn__13498.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn putChangeAddsANewField__2032() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let params__1012: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___13484: TableDef = userTable__637();
        let mut t___13485: SafeIdentifier = csid__636("name");
        let cs__1013: Changeset = changeset(t___13484.clone(), params__1012.clone()).cast(std::sync::Arc::new(vec![t___13485.clone()])).put_change(csid__636("email"), std::sync::Arc::new("alice@example.com".to_string()));
        let mut t___13490: bool = temper_core::MappedTrait::has( & cs__1013.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__13481(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__13481 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13481())
        };
        test___54.assert(t___13490, fn__13481.clone());
        let mut t___13496: bool = Some(temper_core::MappedTrait::get_or( & cs__1013.changes(), std::sync::Arc::new("email".to_string()), std::sync::Arc::new("".to_string())).as_str()) == Some("alice@example.com");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__13480(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email value".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__13480 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13480())
        };
        test___54.assert(t___13496, fn__13480.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn putChangeOverwritesExistingField__2033() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let params__1015: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___13470: TableDef = userTable__637();
        let mut t___13471: SafeIdentifier = csid__636("name");
        let cs__1016: Changeset = changeset(t___13470.clone(), params__1015.clone()).cast(std::sync::Arc::new(vec![t___13471.clone()])).put_change(csid__636("name"), std::sync::Arc::new("Bob".to_string()));
        let mut t___13478: bool = Some(temper_core::MappedTrait::get_or( & cs__1016.changes(), std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())).as_str()) == Some("Bob");
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__13467(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be overwritten".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__13467 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13467())
        };
        test___55.assert(t___13478, fn__13467.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn putChangeValueAppearsInToInsertSql__2034() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let mut t___7359: SqlFragment;
        let mut t___7360: SqlFragment;
        let params__1018: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___13456: TableDef = userTable__637();
        let mut t___13457: SafeIdentifier = csid__636("name");
        let mut t___13458: SafeIdentifier = csid__636("email");
        let cs__1019: Changeset = changeset(t___13456.clone(), params__1018.clone()).cast(std::sync::Arc::new(vec![t___13457.clone(), t___13458.clone()])).put_change(csid__636("name"), std::sync::Arc::new("Bob".to_string()));
        'ok___14337: {
            'orelse___2420: {
                t___7359 = match cs__1019.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2420
                };
                t___7360 = t___7359.clone();
                break 'ok___14337;
            }
            t___7360 = panic!();
        }
        let s__1020: std::sync::Arc<String> = t___7360.to_string();
        let mut t___13464: bool = temper_core::string::index_of( & s__1020, "'Bob'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___60 {
            s__1020: std::sync::Arc<String>
        }
        impl ClosureGroup___60 {
            fn fn__13452(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should use putChange value: {}", self.s__1020.clone()));
            }
        }
        let closure_group = ClosureGroup___60 {
            s__1020: s__1020.clone()
        };
        let fn__13452 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13452())
        };
        test___56.assert(t___13464, fn__13452.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn getChangeReturnsValueForExistingField__2035() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___7347: std::sync::Arc<String>;
        let params__1022: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___13445: TableDef = userTable__637();
        let mut t___13446: SafeIdentifier = csid__636("name");
        let cs__1023: Changeset = changeset(t___13445.clone(), params__1022.clone()).cast(std::sync::Arc::new(vec![t___13446.clone()]));
        let val__1024: std::sync::Arc<String>;
        'ok___14338: {
            'orelse___2421: {
                t___7347 = match cs__1023.get_change(csid__636("name")) {
                    Ok(x) => x,
                    _ => break 'orelse___2421
                };
                val__1024 = t___7347.clone();
                break 'ok___14338;
            }
            val__1024 = panic!();
        }
        let mut t___13450: bool = Some(val__1024.as_str()) == Some("Alice");
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__13442(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should return Alice".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__13442 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13442())
        };
        test___57.assert(t___13450, fn__13442.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn getChangeBubblesOnMissingField__2036() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let params__1026: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___13436: TableDef = userTable__637();
        let mut t___13437: SafeIdentifier = csid__636("name");
        let cs__1027: Changeset = changeset(t___13436.clone(), params__1026.clone()).cast(std::sync::Arc::new(vec![t___13437.clone()]));
        let didBubble__1028: bool;
        'ok___14339: {
            'orelse___2422: {
                match cs__1027.get_change(csid__636("email")) {
                    Ok(x) => x,
                    _ => break 'orelse___2422
                };
                didBubble__1028 = false;
                break 'ok___14339;
            }
            didBubble__1028 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__13433(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should bubble for missing field".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__13433 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13433())
        };
        test___58.assert(didBubble__1028, fn__13433.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn deleteChangeRemovesField__2037() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let params__1030: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___13418: TableDef = userTable__637();
        let mut t___13419: SafeIdentifier = csid__636("name");
        let mut t___13420: SafeIdentifier = csid__636("email");
        let cs__1031: Changeset = changeset(t___13418.clone(), params__1030.clone()).cast(std::sync::Arc::new(vec![t___13419.clone(), t___13420.clone()])).delete_change(csid__636("email"));
        let mut t___13427: bool = ! temper_core::MappedTrait::has( & cs__1031.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___63 {}
        impl ClosureGroup___63 {
            fn fn__13414(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be removed".to_string());
            }
        }
        let closure_group = ClosureGroup___63 {};
        let fn__13414 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13414())
        };
        test___59.assert(t___13427, fn__13414.clone());
        let mut t___13430: bool = temper_core::MappedTrait::has( & cs__1031.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___64 {}
        impl ClosureGroup___64 {
            fn fn__13413(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should remain".to_string());
            }
        }
        let closure_group = ClosureGroup___64 {};
        let fn__13413 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13413())
        };
        test___59.assert(t___13430, fn__13413.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn deleteChangeOnNonexistentFieldIsNoOp__2038() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let params__1033: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___13401: TableDef = userTable__637();
        let mut t___13402: SafeIdentifier = csid__636("name");
        let cs__1034: Changeset = changeset(t___13401.clone(), params__1033.clone()).cast(std::sync::Arc::new(vec![t___13402.clone()])).delete_change(csid__636("email"));
        let mut t___13407: bool = temper_core::MappedTrait::has( & cs__1034.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__13398(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should still be present".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__13398 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13398())
        };
        test___60.assert(t___13407, fn__13398.clone());
        let mut t___13410: bool = cs__1034.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___66 {}
        impl ClosureGroup___66 {
            fn fn__13397(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___66 {};
        let fn__13397 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13397())
        };
        test___60.assert(t___13410, fn__13397.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionPassesWhenValueInList__2039() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let params__1036: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("admin".to_string()))]);
        let mut t___13389: TableDef = userTable__637();
        let mut t___13390: SafeIdentifier = csid__636("name");
        let cs__1037: Changeset = changeset(t___13389.clone(), params__1036.clone()).cast(std::sync::Arc::new(vec![t___13390.clone()])).validate_inclusion(csid__636("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string()), std::sync::Arc::new("guest".to_string())]));
        let mut t___13394: bool = cs__1037.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___67 {}
        impl ClosureGroup___67 {
            fn fn__13386(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___67 {};
        let fn__13386 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13386())
        };
        test___61.assert(t___13394, fn__13386.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionFailsWhenValueNotInList__2040() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let params__1039: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("hacker".to_string()))]);
        let mut t___13371: TableDef = userTable__637();
        let mut t___13372: SafeIdentifier = csid__636("name");
        let cs__1040: Changeset = changeset(t___13371.clone(), params__1039.clone()).cast(std::sync::Arc::new(vec![t___13372.clone()])).validate_inclusion(csid__636("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string()), std::sync::Arc::new("guest".to_string())]));
        let mut t___13378: bool = ! cs__1040.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__13368(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__13368 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13368())
        };
        test___62.assert(t___13378, fn__13368.clone());
        let mut t___13384: bool = Some(temper_core::ListedTrait::get( & cs__1040.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___69 {}
        impl ClosureGroup___69 {
            fn fn__13367(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on name".to_string());
            }
        }
        let closure_group = ClosureGroup___69 {};
        let fn__13367 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13367())
        };
        test___62.assert(t___13384, fn__13367.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionSkipsWhenFieldNotInChanges__2041() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let params__1042: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___13359: TableDef = userTable__637();
        let mut t___13360: SafeIdentifier = csid__636("name");
        let cs__1043: Changeset = changeset(t___13359.clone(), params__1042.clone()).cast(std::sync::Arc::new(vec![t___13360.clone()])).validate_inclusion(csid__636("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string())]));
        let mut t___13364: bool = cs__1043.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___70 {}
        impl ClosureGroup___70 {
            fn fn__13357(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___70 {};
        let fn__13357 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13357())
        };
        test___63.assert(t___13364, fn__13357.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionPassesWhenValueNotInList__2042() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let params__1045: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___13349: TableDef = userTable__637();
        let mut t___13350: SafeIdentifier = csid__636("name");
        let cs__1046: Changeset = changeset(t___13349.clone(), params__1045.clone()).cast(std::sync::Arc::new(vec![t___13350.clone()])).validate_exclusion(csid__636("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("superuser".to_string())]));
        let mut t___13354: bool = cs__1046.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__13346(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__13346 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13346())
        };
        test___64.assert(t___13354, fn__13346.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionFailsWhenValueInList__2043() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let params__1048: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("admin".to_string()))]);
        let mut t___13331: TableDef = userTable__637();
        let mut t___13332: SafeIdentifier = csid__636("name");
        let cs__1049: Changeset = changeset(t___13331.clone(), params__1048.clone()).cast(std::sync::Arc::new(vec![t___13332.clone()])).validate_exclusion(csid__636("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("superuser".to_string())]));
        let mut t___13338: bool = ! cs__1049.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__13328(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__13328 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13328())
        };
        test___65.assert(t___13338, fn__13328.clone());
        let mut t___13344: bool = Some(temper_core::ListedTrait::get( & cs__1049.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__13327(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on name".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__13327 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13327())
        };
        test___65.assert(t___13344, fn__13327.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionSkipsWhenFieldNotInChanges__2044() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let params__1051: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___13319: TableDef = userTable__637();
        let mut t___13320: SafeIdentifier = csid__636("name");
        let cs__1052: Changeset = changeset(t___13319.clone(), params__1051.clone()).cast(std::sync::Arc::new(vec![t___13320.clone()])).validate_exclusion(csid__636("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string())]));
        let mut t___13324: bool = cs__1052.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__13317(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__13317 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13317())
        };
        test___66.assert(t___13324, fn__13317.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanPasses__2045() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let params__1054: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___13308: TableDef = userTable__637();
        let mut t___13309: SafeIdentifier = csid__636("age");
        let cs__1055: Changeset = changeset(t___13308.clone(), params__1054.clone()).cast(std::sync::Arc::new(vec![t___13309.clone()])).validate_number(csid__636("age"), NumberValidationOpts::new(Some(18.0f64), None, None, None, None));
        let mut t___13314: bool = cs__1055.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__13305(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("25 > 18 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__13305 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13305())
        };
        test___67.assert(t___13314, fn__13305.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanFails__2046() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let params__1057: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("15".to_string()))]);
        let mut t___13295: TableDef = userTable__637();
        let mut t___13296: SafeIdentifier = csid__636("age");
        let cs__1058: Changeset = changeset(t___13295.clone(), params__1057.clone()).cast(std::sync::Arc::new(vec![t___13296.clone()])).validate_number(csid__636("age"), NumberValidationOpts::new(Some(18.0f64), None, None, None, None));
        let mut t___13303: bool = ! cs__1058.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__13292(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("15 > 18 should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__13292 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13292())
        };
        test___68.assert(t___13303, fn__13292.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanPasses__2047() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let params__1060: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("8.5".to_string()))]);
        let mut t___13283: TableDef = userTable__637();
        let mut t___13284: SafeIdentifier = csid__636("score");
        let cs__1061: Changeset = changeset(t___13283.clone(), params__1060.clone()).cast(std::sync::Arc::new(vec![t___13284.clone()])).validate_number(csid__636("score"), NumberValidationOpts::new(None, Some(10.0f64), None, None, None));
        let mut t___13289: bool = cs__1061.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__13280(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("8.5 < 10 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__13280 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13280())
        };
        test___69.assert(t___13289, fn__13280.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanFails__2048() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let params__1063: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("12.0".to_string()))]);
        let mut t___13270: TableDef = userTable__637();
        let mut t___13271: SafeIdentifier = csid__636("score");
        let cs__1064: Changeset = changeset(t___13270.clone(), params__1063.clone()).cast(std::sync::Arc::new(vec![t___13271.clone()])).validate_number(csid__636("score"), NumberValidationOpts::new(None, Some(10.0f64), None, None, None));
        let mut t___13278: bool = ! cs__1064.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__13267(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("12 < 10 should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__13267 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13267())
        };
        test___70.assert(t___13278, fn__13267.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanOrEqualBoundary__2049() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let params__1066: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("18".to_string()))]);
        let mut t___13258: TableDef = userTable__637();
        let mut t___13259: SafeIdentifier = csid__636("age");
        let cs__1067: Changeset = changeset(t___13258.clone(), params__1066.clone()).cast(std::sync::Arc::new(vec![t___13259.clone()])).validate_number(csid__636("age"), NumberValidationOpts::new(None, None, Some(18.0f64), None, None));
        let mut t___13264: bool = cs__1067.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__13255(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("18 >= 18 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__13255 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13255())
        };
        test___71.assert(t___13264, fn__13255.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberCombinedOptions__2050() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let params__1069: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("5.0".to_string()))]);
        let mut t___13246: TableDef = userTable__637();
        let mut t___13247: SafeIdentifier = csid__636("score");
        let cs__1070: Changeset = changeset(t___13246.clone(), params__1069.clone()).cast(std::sync::Arc::new(vec![t___13247.clone()])).validate_number(csid__636("score"), NumberValidationOpts::new(Some(0.0f64), Some(10.0f64), None, None, None));
        let mut t___13252: bool = cs__1070.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___80 {}
        impl ClosureGroup___80 {
            fn fn__13243(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("5 > 0 and < 10 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___80 {};
        let fn__13243 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13243())
        };
        test___72.assert(t___13252, fn__13243.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberNonNumericValue__2051() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let params__1072: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("abc".to_string()))]);
        let mut t___13227: TableDef = userTable__637();
        let mut t___13228: SafeIdentifier = csid__636("age");
        let cs__1073: Changeset = changeset(t___13227.clone(), params__1072.clone()).cast(std::sync::Arc::new(vec![t___13228.clone()])).validate_number(csid__636("age"), NumberValidationOpts::new(Some(0.0f64), None, None, None, None));
        let mut t___13235: bool = ! cs__1073.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___81 {}
        impl ClosureGroup___81 {
            fn fn__13224(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("non-numeric should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___81 {};
        let fn__13224 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13224())
        };
        test___73.assert(t___13235, fn__13224.clone());
        let mut t___13241: bool = Some(temper_core::ListedTrait::get( & cs__1073.errors(), 0).message().as_str()) == Some("must be a number");
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__13223(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct error message".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__13223 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13223())
        };
        test___73.assert(t___13241, fn__13223.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberSkipsWhenFieldNotInChanges__2052() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let params__1075: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___13214: TableDef = userTable__637();
        let mut t___13215: SafeIdentifier = csid__636("age");
        let cs__1076: Changeset = changeset(t___13214.clone(), params__1075.clone()).cast(std::sync::Arc::new(vec![t___13215.clone()])).validate_number(csid__636("age"), NumberValidationOpts::new(Some(0.0f64), None, None, None, None));
        let mut t___13220: bool = cs__1076.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__13212(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__13212 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13212())
        };
        test___74.assert(t___13220, fn__13212.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn validateAcceptancePassesForTrueValues__2053() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___84 {
            test___75: temper_std::testing::Test
        }
        impl ClosureGroup___84 {
            fn fn__13209(& self, v__1078: impl temper_core::ToArcString) {
                let v__1078 = v__1078.to_arc_string();
                let params__1079: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__1078.clone())]);
                let mut t___13201: TableDef = userTable__637();
                let mut t___13202: SafeIdentifier = csid__636("active");
                let cs__1080: Changeset = changeset(t___13201.clone(), params__1079.clone()).cast(std::sync::Arc::new(vec![t___13202.clone()])).validate_acceptance(csid__636("active"));
                let mut t___13206: bool = cs__1080.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___85 {
                    v__1078: std::sync::Arc<String>
                }
                impl ClosureGroup___85 {
                    fn fn__13198(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__1078.clone()));
                    }
                }
                let closure_group = ClosureGroup___85 {
                    v__1078: v__1078.clone()
                };
                let fn__13198 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__13198())
                };
                self.test___75.assert(t___13206, fn__13198.clone());
            }
        }
        let closure_group = ClosureGroup___84 {
            test___75: test___75.clone()
        };
        let fn__13209 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__1078: std::sync::Arc<String> | closure_group.fn__13209(v__1078))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__13209.clone()));
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn validateAcceptanceFailsForNonTrueValues__2054() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        let params__1082: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), std::sync::Arc::new("false".to_string()))]);
        let mut t___13183: TableDef = userTable__637();
        let mut t___13184: SafeIdentifier = csid__636("active");
        let cs__1083: Changeset = changeset(t___13183.clone(), params__1082.clone()).cast(std::sync::Arc::new(vec![t___13184.clone()])).validate_acceptance(csid__636("active"));
        let mut t___13190: bool = ! cs__1083.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn fn__13180(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("false should not be accepted".to_string());
            }
        }
        let closure_group = ClosureGroup___86 {};
        let fn__13180 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13180())
        };
        test___76.assert(t___13190, fn__13180.clone());
        let mut t___13196: bool = Some(temper_core::ListedTrait::get( & cs__1083.errors(), 0).message().as_str()) == Some("must be accepted");
        #[derive(Clone)]
        struct ClosureGroup___87 {}
        impl ClosureGroup___87 {
            fn fn__13179(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct message".to_string());
            }
        }
        let closure_group = ClosureGroup___87 {};
        let fn__13179 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13179())
        };
        test___76.assert(t___13196, fn__13179.clone());
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationPassesWhenFieldsMatch__2055() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let tbl__1085: TableDef = TableDef::new(csid__636("users"), [FieldDef::new(csid__636("password"), FieldType::new(StringField::new()), false), FieldDef::new(csid__636("password_confirmation"), FieldType::new(StringField::new()), true)]);
        let params__1086: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string())), (std::sync::Arc::new("password_confirmation".to_string()), std::sync::Arc::new("secret123".to_string()))]);
        let mut t___13170: SafeIdentifier = csid__636("password");
        let mut t___13171: SafeIdentifier = csid__636("password_confirmation");
        let cs__1087: Changeset = changeset(tbl__1085.clone(), params__1086.clone()).cast(std::sync::Arc::new(vec![t___13170.clone(), t___13171.clone()])).validate_confirmation(csid__636("password"), csid__636("password_confirmation"));
        let mut t___13176: bool = cs__1087.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___88 {}
        impl ClosureGroup___88 {
            fn fn__13158(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("matching fields should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___88 {};
        let fn__13158 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13158())
        };
        test___77.assert(t___13176, fn__13158.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationFailsWhenFieldsDiffer__2056() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let tbl__1089: TableDef = TableDef::new(csid__636("users"), [FieldDef::new(csid__636("password"), FieldType::new(StringField::new()), false), FieldDef::new(csid__636("password_confirmation"), FieldType::new(StringField::new()), true)]);
        let params__1090: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string())), (std::sync::Arc::new("password_confirmation".to_string()), std::sync::Arc::new("wrong456".to_string()))]);
        let mut t___13142: SafeIdentifier = csid__636("password");
        let mut t___13143: SafeIdentifier = csid__636("password_confirmation");
        let cs__1091: Changeset = changeset(tbl__1089.clone(), params__1090.clone()).cast(std::sync::Arc::new(vec![t___13142.clone(), t___13143.clone()])).validate_confirmation(csid__636("password"), csid__636("password_confirmation"));
        let mut t___13150: bool = ! cs__1091.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__13130(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mismatched fields should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__13130 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13130())
        };
        test___78.assert(t___13150, fn__13130.clone());
        let mut t___13156: bool = Some(temper_core::ListedTrait::get( & cs__1091.errors(), 0).field().as_str()) == Some("password_confirmation");
        #[derive(Clone)]
        struct ClosureGroup___90 {}
        impl ClosureGroup___90 {
            fn fn__13129(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on confirmation field".to_string());
            }
        }
        let closure_group = ClosureGroup___90 {};
        let fn__13129 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13129())
        };
        test___78.assert(t___13156, fn__13129.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationFailsWhenConfirmationMissing__2057() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let tbl__1093: TableDef = TableDef::new(csid__636("users"), [FieldDef::new(csid__636("password"), FieldType::new(StringField::new()), false), FieldDef::new(csid__636("password_confirmation"), FieldType::new(StringField::new()), true)]);
        let params__1094: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string()))]);
        let mut t___13120: SafeIdentifier = csid__636("password");
        let cs__1095: Changeset = changeset(tbl__1093.clone(), params__1094.clone()).cast(std::sync::Arc::new(vec![t___13120.clone()])).validate_confirmation(csid__636("password"), csid__636("password_confirmation"));
        let mut t___13127: bool = ! cs__1095.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___91 {}
        impl ClosureGroup___91 {
            fn fn__13109(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("missing confirmation should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___91 {};
        let fn__13109 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13109())
        };
        test___79.assert(t___13127, fn__13109.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsPassesWhenSubstringFound__2058() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let params__1097: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___13101: TableDef = userTable__637();
        let mut t___13102: SafeIdentifier = csid__636("email");
        let cs__1098: Changeset = changeset(t___13101.clone(), params__1097.clone()).cast(std::sync::Arc::new(vec![t___13102.clone()])).validate_contains(csid__636("email"), std::sync::Arc::new("@".to_string()));
        let mut t___13106: bool = cs__1098.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___92 {}
        impl ClosureGroup___92 {
            fn fn__13098(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass when @ present".to_string());
            }
        }
        let closure_group = ClosureGroup___92 {};
        let fn__13098 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13098())
        };
        test___80.assert(t___13106, fn__13098.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsFailsWhenSubstringNotFound__2059() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let params__1100: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice-example.com".to_string()))]);
        let mut t___13089: TableDef = userTable__637();
        let mut t___13090: SafeIdentifier = csid__636("email");
        let cs__1101: Changeset = changeset(t___13089.clone(), params__1100.clone()).cast(std::sync::Arc::new(vec![t___13090.clone()])).validate_contains(csid__636("email"), std::sync::Arc::new("@".to_string()));
        let mut t___13096: bool = ! cs__1101.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___93 {}
        impl ClosureGroup___93 {
            fn fn__13086(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail when @ absent".to_string());
            }
        }
        let closure_group = ClosureGroup___93 {};
        let fn__13086 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13086())
        };
        test___81.assert(t___13096, fn__13086.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsSkipsWhenFieldNotInChanges__2060() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let params__1103: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___13078: TableDef = userTable__637();
        let mut t___13079: SafeIdentifier = csid__636("email");
        let cs__1104: Changeset = changeset(t___13078.clone(), params__1103.clone()).cast(std::sync::Arc::new(vec![t___13079.clone()])).validate_contains(csid__636("email"), std::sync::Arc::new("@".to_string()));
        let mut t___13083: bool = cs__1104.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___94 {}
        impl ClosureGroup___94 {
            fn fn__13076(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___94 {};
        let fn__13076 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13076())
        };
        test___82.assert(t___13083, fn__13076.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn validateStartsWithPasses__2061() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let params__1106: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Dr. Smith".to_string()))]);
        let mut t___13068: TableDef = userTable__637();
        let mut t___13069: SafeIdentifier = csid__636("name");
        let cs__1107: Changeset = changeset(t___13068.clone(), params__1106.clone()).cast(std::sync::Arc::new(vec![t___13069.clone()])).validate_starts_with(csid__636("name"), std::sync::Arc::new("Dr.".to_string()));
        let mut t___13073: bool = cs__1107.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___95 {}
        impl ClosureGroup___95 {
            fn fn__13065(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass for Dr. prefix".to_string());
            }
        }
        let closure_group = ClosureGroup___95 {};
        let fn__13065 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13065())
        };
        test___83.assert(t___13073, fn__13065.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn validateStartsWithFails__2062() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let params__1109: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Mr. Smith".to_string()))]);
        let mut t___13056: TableDef = userTable__637();
        let mut t___13057: SafeIdentifier = csid__636("name");
        let cs__1110: Changeset = changeset(t___13056.clone(), params__1109.clone()).cast(std::sync::Arc::new(vec![t___13057.clone()])).validate_starts_with(csid__636("name"), std::sync::Arc::new("Dr.".to_string()));
        let mut t___13063: bool = ! cs__1110.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___96 {}
        impl ClosureGroup___96 {
            fn fn__13053(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail for Mr. prefix".to_string());
            }
        }
        let closure_group = ClosureGroup___96 {};
        let fn__13053 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13053())
        };
        test___84.assert(t___13063, fn__13053.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithPasses__2063() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let params__1112: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___13045: TableDef = userTable__637();
        let mut t___13046: SafeIdentifier = csid__636("email");
        let cs__1113: Changeset = changeset(t___13045.clone(), params__1112.clone()).cast(std::sync::Arc::new(vec![t___13046.clone()])).validate_ends_with(csid__636("email"), std::sync::Arc::new(".com".to_string()));
        let mut t___13050: bool = cs__1113.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___97 {}
        impl ClosureGroup___97 {
            fn fn__13042(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass for .com suffix".to_string());
            }
        }
        let closure_group = ClosureGroup___97 {};
        let fn__13042 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13042())
        };
        test___85.assert(t___13050, fn__13042.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithFails__2064() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let params__1115: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.org".to_string()))]);
        let mut t___13033: TableDef = userTable__637();
        let mut t___13034: SafeIdentifier = csid__636("email");
        let cs__1116: Changeset = changeset(t___13033.clone(), params__1115.clone()).cast(std::sync::Arc::new(vec![t___13034.clone()])).validate_ends_with(csid__636("email"), std::sync::Arc::new(".com".to_string()));
        let mut t___13040: bool = ! cs__1116.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___98 {}
        impl ClosureGroup___98 {
            fn fn__13030(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail for .org when expecting .com".to_string());
            }
        }
        let closure_group = ClosureGroup___98 {};
        let fn__13030 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13030())
        };
        test___86.assert(t___13040, fn__13030.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithHandlesRepeatedSuffixCorrectly__2065() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let params__1118: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("abcabc".to_string()))]);
        let mut t___13022: TableDef = userTable__637();
        let mut t___13023: SafeIdentifier = csid__636("name");
        let cs__1119: Changeset = changeset(t___13022.clone(), params__1118.clone()).cast(std::sync::Arc::new(vec![t___13023.clone()])).validate_ends_with(csid__636("name"), std::sync::Arc::new("abc".to_string()));
        let mut t___13027: bool = cs__1119.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___99 {}
        impl ClosureGroup___99 {
            fn fn__13019(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("abcabc should end with abc".to_string());
            }
        }
        let closure_group = ClosureGroup___99 {};
        let fn__13019 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13019())
        };
        test___87.assert(t___13027, fn__13019.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__2147() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let q__1467: Query = from(sid__638("users"));
        let mut t___12504: bool = Some(q__1467.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___100 {}
        impl ClosureGroup___100 {
            fn fn__12499(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___100 {};
        let fn__12499 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12499())
        };
        test___88.assert(t___12504, fn__12499.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__2148() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let mut t___12490: SafeIdentifier = sid__638("users");
        let mut t___12491: SafeIdentifier = sid__638("id");
        let mut t___12492: SafeIdentifier = sid__638("name");
        let q__1469: Query = from(t___12490.clone()).select([t___12491.clone(), t___12492.clone()]);
        let mut t___12497: bool = Some(q__1469.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___101 {}
        impl ClosureGroup___101 {
            fn fn__12489(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___101 {};
        let fn__12489 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12489())
        };
        test___89.assert(t___12497, fn__12489.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__2149() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___90 = temper_std::testing::Test::new();
        let mut t___12478: SafeIdentifier = sid__638("users");
        let mut t___12479: SqlBuilder = SqlBuilder::new();
        t___12479.append_safe("age > ");
        t___12479.append_int32(18);
        let mut t___12482: SqlFragment = t___12479.accumulated();
        let q__1471: Query = from(t___12478.clone()).r#where(t___12482.clone());
        let mut t___12487: bool = Some(q__1471.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___102 {}
        impl ClosureGroup___102 {
            fn fn__12477(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___102 {};
        let fn__12477 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12477())
        };
        test___90.assert(t___12487, fn__12477.clone());
        test___90.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__2151() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___91 = temper_std::testing::Test::new();
        let mut t___12466: SafeIdentifier = sid__638("users");
        let mut t___12467: SqlBuilder = SqlBuilder::new();
        t___12467.append_safe("active = ");
        t___12467.append_boolean(true);
        let mut t___12470: SqlFragment = t___12467.accumulated();
        let q__1473: Query = from(t___12466.clone()).r#where(t___12470.clone());
        let mut t___12475: bool = Some(q__1473.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___103 {}
        impl ClosureGroup___103 {
            fn fn__12465(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___103 {};
        let fn__12465 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12465())
        };
        test___91.assert(t___12475, fn__12465.clone());
        test___91.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__2153() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___92 = temper_std::testing::Test::new();
        let mut t___12449: SafeIdentifier = sid__638("users");
        let mut t___12450: SqlBuilder = SqlBuilder::new();
        t___12450.append_safe("age > ");
        t___12450.append_int32(18);
        let mut t___12453: SqlFragment = t___12450.accumulated();
        let mut t___12454: Query = from(t___12449.clone()).r#where(t___12453.clone());
        let mut t___12455: SqlBuilder = SqlBuilder::new();
        t___12455.append_safe("active = ");
        t___12455.append_boolean(true);
        let q__1475: Query = t___12454.r#where(t___12455.accumulated());
        let mut t___12463: bool = Some(q__1475.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___104 {}
        impl ClosureGroup___104 {
            fn fn__12448(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___104 {};
        let fn__12448 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12448())
        };
        test___92.assert(t___12463, fn__12448.clone());
        test___92.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__2156() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___93 = temper_std::testing::Test::new();
        let mut t___12440: SafeIdentifier = sid__638("users");
        let mut t___12441: SafeIdentifier = sid__638("name");
        let q__1477: Query = from(t___12440.clone()).order_by(t___12441.clone(), true);
        let mut t___12446: bool = Some(q__1477.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___105 {}
        impl ClosureGroup___105 {
            fn fn__12439(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___105 {};
        let fn__12439 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12439())
        };
        test___93.assert(t___12446, fn__12439.clone());
        test___93.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__2157() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___94 = temper_std::testing::Test::new();
        let mut t___12431: SafeIdentifier = sid__638("users");
        let mut t___12432: SafeIdentifier = sid__638("created_at");
        let q__1479: Query = from(t___12431.clone()).order_by(t___12432.clone(), false);
        let mut t___12437: bool = Some(q__1479.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___106 {}
        impl ClosureGroup___106 {
            fn fn__12430(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___106 {};
        let fn__12430 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12430())
        };
        test___94.assert(t___12437, fn__12430.clone());
        test___94.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__2158() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___95 = temper_std::testing::Test::new();
        let mut t___6419: Query;
        let mut t___6420: Query;
        let q__1481: Query;
        'ok___14341: {
            'orelse___2424: {
                t___6419 = match from(sid__638("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2424
                };
                t___6420 = match t___6419.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2424
                };
                q__1481 = t___6420.clone();
                break 'ok___14341;
            }
            q__1481 = panic!();
        }
        let mut t___12428: bool = Some(q__1481.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___107 {}
        impl ClosureGroup___107 {
            fn fn__12423(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___107 {};
        let fn__12423 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12423())
        };
        test___95.assert(t___12428, fn__12423.clone());
        test___95.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__2159() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___96 = temper_std::testing::Test::new();
        let didBubble__1483: bool;
        'ok___14342: {
            'orelse___2425: {
                match from(sid__638("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2425
                };
                didBubble__1483 = false;
                break 'ok___14342;
            }
            didBubble__1483 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___108 {}
        impl ClosureGroup___108 {
            fn fn__12419(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___108 {};
        let fn__12419 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12419())
        };
        test___96.assert(didBubble__1483, fn__12419.clone());
        test___96.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__2160() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___97 = temper_std::testing::Test::new();
        let didBubble__1485: bool;
        'ok___14343: {
            'orelse___2426: {
                match from(sid__638("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2426
                };
                didBubble__1485 = false;
                break 'ok___14343;
            }
            didBubble__1485 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___109 {}
        impl ClosureGroup___109 {
            fn fn__12415(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___109 {};
        let fn__12415 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12415())
        };
        test___97.assert(didBubble__1485, fn__12415.clone());
        test___97.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__2161() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___98 = temper_std::testing::Test::new();
        let mut t___12393: SafeIdentifier;
        let mut t___12394: SafeIdentifier;
        let mut t___12395: SafeIdentifier;
        let mut t___12396: SafeIdentifier;
        let mut t___12397: Query;
        let mut t___12398: SqlBuilder;
        let mut t___12402: Query;
        let mut t___12403: SqlBuilder;
        let mut t___6405: Query;
        let mut t___6406: Query;
        let minAge__1487: i32 = 21;
        let q__1488: Query;
        'ok___14344: {
            'orelse___2427: {
                t___12393 = sid__638("users");
                t___12394 = sid__638("id");
                t___12395 = sid__638("name");
                t___12396 = sid__638("email");
                t___12397 = from(t___12393.clone()).select([t___12394.clone(), t___12395.clone(), t___12396.clone()]);
                t___12398 = SqlBuilder::new();
                t___12398.append_safe("age >= ");
                t___12398.append_int32(21);
                t___12402 = t___12397.r#where(t___12398.accumulated());
                t___12403 = SqlBuilder::new();
                t___12403.append_safe("active = ");
                t___12403.append_boolean(true);
                t___6405 = match t___12402.r#where(t___12403.accumulated()).order_by(sid__638("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___2427
                };
                t___6406 = match t___6405.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___2427
                };
                q__1488 = t___6406.clone();
                break 'ok___14344;
            }
            q__1488 = panic!();
        }
        let mut t___12413: bool = Some(q__1488.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___110 {}
        impl ClosureGroup___110 {
            fn fn__12392(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___110 {};
        let fn__12392 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12392())
        };
        test___98.assert(t___12413, fn__12392.clone());
        test___98.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__2164() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___99 = temper_std::testing::Test::new();
        let mut t___6382: SqlFragment;
        let mut t___6383: SqlFragment;
        let q__1490: Query = from(sid__638("users"));
        'ok___14345: {
            'orelse___2428: {
                t___6382 = match q__1490.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2428
                };
                t___6383 = t___6382.clone();
                break 'ok___14345;
            }
            t___6383 = panic!();
        }
        let s__1491: std::sync::Arc<String> = t___6383.to_string();
        let mut t___12390: bool = Some(s__1491.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___111 {
            s__1491: std::sync::Arc<String>
        }
        impl ClosureGroup___111 {
            fn fn__12386(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__1491.clone()));
            }
        }
        let closure_group = ClosureGroup___111 {
            s__1491: s__1491.clone()
        };
        let fn__12386 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12386())
        };
        test___99.assert(t___12390, fn__12386.clone());
        test___99.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__2165() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___100 = temper_std::testing::Test::new();
        let mut t___6374: Query;
        let mut t___6377: SqlFragment;
        let mut t___6378: SqlFragment;
        let q__1493: Query;
        'ok___14346: {
            'orelse___2429: {
                t___6374 = match from(sid__638("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___2429
                };
                q__1493 = t___6374.clone();
                break 'ok___14346;
            }
            q__1493 = panic!();
        }
        'ok___14347: {
            'orelse___2430: {
                t___6377 = match q__1493.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2430
                };
                t___6378 = t___6377.clone();
                break 'ok___14347;
            }
            t___6378 = panic!();
        }
        let s__1494: std::sync::Arc<String> = t___6378.to_string();
        let mut t___12384: bool = Some(s__1494.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___112 {
            s__1494: std::sync::Arc<String>
        }
        impl ClosureGroup___112 {
            fn fn__12380(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__1494.clone()));
            }
        }
        let closure_group = ClosureGroup___112 {
            s__1494: s__1494.clone()
        };
        let fn__12380 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12380())
        };
        test___100.assert(t___12384, fn__12380.clone());
        test___100.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__2166() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___101 = temper_std::testing::Test::new();
        let didBubble__1496: bool;
        'ok___14348: {
            'orelse___2431: {
                match from(sid__638("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2431
                };
                didBubble__1496 = false;
                break 'ok___14348;
            }
            didBubble__1496 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___113 {}
        impl ClosureGroup___113 {
            fn fn__12376(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___113 {};
        let fn__12376 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12376())
        };
        test___101.assert(didBubble__1496, fn__12376.clone());
        test___101.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__2167() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___102 = temper_std::testing::Test::new();
        let evil__1498: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___12360: SafeIdentifier = sid__638("users");
        let mut t___12361: SqlBuilder = SqlBuilder::new();
        t___12361.append_safe("name = ");
        t___12361.append_string("'; DROP TABLE users; --");
        let mut t___12364: SqlFragment = t___12361.accumulated();
        let q__1499: Query = from(t___12360.clone()).r#where(t___12364.clone());
        let s__1500: std::sync::Arc<String> = q__1499.to_sql().to_string();
        let mut t___12369: bool = temper_core::string::index_of( & s__1500, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___114 {
            s__1500: std::sync::Arc<String>
        }
        impl ClosureGroup___114 {
            fn fn__12359(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__1500.clone()));
            }
        }
        let closure_group = ClosureGroup___114 {
            s__1500: s__1500.clone()
        };
        let fn__12359 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12359())
        };
        test___102.assert(t___12369, fn__12359.clone());
        let mut t___12373: bool = temper_core::string::index_of( & s__1500, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___115 {
            s__1500: std::sync::Arc<String>
        }
        impl ClosureGroup___115 {
            fn fn__12358(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__1500.clone()));
            }
        }
        let closure_group = ClosureGroup___115 {
            s__1500: s__1500.clone()
        };
        let fn__12358 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12358())
        };
        test___102.assert(t___12373, fn__12358.clone());
        test___102.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__2169() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___103 = temper_std::testing::Test::new();
        let attack__1502: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__1503: bool;
        'ok___14349: {
            'orelse___2432: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___2432
                };
                didBubble__1503 = false;
                break 'ok___14349;
            }
            didBubble__1503 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___116 {}
        impl ClosureGroup___116 {
            fn fn__12355(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___116 {};
        let fn__12355 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12355())
        };
        test___103.assert(didBubble__1503, fn__12355.clone());
        test___103.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__2170() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___104 = temper_std::testing::Test::new();
        let mut t___12344: SafeIdentifier = sid__638("users");
        let mut t___12345: SafeIdentifier = sid__638("orders");
        let mut t___12346: SqlBuilder = SqlBuilder::new();
        t___12346.append_safe("users.id = orders.user_id");
        let mut t___12348: SqlFragment = t___12346.accumulated();
        let q__1505: Query = from(t___12344.clone()).inner_join(t___12345.clone(), t___12348.clone());
        let mut t___12353: bool = Some(q__1505.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___117 {}
        impl ClosureGroup___117 {
            fn fn__12343(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___117 {};
        let fn__12343 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12343())
        };
        test___104.assert(t___12353, fn__12343.clone());
        test___104.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__2172() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___105 = temper_std::testing::Test::new();
        let mut t___12332: SafeIdentifier = sid__638("users");
        let mut t___12333: SafeIdentifier = sid__638("profiles");
        let mut t___12334: SqlBuilder = SqlBuilder::new();
        t___12334.append_safe("users.id = profiles.user_id");
        let mut t___12336: SqlFragment = t___12334.accumulated();
        let q__1507: Query = from(t___12332.clone()).left_join(t___12333.clone(), t___12336.clone());
        let mut t___12341: bool = Some(q__1507.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___118 {}
        impl ClosureGroup___118 {
            fn fn__12331(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___118 {};
        let fn__12331 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12331())
        };
        test___105.assert(t___12341, fn__12331.clone());
        test___105.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__2174() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___106 = temper_std::testing::Test::new();
        let mut t___12320: SafeIdentifier = sid__638("orders");
        let mut t___12321: SafeIdentifier = sid__638("users");
        let mut t___12322: SqlBuilder = SqlBuilder::new();
        t___12322.append_safe("orders.user_id = users.id");
        let mut t___12324: SqlFragment = t___12322.accumulated();
        let q__1509: Query = from(t___12320.clone()).right_join(t___12321.clone(), t___12324.clone());
        let mut t___12329: bool = Some(q__1509.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___119 {}
        impl ClosureGroup___119 {
            fn fn__12319(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___119 {};
        let fn__12319 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12319())
        };
        test___106.assert(t___12329, fn__12319.clone());
        test___106.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__2176() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___107 = temper_std::testing::Test::new();
        let mut t___12308: SafeIdentifier = sid__638("users");
        let mut t___12309: SafeIdentifier = sid__638("orders");
        let mut t___12310: SqlBuilder = SqlBuilder::new();
        t___12310.append_safe("users.id = orders.user_id");
        let mut t___12312: SqlFragment = t___12310.accumulated();
        let q__1511: Query = from(t___12308.clone()).full_join(t___12309.clone(), t___12312.clone());
        let mut t___12317: bool = Some(q__1511.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___120 {}
        impl ClosureGroup___120 {
            fn fn__12307(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___120 {};
        let fn__12307 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12307())
        };
        test___107.assert(t___12317, fn__12307.clone());
        test___107.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__2178() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___108 = temper_std::testing::Test::new();
        let mut t___12291: SafeIdentifier = sid__638("users");
        let mut t___12292: SafeIdentifier = sid__638("orders");
        let mut t___12293: SqlBuilder = SqlBuilder::new();
        t___12293.append_safe("users.id = orders.user_id");
        let mut t___12295: SqlFragment = t___12293.accumulated();
        let mut t___12296: Query = from(t___12291.clone()).inner_join(t___12292.clone(), t___12295.clone());
        let mut t___12297: SafeIdentifier = sid__638("profiles");
        let mut t___12298: SqlBuilder = SqlBuilder::new();
        t___12298.append_safe("users.id = profiles.user_id");
        let q__1513: Query = t___12296.left_join(t___12297.clone(), t___12298.accumulated());
        let mut t___12305: bool = Some(q__1513.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___121 {}
        impl ClosureGroup___121 {
            fn fn__12290(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___121 {};
        let fn__12290 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12290())
        };
        test___108.assert(t___12305, fn__12290.clone());
        test___108.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__2181() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___109 = temper_std::testing::Test::new();
        let mut t___12272: SafeIdentifier;
        let mut t___12273: SafeIdentifier;
        let mut t___12274: SqlBuilder;
        let mut t___12276: SqlFragment;
        let mut t___12277: Query;
        let mut t___12278: SqlBuilder;
        let mut t___6289: Query;
        let q__1515: Query;
        'ok___14350: {
            'orelse___2433: {
                t___12272 = sid__638("users");
                t___12273 = sid__638("orders");
                t___12274 = SqlBuilder::new();
                t___12274.append_safe("users.id = orders.user_id");
                t___12276 = t___12274.accumulated();
                t___12277 = from(t___12272.clone()).inner_join(t___12273.clone(), t___12276.clone());
                t___12278 = SqlBuilder::new();
                t___12278.append_safe("orders.total > ");
                t___12278.append_int32(100);
                t___6289 = match t___12277.r#where(t___12278.accumulated()).order_by(sid__638("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2433
                };
                q__1515 = t___6289.clone();
                break 'ok___14350;
            }
            q__1515 = panic!();
        }
        let mut t___12288: bool = Some(q__1515.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___122 {}
        impl ClosureGroup___122 {
            fn fn__12271(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___122 {};
        let fn__12271 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12271())
        };
        test___109.assert(t___12288, fn__12271.clone());
        test___109.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__2184() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___110 = temper_std::testing::Test::new();
        let c__1517: SqlFragment = col(sid__638("users"), sid__638("id"));
        let mut t___12269: bool = Some(c__1517.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___123 {}
        impl ClosureGroup___123 {
            fn fn__12263(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___123 {};
        let fn__12263 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12263())
        };
        test___110.assert(t___12269, fn__12263.clone());
        test___110.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__2185() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___111 = temper_std::testing::Test::new();
        let onCond__1519: SqlFragment = col(sid__638("users"), sid__638("id"));
        let b__1520: SqlBuilder = SqlBuilder::new();
        b__1520.append_fragment(onCond__1519.clone());
        b__1520.append_safe(" = ");
        b__1520.append_fragment(col(sid__638("orders"), sid__638("user_id")));
        let mut t___12254: SafeIdentifier = sid__638("users");
        let mut t___12255: SafeIdentifier = sid__638("orders");
        let mut t___12256: SqlFragment = b__1520.accumulated();
        let q__1521: Query = from(t___12254.clone()).inner_join(t___12255.clone(), t___12256.clone());
        let mut t___12261: bool = Some(q__1521.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___124 {}
        impl ClosureGroup___124 {
            fn fn__12243(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___124 {};
        let fn__12243 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12243())
        };
        test___111.assert(t___12261, fn__12243.clone());
        test___111.soft_fail_to_hard()
    }
    #[test]
    fn orWhereBasic__2186() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___112 = temper_std::testing::Test::new();
        let mut t___12232: SafeIdentifier = sid__638("users");
        let mut t___12233: SqlBuilder = SqlBuilder::new();
        t___12233.append_safe("status = ");
        t___12233.append_string("active");
        let mut t___12236: SqlFragment = t___12233.accumulated();
        let q__1523: Query = from(t___12232.clone()).or_where(t___12236.clone());
        let mut t___12241: bool = Some(q__1523.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE status = 'active'");
        #[derive(Clone)]
        struct ClosureGroup___125 {}
        impl ClosureGroup___125 {
            fn fn__12231(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orWhere basic".to_string());
            }
        }
        let closure_group = ClosureGroup___125 {};
        let fn__12231 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12231())
        };
        test___112.assert(t___12241, fn__12231.clone());
        test___112.soft_fail_to_hard()
    }
    #[test]
    fn whereThenOrWhere__2188() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___113 = temper_std::testing::Test::new();
        let mut t___12215: SafeIdentifier = sid__638("users");
        let mut t___12216: SqlBuilder = SqlBuilder::new();
        t___12216.append_safe("age > ");
        t___12216.append_int32(18);
        let mut t___12219: SqlFragment = t___12216.accumulated();
        let mut t___12220: Query = from(t___12215.clone()).r#where(t___12219.clone());
        let mut t___12221: SqlBuilder = SqlBuilder::new();
        t___12221.append_safe("vip = ");
        t___12221.append_boolean(true);
        let q__1525: Query = t___12220.or_where(t___12221.accumulated());
        let mut t___12229: bool = Some(q__1525.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___126 {}
        impl ClosureGroup___126 {
            fn fn__12214(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where then orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___126 {};
        let fn__12214 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12214())
        };
        test___113.assert(t___12229, fn__12214.clone());
        test___113.soft_fail_to_hard()
    }
    #[test]
    fn multipleOrWhere__2191() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___114 = temper_std::testing::Test::new();
        let mut t___12193: SafeIdentifier = sid__638("users");
        let mut t___12194: SqlBuilder = SqlBuilder::new();
        t___12194.append_safe("active = ");
        t___12194.append_boolean(true);
        let mut t___12197: SqlFragment = t___12194.accumulated();
        let mut t___12198: Query = from(t___12193.clone()).r#where(t___12197.clone());
        let mut t___12199: SqlBuilder = SqlBuilder::new();
        t___12199.append_safe("role = ");
        t___12199.append_string("admin");
        let mut t___12203: Query = t___12198.or_where(t___12199.accumulated());
        let mut t___12204: SqlBuilder = SqlBuilder::new();
        t___12204.append_safe("role = ");
        t___12204.append_string("moderator");
        let q__1527: Query = t___12203.or_where(t___12204.accumulated());
        let mut t___12212: bool = Some(q__1527.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'");
        #[derive(Clone)]
        struct ClosureGroup___127 {}
        impl ClosureGroup___127 {
            fn fn__12192(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("multiple orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___127 {};
        let fn__12192 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12192())
        };
        test___114.assert(t___12212, fn__12192.clone());
        test___114.soft_fail_to_hard()
    }
    #[test]
    fn mixedWhereAndOrWhere__2195() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___115 = temper_std::testing::Test::new();
        let mut t___12171: SafeIdentifier = sid__638("users");
        let mut t___12172: SqlBuilder = SqlBuilder::new();
        t___12172.append_safe("age > ");
        t___12172.append_int32(18);
        let mut t___12175: SqlFragment = t___12172.accumulated();
        let mut t___12176: Query = from(t___12171.clone()).r#where(t___12175.clone());
        let mut t___12177: SqlBuilder = SqlBuilder::new();
        t___12177.append_safe("active = ");
        t___12177.append_boolean(true);
        let mut t___12181: Query = t___12176.r#where(t___12177.accumulated());
        let mut t___12182: SqlBuilder = SqlBuilder::new();
        t___12182.append_safe("vip = ");
        t___12182.append_boolean(true);
        let q__1529: Query = t___12181.or_where(t___12182.accumulated());
        let mut t___12190: bool = Some(q__1529.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___128 {}
        impl ClosureGroup___128 {
            fn fn__12170(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed where and orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___128 {};
        let fn__12170 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12170())
        };
        test___115.assert(t___12190, fn__12170.clone());
        test___115.soft_fail_to_hard()
    }
    #[test]
    fn whereNull__2199() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___116 = temper_std::testing::Test::new();
        let mut t___12162: SafeIdentifier = sid__638("users");
        let mut t___12163: SafeIdentifier = sid__638("deleted_at");
        let q__1531: Query = from(t___12162.clone()).where_null(t___12163.clone());
        let mut t___12168: bool = Some(q__1531.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___129 {}
        impl ClosureGroup___129 {
            fn fn__12161(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull".to_string());
            }
        }
        let closure_group = ClosureGroup___129 {};
        let fn__12161 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12161())
        };
        test___116.assert(t___12168, fn__12161.clone());
        test___116.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNull__2200() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___117 = temper_std::testing::Test::new();
        let mut t___12153: SafeIdentifier = sid__638("users");
        let mut t___12154: SafeIdentifier = sid__638("email");
        let q__1533: Query = from(t___12153.clone()).where_not_null(t___12154.clone());
        let mut t___12159: bool = Some(q__1533.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email IS NOT NULL");
        #[derive(Clone)]
        struct ClosureGroup___130 {}
        impl ClosureGroup___130 {
            fn fn__12152(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull".to_string());
            }
        }
        let closure_group = ClosureGroup___130 {};
        let fn__12152 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12152())
        };
        test___117.assert(t___12159, fn__12152.clone());
        test___117.soft_fail_to_hard()
    }
    #[test]
    fn whereNullChainedWithWhere__2201() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___118 = temper_std::testing::Test::new();
        let mut t___12139: SafeIdentifier = sid__638("users");
        let mut t___12140: SqlBuilder = SqlBuilder::new();
        t___12140.append_safe("active = ");
        t___12140.append_boolean(true);
        let mut t___12143: SqlFragment = t___12140.accumulated();
        let q__1535: Query = from(t___12139.clone()).r#where(t___12143.clone()).where_null(sid__638("deleted_at"));
        let mut t___12150: bool = Some(q__1535.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___131 {}
        impl ClosureGroup___131 {
            fn fn__12138(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull chained".to_string());
            }
        }
        let closure_group = ClosureGroup___131 {};
        let fn__12138 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12138())
        };
        test___118.assert(t___12150, fn__12138.clone());
        test___118.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNullChainedWithOrWhere__2203() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___119 = temper_std::testing::Test::new();
        let mut t___12125: SafeIdentifier = sid__638("users");
        let mut t___12126: SafeIdentifier = sid__638("deleted_at");
        let mut t___12127: Query = from(t___12125.clone()).where_null(t___12126.clone());
        let mut t___12128: SqlBuilder = SqlBuilder::new();
        t___12128.append_safe("role = ");
        t___12128.append_string("admin");
        let q__1537: Query = t___12127.or_where(t___12128.accumulated());
        let mut t___12136: bool = Some(q__1537.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'");
        #[derive(Clone)]
        struct ClosureGroup___132 {}
        impl ClosureGroup___132 {
            fn fn__12124(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull with orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___132 {};
        let fn__12124 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12124())
        };
        test___119.assert(t___12136, fn__12124.clone());
        test___119.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithIntValues__2205() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___120 = temper_std::testing::Test::new();
        let mut t___12113: SafeIdentifier = sid__638("users");
        let mut t___12114: SafeIdentifier = sid__638("id");
        let mut t___12115: SqlInt32 = SqlInt32::new(1);
        let mut t___12116: SqlInt32 = SqlInt32::new(2);
        let mut t___12117: SqlInt32 = SqlInt32::new(3);
        let q__1539: Query = from(t___12113.clone()).where_in(t___12114.clone(), [SqlPart::new(t___12115.clone()), SqlPart::new(t___12116.clone()), SqlPart::new(t___12117.clone())]);
        let mut t___12122: bool = Some(q__1539.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___133 {}
        impl ClosureGroup___133 {
            fn fn__12112(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn ints".to_string());
            }
        }
        let closure_group = ClosureGroup___133 {};
        let fn__12112 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12112())
        };
        test___120.assert(t___12122, fn__12112.clone());
        test___120.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithStringValuesEscaping__2206() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___121 = temper_std::testing::Test::new();
        let mut t___12102: SafeIdentifier = sid__638("users");
        let mut t___12103: SafeIdentifier = sid__638("name");
        let mut t___12104: SqlString = SqlString::new("Alice");
        let mut t___12105: SqlString = SqlString::new("Bob's");
        let q__1541: Query = from(t___12102.clone()).where_in(t___12103.clone(), [SqlPart::new(t___12104.clone()), SqlPart::new(t___12105.clone())]);
        let mut t___12110: bool = Some(q__1541.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')");
        #[derive(Clone)]
        struct ClosureGroup___134 {}
        impl ClosureGroup___134 {
            fn fn__12101(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn strings".to_string());
            }
        }
        let closure_group = ClosureGroup___134 {};
        let fn__12101 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12101())
        };
        test___121.assert(t___12110, fn__12101.clone());
        test___121.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithEmptyListProduces1_0__2207() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___122 = temper_std::testing::Test::new();
        let mut t___12093: SafeIdentifier = sid__638("users");
        let mut t___12094: SafeIdentifier = sid__638("id");
        let q__1543: Query = from(t___12093.clone()).where_in(t___12094.clone(), []);
        let mut t___12099: bool = Some(q__1543.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___135 {}
        impl ClosureGroup___135 {
            fn fn__12092(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn empty".to_string());
            }
        }
        let closure_group = ClosureGroup___135 {};
        let fn__12092 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12092())
        };
        test___122.assert(t___12099, fn__12092.clone());
        test___122.soft_fail_to_hard()
    }
    #[test]
    fn whereInChained__2208() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___123 = temper_std::testing::Test::new();
        let mut t___12077: SafeIdentifier = sid__638("users");
        let mut t___12078: SqlBuilder = SqlBuilder::new();
        t___12078.append_safe("active = ");
        t___12078.append_boolean(true);
        let mut t___12081: SqlFragment = t___12078.accumulated();
        let q__1545: Query = from(t___12077.clone()).r#where(t___12081.clone()).where_in(sid__638("role"), [SqlPart::new(SqlString::new("admin")), SqlPart::new(SqlString::new("user"))]);
        let mut t___12090: bool = Some(q__1545.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')");
        #[derive(Clone)]
        struct ClosureGroup___136 {}
        impl ClosureGroup___136 {
            fn fn__12076(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn chained".to_string());
            }
        }
        let closure_group = ClosureGroup___136 {};
        let fn__12076 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12076())
        };
        test___123.assert(t___12090, fn__12076.clone());
        test___123.soft_fail_to_hard()
    }
    #[test]
    fn whereInSingleElement__2210() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___124 = temper_std::testing::Test::new();
        let mut t___12067: SafeIdentifier = sid__638("users");
        let mut t___12068: SafeIdentifier = sid__638("id");
        let mut t___12069: SqlInt32 = SqlInt32::new(42);
        let q__1547: Query = from(t___12067.clone()).where_in(t___12068.clone(), [SqlPart::new(t___12069.clone())]);
        let mut t___12074: bool = Some(q__1547.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___137 {}
        impl ClosureGroup___137 {
            fn fn__12066(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn single".to_string());
            }
        }
        let closure_group = ClosureGroup___137 {};
        let fn__12066 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12066())
        };
        test___124.assert(t___12074, fn__12066.clone());
        test___124.soft_fail_to_hard()
    }
    #[test]
    fn whereNotBasic__2211() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___125 = temper_std::testing::Test::new();
        let mut t___12055: SafeIdentifier = sid__638("users");
        let mut t___12056: SqlBuilder = SqlBuilder::new();
        t___12056.append_safe("active = ");
        t___12056.append_boolean(true);
        let mut t___12059: SqlFragment = t___12056.accumulated();
        let q__1549: Query = from(t___12055.clone()).where_not(t___12059.clone());
        let mut t___12064: bool = Some(q__1549.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE NOT (active = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___138 {}
        impl ClosureGroup___138 {
            fn fn__12054(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot".to_string());
            }
        }
        let closure_group = ClosureGroup___138 {};
        let fn__12054 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12054())
        };
        test___125.assert(t___12064, fn__12054.clone());
        test___125.soft_fail_to_hard()
    }
    #[test]
    fn whereNotChained__2213() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___126 = temper_std::testing::Test::new();
        let mut t___12038: SafeIdentifier = sid__638("users");
        let mut t___12039: SqlBuilder = SqlBuilder::new();
        t___12039.append_safe("age > ");
        t___12039.append_int32(18);
        let mut t___12042: SqlFragment = t___12039.accumulated();
        let mut t___12043: Query = from(t___12038.clone()).r#where(t___12042.clone());
        let mut t___12044: SqlBuilder = SqlBuilder::new();
        t___12044.append_safe("banned = ");
        t___12044.append_boolean(true);
        let q__1551: Query = t___12043.where_not(t___12044.accumulated());
        let mut t___12052: bool = Some(q__1551.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___139 {}
        impl ClosureGroup___139 {
            fn fn__12037(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot chained".to_string());
            }
        }
        let closure_group = ClosureGroup___139 {};
        let fn__12037 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12037())
        };
        test___126.assert(t___12052, fn__12037.clone());
        test___126.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenIntegers__2216() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___127 = temper_std::testing::Test::new();
        let mut t___12027: SafeIdentifier = sid__638("users");
        let mut t___12028: SafeIdentifier = sid__638("age");
        let mut t___12029: SqlInt32 = SqlInt32::new(18);
        let mut t___12030: SqlInt32 = SqlInt32::new(65);
        let q__1553: Query = from(t___12027.clone()).where_between(t___12028.clone(), SqlPart::new(t___12029.clone()), SqlPart::new(t___12030.clone()));
        let mut t___12035: bool = Some(q__1553.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age BETWEEN 18 AND 65");
        #[derive(Clone)]
        struct ClosureGroup___140 {}
        impl ClosureGroup___140 {
            fn fn__12026(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween ints".to_string());
            }
        }
        let closure_group = ClosureGroup___140 {};
        let fn__12026 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12026())
        };
        test___127.assert(t___12035, fn__12026.clone());
        test___127.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenChained__2217() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___128 = temper_std::testing::Test::new();
        let mut t___12011: SafeIdentifier = sid__638("users");
        let mut t___12012: SqlBuilder = SqlBuilder::new();
        t___12012.append_safe("active = ");
        t___12012.append_boolean(true);
        let mut t___12015: SqlFragment = t___12012.accumulated();
        let q__1555: Query = from(t___12011.clone()).r#where(t___12015.clone()).where_between(sid__638("age"), SqlPart::new(SqlInt32::new(21)), SqlPart::new(SqlInt32::new(30)));
        let mut t___12024: bool = Some(q__1555.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30");
        #[derive(Clone)]
        struct ClosureGroup___141 {}
        impl ClosureGroup___141 {
            fn fn__12010(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween chained".to_string());
            }
        }
        let closure_group = ClosureGroup___141 {};
        let fn__12010 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12010())
        };
        test___128.assert(t___12024, fn__12010.clone());
        test___128.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeBasic__2219() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___129 = temper_std::testing::Test::new();
        let mut t___12002: SafeIdentifier = sid__638("users");
        let mut t___12003: SafeIdentifier = sid__638("name");
        let q__1557: Query = from(t___12002.clone()).where_like(t___12003.clone(), "John%");
        let mut t___12008: bool = Some(q__1557.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE 'John%'");
        #[derive(Clone)]
        struct ClosureGroup___142 {}
        impl ClosureGroup___142 {
            fn fn__12001(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike".to_string());
            }
        }
        let closure_group = ClosureGroup___142 {};
        let fn__12001 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12001())
        };
        test___129.assert(t___12008, fn__12001.clone());
        test___129.soft_fail_to_hard()
    }
    #[test]
    fn whereIlikeBasic__2220() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___130 = temper_std::testing::Test::new();
        let mut t___11993: SafeIdentifier = sid__638("users");
        let mut t___11994: SafeIdentifier = sid__638("email");
        let q__1559: Query = from(t___11993.clone()).where_i_like(t___11994.clone(), "%@gmail.com");
        let mut t___11999: bool = Some(q__1559.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email ILIKE '%@gmail.com'");
        #[derive(Clone)]
        struct ClosureGroup___143 {}
        impl ClosureGroup___143 {
            fn fn__11992(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereILike".to_string());
            }
        }
        let closure_group = ClosureGroup___143 {};
        let fn__11992 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11992())
        };
        test___130.assert(t___11999, fn__11992.clone());
        test___130.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWithInjectionAttempt__2221() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___131 = temper_std::testing::Test::new();
        let mut t___11979: SafeIdentifier = sid__638("users");
        let mut t___11980: SafeIdentifier = sid__638("name");
        let q__1561: Query = from(t___11979.clone()).where_like(t___11980.clone(), "'; DROP TABLE users; --");
        let s__1562: std::sync::Arc<String> = q__1561.to_sql().to_string();
        let mut t___11985: bool = temper_core::string::index_of( & s__1562, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___144 {
            s__1562: std::sync::Arc<String>
        }
        impl ClosureGroup___144 {
            fn fn__11978(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like injection escaped: {}", self.s__1562.clone()));
            }
        }
        let closure_group = ClosureGroup___144 {
            s__1562: s__1562.clone()
        };
        let fn__11978 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11978())
        };
        test___131.assert(t___11985, fn__11978.clone());
        let mut t___11989: bool = temper_core::string::index_of( & s__1562, "LIKE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___145 {
            s__1562: std::sync::Arc<String>
        }
        impl ClosureGroup___145 {
            fn fn__11977(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like structure intact: {}", self.s__1562.clone()));
            }
        }
        let closure_group = ClosureGroup___145 {
            s__1562: s__1562.clone()
        };
        let fn__11977 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11977())
        };
        test___131.assert(t___11989, fn__11977.clone());
        test___131.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWildcardPatterns__2222() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___132 = temper_std::testing::Test::new();
        let mut t___11969: SafeIdentifier = sid__638("users");
        let mut t___11970: SafeIdentifier = sid__638("name");
        let q__1564: Query = from(t___11969.clone()).where_like(t___11970.clone(), "%son%");
        let mut t___11975: bool = Some(q__1564.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE '%son%'");
        #[derive(Clone)]
        struct ClosureGroup___146 {}
        impl ClosureGroup___146 {
            fn fn__11968(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike wildcard".to_string());
            }
        }
        let closure_group = ClosureGroup___146 {};
        let fn__11968 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11968())
        };
        test___132.assert(t___11975, fn__11968.clone());
        test___132.soft_fail_to_hard()
    }
    #[test]
    fn countAllProducesCount__2223() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___133 = temper_std::testing::Test::new();
        let f__1566: SqlFragment = count_all();
        let mut t___11966: bool = Some(f__1566.to_string().as_str()) == Some("COUNT(*)");
        #[derive(Clone)]
        struct ClosureGroup___147 {}
        impl ClosureGroup___147 {
            fn fn__11962(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countAll".to_string());
            }
        }
        let closure_group = ClosureGroup___147 {};
        let fn__11962 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11962())
        };
        test___133.assert(t___11966, fn__11962.clone());
        test___133.soft_fail_to_hard()
    }
    #[test]
    fn countColProducesCountField__2224() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___134 = temper_std::testing::Test::new();
        let f__1568: SqlFragment = count_col(sid__638("id"));
        let mut t___11960: bool = Some(f__1568.to_string().as_str()) == Some("COUNT(id)");
        #[derive(Clone)]
        struct ClosureGroup___148 {}
        impl ClosureGroup___148 {
            fn fn__11955(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countCol".to_string());
            }
        }
        let closure_group = ClosureGroup___148 {};
        let fn__11955 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11955())
        };
        test___134.assert(t___11960, fn__11955.clone());
        test___134.soft_fail_to_hard()
    }
    #[test]
    fn sumColProducesSumField__2225() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___135 = temper_std::testing::Test::new();
        let f__1570: SqlFragment = sum_col(sid__638("amount"));
        let mut t___11953: bool = Some(f__1570.to_string().as_str()) == Some("SUM(amount)");
        #[derive(Clone)]
        struct ClosureGroup___149 {}
        impl ClosureGroup___149 {
            fn fn__11948(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("sumCol".to_string());
            }
        }
        let closure_group = ClosureGroup___149 {};
        let fn__11948 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11948())
        };
        test___135.assert(t___11953, fn__11948.clone());
        test___135.soft_fail_to_hard()
    }
    #[test]
    fn avgColProducesAvgField__2226() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___136 = temper_std::testing::Test::new();
        let f__1572: SqlFragment = avg_col(sid__638("price"));
        let mut t___11946: bool = Some(f__1572.to_string().as_str()) == Some("AVG(price)");
        #[derive(Clone)]
        struct ClosureGroup___150 {}
        impl ClosureGroup___150 {
            fn fn__11941(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("avgCol".to_string());
            }
        }
        let closure_group = ClosureGroup___150 {};
        let fn__11941 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11941())
        };
        test___136.assert(t___11946, fn__11941.clone());
        test___136.soft_fail_to_hard()
    }
    #[test]
    fn minColProducesMinField__2227() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___137 = temper_std::testing::Test::new();
        let f__1574: SqlFragment = min_col(sid__638("created_at"));
        let mut t___11939: bool = Some(f__1574.to_string().as_str()) == Some("MIN(created_at)");
        #[derive(Clone)]
        struct ClosureGroup___151 {}
        impl ClosureGroup___151 {
            fn fn__11934(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("minCol".to_string());
            }
        }
        let closure_group = ClosureGroup___151 {};
        let fn__11934 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11934())
        };
        test___137.assert(t___11939, fn__11934.clone());
        test___137.soft_fail_to_hard()
    }
    #[test]
    fn maxColProducesMaxField__2228() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___138 = temper_std::testing::Test::new();
        let f__1576: SqlFragment = max_col(sid__638("score"));
        let mut t___11932: bool = Some(f__1576.to_string().as_str()) == Some("MAX(score)");
        #[derive(Clone)]
        struct ClosureGroup___152 {}
        impl ClosureGroup___152 {
            fn fn__11927(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("maxCol".to_string());
            }
        }
        let closure_group = ClosureGroup___152 {};
        let fn__11927 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11927())
        };
        test___138.assert(t___11932, fn__11927.clone());
        test___138.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithAggregate__2229() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___139 = temper_std::testing::Test::new();
        let mut t___11919: SafeIdentifier = sid__638("orders");
        let mut t___11920: SqlFragment = count_all();
        let q__1578: Query = from(t___11919.clone()).select_expr([t___11920.clone()]);
        let mut t___11925: bool = Some(q__1578.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM orders");
        #[derive(Clone)]
        struct ClosureGroup___153 {}
        impl ClosureGroup___153 {
            fn fn__11918(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr count".to_string());
            }
        }
        let closure_group = ClosureGroup___153 {};
        let fn__11918 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11918())
        };
        test___139.assert(t___11925, fn__11918.clone());
        test___139.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithMultipleExpressions__2230() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___140 = temper_std::testing::Test::new();
        let nameFrag__1580: SqlFragment = col(sid__638("users"), sid__638("name"));
        let mut t___11910: SafeIdentifier = sid__638("users");
        let mut t___11911: SqlFragment = count_all();
        let q__1581: Query = from(t___11910.clone()).select_expr([nameFrag__1580.clone(), t___11911.clone()]);
        let mut t___11916: bool = Some(q__1581.to_sql().to_string().as_str()) == Some("SELECT users.name, COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___154 {}
        impl ClosureGroup___154 {
            fn fn__11906(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr multi".to_string());
            }
        }
        let closure_group = ClosureGroup___154 {};
        let fn__11906 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11906())
        };
        test___140.assert(t___11916, fn__11906.clone());
        test___140.soft_fail_to_hard()
    }
    #[test]
    fn selectExprOverridesSelectedFields__2231() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___141 = temper_std::testing::Test::new();
        let mut t___11895: SafeIdentifier = sid__638("users");
        let mut t___11896: SafeIdentifier = sid__638("id");
        let mut t___11897: SafeIdentifier = sid__638("name");
        let q__1583: Query = from(t___11895.clone()).select([t___11896.clone(), t___11897.clone()]).select_expr([count_all()]);
        let mut t___11904: bool = Some(q__1583.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___155 {}
        impl ClosureGroup___155 {
            fn fn__11894(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr overrides select".to_string());
            }
        }
        let closure_group = ClosureGroup___155 {};
        let fn__11894 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11894())
        };
        test___141.assert(t___11904, fn__11894.clone());
        test___141.soft_fail_to_hard()
    }
    #[test]
    fn groupBySingleField__2232() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___142 = temper_std::testing::Test::new();
        let mut t___11881: SafeIdentifier = sid__638("orders");
        let mut t___11884: SqlFragment = col(sid__638("orders"), sid__638("status"));
        let mut t___11885: SqlFragment = count_all();
        let q__1585: Query = from(t___11881.clone()).select_expr([t___11884.clone(), t___11885.clone()]).group_by(sid__638("status"));
        let mut t___11892: bool = Some(q__1585.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status");
        #[derive(Clone)]
        struct ClosureGroup___156 {}
        impl ClosureGroup___156 {
            fn fn__11880(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy single".to_string());
            }
        }
        let closure_group = ClosureGroup___156 {};
        let fn__11880 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11880())
        };
        test___142.assert(t___11892, fn__11880.clone());
        test___142.soft_fail_to_hard()
    }
    #[test]
    fn groupByMultipleFields__2233() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___143 = temper_std::testing::Test::new();
        let mut t___11870: SafeIdentifier = sid__638("orders");
        let mut t___11871: SafeIdentifier = sid__638("status");
        let q__1587: Query = from(t___11870.clone()).group_by(t___11871.clone()).group_by(sid__638("category"));
        let mut t___11878: bool = Some(q__1587.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status, category");
        #[derive(Clone)]
        struct ClosureGroup___157 {}
        impl ClosureGroup___157 {
            fn fn__11869(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy multiple".to_string());
            }
        }
        let closure_group = ClosureGroup___157 {};
        let fn__11869 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11869())
        };
        test___143.assert(t___11878, fn__11869.clone());
        test___143.soft_fail_to_hard()
    }
    #[test]
    fn havingBasic__2234() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___144 = temper_std::testing::Test::new();
        let mut t___11851: SafeIdentifier = sid__638("orders");
        let mut t___11854: SqlFragment = col(sid__638("orders"), sid__638("status"));
        let mut t___11855: SqlFragment = count_all();
        let mut t___11858: Query = from(t___11851.clone()).select_expr([t___11854.clone(), t___11855.clone()]).group_by(sid__638("status"));
        let mut t___11859: SqlBuilder = SqlBuilder::new();
        t___11859.append_safe("COUNT(*) > ");
        t___11859.append_int32(5);
        let q__1589: Query = t___11858.having(t___11859.accumulated());
        let mut t___11867: bool = Some(q__1589.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status HAVING COUNT(*) > 5");
        #[derive(Clone)]
        struct ClosureGroup___158 {}
        impl ClosureGroup___158 {
            fn fn__11850(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("having basic".to_string());
            }
        }
        let closure_group = ClosureGroup___158 {};
        let fn__11850 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11850())
        };
        test___144.assert(t___11867, fn__11850.clone());
        test___144.soft_fail_to_hard()
    }
    #[test]
    fn orHaving__2236() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___145 = temper_std::testing::Test::new();
        let mut t___11832: SafeIdentifier = sid__638("orders");
        let mut t___11833: SafeIdentifier = sid__638("status");
        let mut t___11834: Query = from(t___11832.clone()).group_by(t___11833.clone());
        let mut t___11835: SqlBuilder = SqlBuilder::new();
        t___11835.append_safe("COUNT(*) > ");
        t___11835.append_int32(5);
        let mut t___11839: Query = t___11834.having(t___11835.accumulated());
        let mut t___11840: SqlBuilder = SqlBuilder::new();
        t___11840.append_safe("SUM(total) > ");
        t___11840.append_int32(1000);
        let q__1591: Query = t___11839.or_having(t___11840.accumulated());
        let mut t___11848: bool = Some(q__1591.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status HAVING COUNT(*) > 5 OR SUM(total) > 1000");
        #[derive(Clone)]
        struct ClosureGroup___159 {}
        impl ClosureGroup___159 {
            fn fn__11831(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orHaving".to_string());
            }
        }
        let closure_group = ClosureGroup___159 {};
        let fn__11831 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11831())
        };
        test___145.assert(t___11848, fn__11831.clone());
        test___145.soft_fail_to_hard()
    }
    #[test]
    fn distinctBasic__2239() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___146 = temper_std::testing::Test::new();
        let mut t___11822: SafeIdentifier = sid__638("users");
        let mut t___11823: SafeIdentifier = sid__638("name");
        let q__1593: Query = from(t___11822.clone()).select([t___11823.clone()]).distinct();
        let mut t___11829: bool = Some(q__1593.to_sql().to_string().as_str()) == Some("SELECT DISTINCT name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___160 {}
        impl ClosureGroup___160 {
            fn fn__11821(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct".to_string());
            }
        }
        let closure_group = ClosureGroup___160 {};
        let fn__11821 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11821())
        };
        test___146.assert(t___11829, fn__11821.clone());
        test___146.soft_fail_to_hard()
    }
    #[test]
    fn distinctWithWhere__2240() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___147 = temper_std::testing::Test::new();
        let mut t___11807: SafeIdentifier = sid__638("users");
        let mut t___11808: SafeIdentifier = sid__638("email");
        let mut t___11809: Query = from(t___11807.clone()).select([t___11808.clone()]);
        let mut t___11810: SqlBuilder = SqlBuilder::new();
        t___11810.append_safe("active = ");
        t___11810.append_boolean(true);
        let q__1595: Query = t___11809.r#where(t___11810.accumulated()).distinct();
        let mut t___11819: bool = Some(q__1595.to_sql().to_string().as_str()) == Some("SELECT DISTINCT email FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___161 {}
        impl ClosureGroup___161 {
            fn fn__11806(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct with where".to_string());
            }
        }
        let closure_group = ClosureGroup___161 {};
        let fn__11806 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11806())
        };
        test___147.assert(t___11819, fn__11806.clone());
        test___147.soft_fail_to_hard()
    }
    #[test]
    fn countSqlBare__2242() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___148 = temper_std::testing::Test::new();
        let q__1597: Query = from(sid__638("users"));
        let mut t___11804: bool = Some(q__1597.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___162 {}
        impl ClosureGroup___162 {
            fn fn__11799(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql bare".to_string());
            }
        }
        let closure_group = ClosureGroup___162 {};
        let fn__11799 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11799())
        };
        test___148.assert(t___11804, fn__11799.clone());
        test___148.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithWhere__2243() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___149 = temper_std::testing::Test::new();
        let mut t___11788: SafeIdentifier = sid__638("users");
        let mut t___11789: SqlBuilder = SqlBuilder::new();
        t___11789.append_safe("active = ");
        t___11789.append_boolean(true);
        let mut t___11792: SqlFragment = t___11789.accumulated();
        let q__1599: Query = from(t___11788.clone()).r#where(t___11792.clone());
        let mut t___11797: bool = Some(q__1599.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___163 {}
        impl ClosureGroup___163 {
            fn fn__11787(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with where".to_string());
            }
        }
        let closure_group = ClosureGroup___163 {};
        let fn__11787 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11787())
        };
        test___149.assert(t___11797, fn__11787.clone());
        test___149.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithJoin__2245() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___150 = temper_std::testing::Test::new();
        let mut t___11771: SafeIdentifier = sid__638("users");
        let mut t___11772: SafeIdentifier = sid__638("orders");
        let mut t___11773: SqlBuilder = SqlBuilder::new();
        t___11773.append_safe("users.id = orders.user_id");
        let mut t___11775: SqlFragment = t___11773.accumulated();
        let mut t___11776: Query = from(t___11771.clone()).inner_join(t___11772.clone(), t___11775.clone());
        let mut t___11777: SqlBuilder = SqlBuilder::new();
        t___11777.append_safe("orders.total > ");
        t___11777.append_int32(100);
        let q__1601: Query = t___11776.r#where(t___11777.accumulated());
        let mut t___11785: bool = Some(q__1601.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100");
        #[derive(Clone)]
        struct ClosureGroup___164 {}
        impl ClosureGroup___164 {
            fn fn__11770(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with join".to_string());
            }
        }
        let closure_group = ClosureGroup___164 {};
        let fn__11770 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11770())
        };
        test___150.assert(t___11785, fn__11770.clone());
        test___150.soft_fail_to_hard()
    }
    #[test]
    fn countSqlDropsOrderByLimitOffset__2248() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___151 = temper_std::testing::Test::new();
        let mut t___11757: SafeIdentifier;
        let mut t___11758: SqlBuilder;
        let mut t___11761: SqlFragment;
        let mut t___5865: Query;
        let mut t___5866: Query;
        let q__1603: Query;
        'ok___14351: {
            'orelse___2434: {
                t___11757 = sid__638("users");
                t___11758 = SqlBuilder::new();
                t___11758.append_safe("active = ");
                t___11758.append_boolean(true);
                t___11761 = t___11758.accumulated();
                t___5865 = match from(t___11757.clone()).r#where(t___11761.clone()).order_by(sid__638("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2434
                };
                t___5866 = match t___5865.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2434
                };
                q__1603 = t___5866.clone();
                break 'ok___14351;
            }
            q__1603 = panic!();
        }
        let s__1604: std::sync::Arc<String> = q__1603.count_sql().to_string();
        let mut t___11768: bool = Some(s__1604.as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___165 {
            s__1604: std::sync::Arc<String>
        }
        impl ClosureGroup___165 {
            fn fn__11756(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("countSql drops extras: {}", self.s__1604.clone()));
            }
        }
        let closure_group = ClosureGroup___165 {
            s__1604: s__1604.clone()
        };
        let fn__11756 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11756())
        };
        test___151.assert(t___11768, fn__11756.clone());
        test___151.soft_fail_to_hard()
    }
    #[test]
    fn fullAggregationQuery__2250() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___152 = temper_std::testing::Test::new();
        let mut t___11724: SafeIdentifier = sid__638("orders");
        let mut t___11727: SqlFragment = col(sid__638("orders"), sid__638("status"));
        let mut t___11728: SqlFragment = count_all();
        let mut t___11730: SqlFragment = sum_col(sid__638("total"));
        let mut t___11731: Query = from(t___11724.clone()).select_expr([t___11727.clone(), t___11728.clone(), t___11730.clone()]);
        let mut t___11732: SafeIdentifier = sid__638("users");
        let mut t___11733: SqlBuilder = SqlBuilder::new();
        t___11733.append_safe("orders.user_id = users.id");
        let mut t___11736: Query = t___11731.inner_join(t___11732.clone(), t___11733.accumulated());
        let mut t___11737: SqlBuilder = SqlBuilder::new();
        t___11737.append_safe("users.active = ");
        t___11737.append_boolean(true);
        let mut t___11743: Query = t___11736.r#where(t___11737.accumulated()).group_by(sid__638("status"));
        let mut t___11744: SqlBuilder = SqlBuilder::new();
        t___11744.append_safe("COUNT(*) > ");
        t___11744.append_int32(3);
        let q__1606: Query = t___11743.having(t___11744.accumulated()).order_by(sid__638("status"), true);
        let expected__1607: std::sync::Arc<String> = std::sync::Arc::new("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC".to_string());
        let mut t___11754: bool = Some(q__1606.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC");
        #[derive(Clone)]
        struct ClosureGroup___166 {}
        impl ClosureGroup___166 {
            fn fn__11723(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full aggregation".to_string());
            }
        }
        let closure_group = ClosureGroup___166 {};
        let fn__11723 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11723())
        };
        test___152.assert(t___11754, fn__11723.clone());
        test___152.soft_fail_to_hard()
    }
    #[test]
    fn unionSql__2254() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___153 = temper_std::testing::Test::new();
        let mut t___11706: SafeIdentifier = sid__638("users");
        let mut t___11707: SqlBuilder = SqlBuilder::new();
        t___11707.append_safe("role = ");
        t___11707.append_string("admin");
        let mut t___11710: SqlFragment = t___11707.accumulated();
        let a__1609: Query = from(t___11706.clone()).r#where(t___11710.clone());
        let mut t___11712: SafeIdentifier = sid__638("users");
        let mut t___11713: SqlBuilder = SqlBuilder::new();
        t___11713.append_safe("role = ");
        t___11713.append_string("moderator");
        let mut t___11716: SqlFragment = t___11713.accumulated();
        let b__1610: Query = from(t___11712.clone()).r#where(t___11716.clone());
        let s__1611: std::sync::Arc<String> = union_sql(a__1609.clone(), b__1610.clone()).to_string();
        let mut t___11721: bool = Some(s__1611.as_str()) == Some("(SELECT * FROM users WHERE role = 'admin') UNION (SELECT * FROM users WHERE role = 'moderator')");
        #[derive(Clone)]
        struct ClosureGroup___167 {
            s__1611: std::sync::Arc<String>
        }
        impl ClosureGroup___167 {
            fn fn__11705(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionSql: {}", self.s__1611.clone()));
            }
        }
        let closure_group = ClosureGroup___167 {
            s__1611: s__1611.clone()
        };
        let fn__11705 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11705())
        };
        test___153.assert(t___11721, fn__11705.clone());
        test___153.soft_fail_to_hard()
    }
    #[test]
    fn unionAllSql__2257() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___154 = temper_std::testing::Test::new();
        let mut t___11694: SafeIdentifier = sid__638("users");
        let mut t___11695: SafeIdentifier = sid__638("name");
        let a__1613: Query = from(t___11694.clone()).select([t___11695.clone()]);
        let mut t___11697: SafeIdentifier = sid__638("contacts");
        let mut t___11698: SafeIdentifier = sid__638("name");
        let b__1614: Query = from(t___11697.clone()).select([t___11698.clone()]);
        let s__1615: std::sync::Arc<String> = union_all_sql(a__1613.clone(), b__1614.clone()).to_string();
        let mut t___11703: bool = Some(s__1615.as_str()) == Some("(SELECT name FROM users) UNION ALL (SELECT name FROM contacts)");
        #[derive(Clone)]
        struct ClosureGroup___168 {
            s__1615: std::sync::Arc<String>
        }
        impl ClosureGroup___168 {
            fn fn__11693(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionAllSql: {}", self.s__1615.clone()));
            }
        }
        let closure_group = ClosureGroup___168 {
            s__1615: s__1615.clone()
        };
        let fn__11693 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11693())
        };
        test___154.assert(t___11703, fn__11693.clone());
        test___154.soft_fail_to_hard()
    }
    #[test]
    fn intersectSql__2258() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___155 = temper_std::testing::Test::new();
        let mut t___11682: SafeIdentifier = sid__638("users");
        let mut t___11683: SafeIdentifier = sid__638("email");
        let a__1617: Query = from(t___11682.clone()).select([t___11683.clone()]);
        let mut t___11685: SafeIdentifier = sid__638("subscribers");
        let mut t___11686: SafeIdentifier = sid__638("email");
        let b__1618: Query = from(t___11685.clone()).select([t___11686.clone()]);
        let s__1619: std::sync::Arc<String> = intersect_sql(a__1617.clone(), b__1618.clone()).to_string();
        let mut t___11691: bool = Some(s__1619.as_str()) == Some("(SELECT email FROM users) INTERSECT (SELECT email FROM subscribers)");
        #[derive(Clone)]
        struct ClosureGroup___169 {
            s__1619: std::sync::Arc<String>
        }
        impl ClosureGroup___169 {
            fn fn__11681(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("intersectSql: {}", self.s__1619.clone()));
            }
        }
        let closure_group = ClosureGroup___169 {
            s__1619: s__1619.clone()
        };
        let fn__11681 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11681())
        };
        test___155.assert(t___11691, fn__11681.clone());
        test___155.soft_fail_to_hard()
    }
    #[test]
    fn exceptSql__2259() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___156 = temper_std::testing::Test::new();
        let mut t___11670: SafeIdentifier = sid__638("users");
        let mut t___11671: SafeIdentifier = sid__638("id");
        let a__1621: Query = from(t___11670.clone()).select([t___11671.clone()]);
        let mut t___11673: SafeIdentifier = sid__638("banned");
        let mut t___11674: SafeIdentifier = sid__638("id");
        let b__1622: Query = from(t___11673.clone()).select([t___11674.clone()]);
        let s__1623: std::sync::Arc<String> = except_sql(a__1621.clone(), b__1622.clone()).to_string();
        let mut t___11679: bool = Some(s__1623.as_str()) == Some("(SELECT id FROM users) EXCEPT (SELECT id FROM banned)");
        #[derive(Clone)]
        struct ClosureGroup___170 {
            s__1623: std::sync::Arc<String>
        }
        impl ClosureGroup___170 {
            fn fn__11669(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exceptSql: {}", self.s__1623.clone()));
            }
        }
        let closure_group = ClosureGroup___170 {
            s__1623: s__1623.clone()
        };
        let fn__11669 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11669())
        };
        test___156.assert(t___11679, fn__11669.clone());
        test___156.soft_fail_to_hard()
    }
    #[test]
    fn subqueryWithAlias__2260() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___157 = temper_std::testing::Test::new();
        let mut t___11655: SafeIdentifier = sid__638("orders");
        let mut t___11656: SafeIdentifier = sid__638("user_id");
        let mut t___11657: Query = from(t___11655.clone()).select([t___11656.clone()]);
        let mut t___11658: SqlBuilder = SqlBuilder::new();
        t___11658.append_safe("total > ");
        t___11658.append_int32(100);
        let inner__1625: Query = t___11657.r#where(t___11658.accumulated());
        let s__1626: std::sync::Arc<String> = subquery(inner__1625.clone(), sid__638("big_orders")).to_string();
        let mut t___11667: bool = Some(s__1626.as_str()) == Some("(SELECT user_id FROM orders WHERE total > 100) AS big_orders");
        #[derive(Clone)]
        struct ClosureGroup___171 {
            s__1626: std::sync::Arc<String>
        }
        impl ClosureGroup___171 {
            fn fn__11654(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("subquery: {}", self.s__1626.clone()));
            }
        }
        let closure_group = ClosureGroup___171 {
            s__1626: s__1626.clone()
        };
        let fn__11654 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11654())
        };
        test___157.assert(t___11667, fn__11654.clone());
        test___157.soft_fail_to_hard()
    }
    #[test]
    fn existsSql__2262() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___158 = temper_std::testing::Test::new();
        let mut t___11644: SafeIdentifier = sid__638("orders");
        let mut t___11645: SqlBuilder = SqlBuilder::new();
        t___11645.append_safe("orders.user_id = users.id");
        let mut t___11647: SqlFragment = t___11645.accumulated();
        let inner__1628: Query = from(t___11644.clone()).r#where(t___11647.clone());
        let s__1629: std::sync::Arc<String> = exists_sql(inner__1628.clone()).to_string();
        let mut t___11652: bool = Some(s__1629.as_str()) == Some("EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___172 {
            s__1629: std::sync::Arc<String>
        }
        impl ClosureGroup___172 {
            fn fn__11643(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("existsSql: {}", self.s__1629.clone()));
            }
        }
        let closure_group = ClosureGroup___172 {
            s__1629: s__1629.clone()
        };
        let fn__11643 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11643())
        };
        test___158.assert(t___11652, fn__11643.clone());
        test___158.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubquery__2264() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___159 = temper_std::testing::Test::new();
        let mut t___11627: SafeIdentifier = sid__638("orders");
        let mut t___11628: SafeIdentifier = sid__638("user_id");
        let mut t___11629: Query = from(t___11627.clone()).select([t___11628.clone()]);
        let mut t___11630: SqlBuilder = SqlBuilder::new();
        t___11630.append_safe("total > ");
        t___11630.append_int32(1000);
        let sub__1631: Query = t___11629.r#where(t___11630.accumulated());
        let mut t___11635: SafeIdentifier = sid__638("users");
        let mut t___11636: SafeIdentifier = sid__638("id");
        let q__1632: Query = from(t___11635.clone()).where_in_subquery(t___11636.clone(), sub__1631.clone());
        let s__1633: std::sync::Arc<String> = q__1632.to_sql().to_string();
        let mut t___11641: bool = Some(s__1633.as_str()) == Some("SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 1000)");
        #[derive(Clone)]
        struct ClosureGroup___173 {
            s__1633: std::sync::Arc<String>
        }
        impl ClosureGroup___173 {
            fn fn__11626(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery: {}", self.s__1633.clone()));
            }
        }
        let closure_group = ClosureGroup___173 {
            s__1633: s__1633.clone()
        };
        let fn__11626 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11626())
        };
        test___159.assert(t___11641, fn__11626.clone());
        test___159.soft_fail_to_hard()
    }
    #[test]
    fn setOperationWithWhereOnEachSide__2266() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___160 = temper_std::testing::Test::new();
        let mut t___11604: SafeIdentifier = sid__638("users");
        let mut t___11605: SqlBuilder = SqlBuilder::new();
        t___11605.append_safe("age > ");
        t___11605.append_int32(18);
        let mut t___11608: SqlFragment = t___11605.accumulated();
        let mut t___11609: Query = from(t___11604.clone()).r#where(t___11608.clone());
        let mut t___11610: SqlBuilder = SqlBuilder::new();
        t___11610.append_safe("active = ");
        t___11610.append_boolean(true);
        let a__1635: Query = t___11609.r#where(t___11610.accumulated());
        let mut t___11615: SafeIdentifier = sid__638("users");
        let mut t___11616: SqlBuilder = SqlBuilder::new();
        t___11616.append_safe("role = ");
        t___11616.append_string("vip");
        let mut t___11619: SqlFragment = t___11616.accumulated();
        let b__1636: Query = from(t___11615.clone()).r#where(t___11619.clone());
        let s__1637: std::sync::Arc<String> = union_sql(a__1635.clone(), b__1636.clone()).to_string();
        let mut t___11624: bool = Some(s__1637.as_str()) == Some("(SELECT * FROM users WHERE age > 18 AND active = TRUE) UNION (SELECT * FROM users WHERE role = 'vip')");
        #[derive(Clone)]
        struct ClosureGroup___174 {
            s__1637: std::sync::Arc<String>
        }
        impl ClosureGroup___174 {
            fn fn__11603(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("union with where: {}", self.s__1637.clone()));
            }
        }
        let closure_group = ClosureGroup___174 {
            s__1637: s__1637.clone()
        };
        let fn__11603 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11603())
        };
        test___160.assert(t___11624, fn__11603.clone());
        test___160.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubqueryChainedWithWhere__2270() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___161 = temper_std::testing::Test::new();
        let mut t___11587: SafeIdentifier = sid__638("orders");
        let mut t___11588: SafeIdentifier = sid__638("user_id");
        let sub__1639: Query = from(t___11587.clone()).select([t___11588.clone()]);
        let mut t___11590: SafeIdentifier = sid__638("users");
        let mut t___11591: SqlBuilder = SqlBuilder::new();
        t___11591.append_safe("active = ");
        t___11591.append_boolean(true);
        let mut t___11594: SqlFragment = t___11591.accumulated();
        let q__1640: Query = from(t___11590.clone()).r#where(t___11594.clone()).where_in_subquery(sid__638("id"), sub__1639.clone());
        let s__1641: std::sync::Arc<String> = q__1640.to_sql().to_string();
        let mut t___11601: bool = Some(s__1641.as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND id IN (SELECT user_id FROM orders)");
        #[derive(Clone)]
        struct ClosureGroup___175 {
            s__1641: std::sync::Arc<String>
        }
        impl ClosureGroup___175 {
            fn fn__11586(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery chained: {}", self.s__1641.clone()));
            }
        }
        let closure_group = ClosureGroup___175 {
            s__1641: s__1641.clone()
        };
        let fn__11586 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11586())
        };
        test___161.assert(t___11601, fn__11586.clone());
        test___161.soft_fail_to_hard()
    }
    #[test]
    fn existsSqlUsedInWhere__2272() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___162 = temper_std::testing::Test::new();
        let mut t___11573: SafeIdentifier = sid__638("orders");
        let mut t___11574: SqlBuilder = SqlBuilder::new();
        t___11574.append_safe("orders.user_id = users.id");
        let mut t___11576: SqlFragment = t___11574.accumulated();
        let sub__1643: Query = from(t___11573.clone()).r#where(t___11576.clone());
        let mut t___11578: SafeIdentifier = sid__638("users");
        let mut t___11579: SqlFragment = exists_sql(sub__1643.clone());
        let q__1644: Query = from(t___11578.clone()).r#where(t___11579.clone());
        let s__1645: std::sync::Arc<String> = q__1644.to_sql().to_string();
        let mut t___11584: bool = Some(s__1645.as_str()) == Some("SELECT * FROM users WHERE EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___176 {
            s__1645: std::sync::Arc<String>
        }
        impl ClosureGroup___176 {
            fn fn__11572(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exists in where: {}", self.s__1645.clone()));
            }
        }
        let closure_group = ClosureGroup___176 {
            s__1645: s__1645.clone()
        };
        let fn__11572 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11572())
        };
        test___162.assert(t___11584, fn__11572.clone());
        test___162.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBasic__2274() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___163 = temper_std::testing::Test::new();
        let mut t___11559: SafeIdentifier;
        let mut t___11560: SafeIdentifier;
        let mut t___11561: SqlString;
        let mut t___11562: UpdateQuery;
        let mut t___11563: SqlBuilder;
        let mut t___5687: SqlFragment;
        let q__1647: SqlFragment;
        'ok___14352: {
            'orelse___2435: {
                t___11559 = sid__638("users");
                t___11560 = sid__638("name");
                t___11561 = SqlString::new("Alice");
                t___11562 = update(t___11559.clone()).set(t___11560.clone(), SqlPart::new(t___11561.clone()));
                t___11563 = SqlBuilder::new();
                t___11563.append_safe("id = ");
                t___11563.append_int32(1);
                t___5687 = match t___11562.r#where(t___11563.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2435
                };
                q__1647 = t___5687.clone();
                break 'ok___14352;
            }
            q__1647 = panic!();
        }
        let mut t___11570: bool = Some(q__1647.to_string().as_str()) == Some("UPDATE users SET name = 'Alice' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___177 {}
        impl ClosureGroup___177 {
            fn fn__11558(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update basic".to_string());
            }
        }
        let closure_group = ClosureGroup___177 {};
        let fn__11558 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11558())
        };
        test___163.assert(t___11570, fn__11558.clone());
        test___163.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleSet__2276() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___164 = temper_std::testing::Test::new();
        let mut t___11542: SafeIdentifier;
        let mut t___11543: SafeIdentifier;
        let mut t___11544: SqlString;
        let mut t___11548: UpdateQuery;
        let mut t___11549: SqlBuilder;
        let mut t___5672: SqlFragment;
        let q__1649: SqlFragment;
        'ok___14353: {
            'orelse___2436: {
                t___11542 = sid__638("users");
                t___11543 = sid__638("name");
                t___11544 = SqlString::new("Bob");
                t___11548 = update(t___11542.clone()).set(t___11543.clone(), SqlPart::new(t___11544.clone())).set(sid__638("age"), SqlPart::new(SqlInt32::new(30)));
                t___11549 = SqlBuilder::new();
                t___11549.append_safe("id = ");
                t___11549.append_int32(2);
                t___5672 = match t___11548.r#where(t___11549.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2436
                };
                q__1649 = t___5672.clone();
                break 'ok___14353;
            }
            q__1649 = panic!();
        }
        let mut t___11556: bool = Some(q__1649.to_string().as_str()) == Some("UPDATE users SET name = 'Bob', age = 30 WHERE id = 2");
        #[derive(Clone)]
        struct ClosureGroup___178 {}
        impl ClosureGroup___178 {
            fn fn__11541(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi set".to_string());
            }
        }
        let closure_group = ClosureGroup___178 {};
        let fn__11541 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11541())
        };
        test___164.assert(t___11556, fn__11541.clone());
        test___164.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleWhere__2278() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___165 = temper_std::testing::Test::new();
        let mut t___11523: SafeIdentifier;
        let mut t___11524: SafeIdentifier;
        let mut t___11525: SqlBoolean;
        let mut t___11526: UpdateQuery;
        let mut t___11527: SqlBuilder;
        let mut t___11531: UpdateQuery;
        let mut t___11532: SqlBuilder;
        let mut t___5654: SqlFragment;
        let q__1651: SqlFragment;
        'ok___14354: {
            'orelse___2437: {
                t___11523 = sid__638("users");
                t___11524 = sid__638("active");
                t___11525 = SqlBoolean::new(false);
                t___11526 = update(t___11523.clone()).set(t___11524.clone(), SqlPart::new(t___11525.clone()));
                t___11527 = SqlBuilder::new();
                t___11527.append_safe("age < ");
                t___11527.append_int32(18);
                t___11531 = t___11526.r#where(t___11527.accumulated());
                t___11532 = SqlBuilder::new();
                t___11532.append_safe("role = ");
                t___11532.append_string("guest");
                t___5654 = match t___11531.r#where(t___11532.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2437
                };
                q__1651 = t___5654.clone();
                break 'ok___14354;
            }
            q__1651 = panic!();
        }
        let mut t___11539: bool = Some(q__1651.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE age < 18 AND role = 'guest'");
        #[derive(Clone)]
        struct ClosureGroup___179 {}
        impl ClosureGroup___179 {
            fn fn__11522(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___179 {};
        let fn__11522 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11522())
        };
        test___165.assert(t___11539, fn__11522.clone());
        test___165.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryOrWhere__2281() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___166 = temper_std::testing::Test::new();
        let mut t___11504: SafeIdentifier;
        let mut t___11505: SafeIdentifier;
        let mut t___11506: SqlString;
        let mut t___11507: UpdateQuery;
        let mut t___11508: SqlBuilder;
        let mut t___11512: UpdateQuery;
        let mut t___11513: SqlBuilder;
        let mut t___5633: SqlFragment;
        let q__1653: SqlFragment;
        'ok___14355: {
            'orelse___2438: {
                t___11504 = sid__638("users");
                t___11505 = sid__638("status");
                t___11506 = SqlString::new("banned");
                t___11507 = update(t___11504.clone()).set(t___11505.clone(), SqlPart::new(t___11506.clone()));
                t___11508 = SqlBuilder::new();
                t___11508.append_safe("spam_count > ");
                t___11508.append_int32(10);
                t___11512 = t___11507.r#where(t___11508.accumulated());
                t___11513 = SqlBuilder::new();
                t___11513.append_safe("reported = ");
                t___11513.append_boolean(true);
                t___5633 = match t___11512.or_where(t___11513.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2438
                };
                q__1653 = t___5633.clone();
                break 'ok___14355;
            }
            q__1653 = panic!();
        }
        let mut t___11520: bool = Some(q__1653.to_string().as_str()) == Some("UPDATE users SET status = 'banned' WHERE spam_count > 10 OR reported = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___180 {}
        impl ClosureGroup___180 {
            fn fn__11503(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___180 {};
        let fn__11503 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11503())
        };
        test___166.assert(t___11520, fn__11503.clone());
        test___166.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutWhere__2284() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___167 = temper_std::testing::Test::new();
        let mut t___11497: SafeIdentifier;
        let mut t___11498: SafeIdentifier;
        let mut t___11499: SqlInt32;
        let didBubble__1655: bool;
        'ok___14356: {
            'orelse___2439: {
                t___11497 = sid__638("users");
                t___11498 = sid__638("x");
                t___11499 = SqlInt32::new(1);
                match update(t___11497.clone()).set(t___11498.clone(), SqlPart::new(t___11499.clone())).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2439
                };
                didBubble__1655 = false;
                break 'ok___14356;
            }
            didBubble__1655 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___181 {}
        impl ClosureGroup___181 {
            fn fn__11496(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___181 {};
        let fn__11496 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11496())
        };
        test___167.assert(didBubble__1655, fn__11496.clone());
        test___167.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutSet__2285() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___168 = temper_std::testing::Test::new();
        let mut t___11488: SafeIdentifier;
        let mut t___11489: SqlBuilder;
        let mut t___11492: SqlFragment;
        let didBubble__1657: bool;
        'ok___14357: {
            'orelse___2440: {
                t___11488 = sid__638("users");
                t___11489 = SqlBuilder::new();
                t___11489.append_safe("id = ");
                t___11489.append_int32(1);
                t___11492 = t___11489.accumulated();
                match update(t___11488.clone()).r#where(t___11492.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2440
                };
                didBubble__1657 = false;
                break 'ok___14357;
            }
            didBubble__1657 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___182 {}
        impl ClosureGroup___182 {
            fn fn__11487(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without SET should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___182 {};
        let fn__11487 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11487())
        };
        test___168.assert(didBubble__1657, fn__11487.clone());
        test___168.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryWithLimit__2287() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___169 = temper_std::testing::Test::new();
        let mut t___11474: SafeIdentifier;
        let mut t___11475: SafeIdentifier;
        let mut t___11476: SqlBoolean;
        let mut t___11477: UpdateQuery;
        let mut t___11478: SqlBuilder;
        let mut t___5596: UpdateQuery;
        let mut t___5597: SqlFragment;
        let q__1659: SqlFragment;
        'ok___14358: {
            'orelse___2441: {
                t___11474 = sid__638("users");
                t___11475 = sid__638("active");
                t___11476 = SqlBoolean::new(false);
                t___11477 = update(t___11474.clone()).set(t___11475.clone(), SqlPart::new(t___11476.clone()));
                t___11478 = SqlBuilder::new();
                t___11478.append_safe("last_login < ");
                t___11478.append_string("2024-01-01");
                t___5596 = match t___11477.r#where(t___11478.accumulated()).limit(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2441
                };
                t___5597 = match t___5596.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2441
                };
                q__1659 = t___5597.clone();
                break 'ok___14358;
            }
            q__1659 = panic!();
        }
        let mut t___11485: bool = Some(q__1659.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE last_login < '2024-01-01' LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___183 {}
        impl ClosureGroup___183 {
            fn fn__11473(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update limit".to_string());
            }
        }
        let closure_group = ClosureGroup___183 {};
        let fn__11473 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11473())
        };
        test___169.assert(t___11485, fn__11473.clone());
        test___169.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryEscaping__2289() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___170 = temper_std::testing::Test::new();
        let mut t___11460: SafeIdentifier;
        let mut t___11461: SafeIdentifier;
        let mut t___11462: SqlString;
        let mut t___11463: UpdateQuery;
        let mut t___11464: SqlBuilder;
        let mut t___5581: SqlFragment;
        let q__1661: SqlFragment;
        'ok___14359: {
            'orelse___2442: {
                t___11460 = sid__638("users");
                t___11461 = sid__638("bio");
                t___11462 = SqlString::new("It's a test");
                t___11463 = update(t___11460.clone()).set(t___11461.clone(), SqlPart::new(t___11462.clone()));
                t___11464 = SqlBuilder::new();
                t___11464.append_safe("id = ");
                t___11464.append_int32(1);
                t___5581 = match t___11463.r#where(t___11464.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2442
                };
                q__1661 = t___5581.clone();
                break 'ok___14359;
            }
            q__1661 = panic!();
        }
        let mut t___11471: bool = Some(q__1661.to_string().as_str()) == Some("UPDATE users SET bio = 'It''s a test' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___184 {}
        impl ClosureGroup___184 {
            fn fn__11459(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update escaping".to_string());
            }
        }
        let closure_group = ClosureGroup___184 {};
        let fn__11459 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11459())
        };
        test___170.assert(t___11471, fn__11459.clone());
        test___170.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBasic__2291() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___171 = temper_std::testing::Test::new();
        let mut t___11449: SafeIdentifier;
        let mut t___11450: SqlBuilder;
        let mut t___11453: SqlFragment;
        let mut t___5566: SqlFragment;
        let q__1663: SqlFragment;
        'ok___14360: {
            'orelse___2443: {
                t___11449 = sid__638("users");
                t___11450 = SqlBuilder::new();
                t___11450.append_safe("id = ");
                t___11450.append_int32(1);
                t___11453 = t___11450.accumulated();
                t___5566 = match delete_from(t___11449.clone()).r#where(t___11453.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2443
                };
                q__1663 = t___5566.clone();
                break 'ok___14360;
            }
            q__1663 = panic!();
        }
        let mut t___11457: bool = Some(q__1663.to_string().as_str()) == Some("DELETE FROM users WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___185 {}
        impl ClosureGroup___185 {
            fn fn__11448(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete basic".to_string());
            }
        }
        let closure_group = ClosureGroup___185 {};
        let fn__11448 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11448())
        };
        test___171.assert(t___11457, fn__11448.clone());
        test___171.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryMultipleWhere__2293() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___172 = temper_std::testing::Test::new();
        let mut t___11433: SafeIdentifier;
        let mut t___11434: SqlBuilder;
        let mut t___11437: SqlFragment;
        let mut t___11438: DeleteQuery;
        let mut t___11439: SqlBuilder;
        let mut t___5554: SqlFragment;
        let q__1665: SqlFragment;
        'ok___14361: {
            'orelse___2444: {
                t___11433 = sid__638("logs");
                t___11434 = SqlBuilder::new();
                t___11434.append_safe("created_at < ");
                t___11434.append_string("2024-01-01");
                t___11437 = t___11434.accumulated();
                t___11438 = delete_from(t___11433.clone()).r#where(t___11437.clone());
                t___11439 = SqlBuilder::new();
                t___11439.append_safe("level = ");
                t___11439.append_string("debug");
                t___5554 = match t___11438.r#where(t___11439.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2444
                };
                q__1665 = t___5554.clone();
                break 'ok___14361;
            }
            q__1665 = panic!();
        }
        let mut t___11446: bool = Some(q__1665.to_string().as_str()) == Some("DELETE FROM logs WHERE created_at < '2024-01-01' AND level = 'debug'");
        #[derive(Clone)]
        struct ClosureGroup___186 {}
        impl ClosureGroup___186 {
            fn fn__11432(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___186 {};
        let fn__11432 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11432())
        };
        test___172.assert(t___11446, fn__11432.clone());
        test___172.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBubblesWithoutWhere__2296() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___173 = temper_std::testing::Test::new();
        let didBubble__1667: bool;
        'ok___14362: {
            'orelse___2445: {
                match delete_from(sid__638("users")).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2445
                };
                didBubble__1667 = false;
                break 'ok___14362;
            }
            didBubble__1667 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___187 {}
        impl ClosureGroup___187 {
            fn fn__11428(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___187 {};
        let fn__11428 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11428())
        };
        test___173.assert(didBubble__1667, fn__11428.clone());
        test___173.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryOrWhere__2297() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___174 = temper_std::testing::Test::new();
        let mut t___11413: SafeIdentifier;
        let mut t___11414: SqlBuilder;
        let mut t___11417: SqlFragment;
        let mut t___11418: DeleteQuery;
        let mut t___11419: SqlBuilder;
        let mut t___5533: SqlFragment;
        let q__1669: SqlFragment;
        'ok___14363: {
            'orelse___2446: {
                t___11413 = sid__638("sessions");
                t___11414 = SqlBuilder::new();
                t___11414.append_safe("expired = ");
                t___11414.append_boolean(true);
                t___11417 = t___11414.accumulated();
                t___11418 = delete_from(t___11413.clone()).r#where(t___11417.clone());
                t___11419 = SqlBuilder::new();
                t___11419.append_safe("created_at < ");
                t___11419.append_string("2023-01-01");
                t___5533 = match t___11418.or_where(t___11419.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2446
                };
                q__1669 = t___5533.clone();
                break 'ok___14363;
            }
            q__1669 = panic!();
        }
        let mut t___11426: bool = Some(q__1669.to_string().as_str()) == Some("DELETE FROM sessions WHERE expired = TRUE OR created_at < '2023-01-01'");
        #[derive(Clone)]
        struct ClosureGroup___188 {}
        impl ClosureGroup___188 {
            fn fn__11412(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___188 {};
        let fn__11412 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11412())
        };
        test___174.assert(t___11426, fn__11412.clone());
        test___174.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryWithLimit__2300() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___175 = temper_std::testing::Test::new();
        let mut t___11402: SafeIdentifier;
        let mut t___11403: SqlBuilder;
        let mut t___11406: SqlFragment;
        let mut t___5514: DeleteQuery;
        let mut t___5515: SqlFragment;
        let q__1671: SqlFragment;
        'ok___14364: {
            'orelse___2447: {
                t___11402 = sid__638("logs");
                t___11403 = SqlBuilder::new();
                t___11403.append_safe("level = ");
                t___11403.append_string("debug");
                t___11406 = t___11403.accumulated();
                t___5514 = match delete_from(t___11402.clone()).r#where(t___11406.clone()).limit(1000) {
                    Ok(x) => x,
                    _ => break 'orelse___2447
                };
                t___5515 = match t___5514.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2447
                };
                q__1671 = t___5515.clone();
                break 'ok___14364;
            }
            q__1671 = panic!();
        }
        let mut t___11410: bool = Some(q__1671.to_string().as_str()) == Some("DELETE FROM logs WHERE level = 'debug' LIMIT 1000");
        #[derive(Clone)]
        struct ClosureGroup___189 {}
        impl ClosureGroup___189 {
            fn fn__11401(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete limit".to_string());
            }
        }
        let closure_group = ClosureGroup___189 {};
        let fn__11401 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11401())
        };
        test___175.assert(t___11410, fn__11401.clone());
        test___175.soft_fail_to_hard()
    }
    #[test]
    fn orderByNullsNullsFirst__2302() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___176 = temper_std::testing::Test::new();
        let mut t___11392: SafeIdentifier = sid__638("users");
        let mut t___11393: SafeIdentifier = sid__638("email");
        let mut t___11394: NullsFirst = NullsFirst::new();
        let q__1673: Query = from(t___11392.clone()).order_by_nulls(t___11393.clone(), true, NullsPosition::new(t___11394.clone()));
        let mut t___11399: bool = Some(q__1673.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY email ASC NULLS FIRST");
        #[derive(Clone)]
        struct ClosureGroup___190 {}
        impl ClosureGroup___190 {
            fn fn__11391(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("nulls first".to_string());
            }
        }
        let closure_group = ClosureGroup___190 {};
        let fn__11391 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11391())
        };
        test___176.assert(t___11399, fn__11391.clone());
        test___176.soft_fail_to_hard()
    }
    #[test]
    fn orderByNullsNullsLast__2303() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___177 = temper_std::testing::Test::new();
        let mut t___11382: SafeIdentifier = sid__638("users");
        let mut t___11383: SafeIdentifier = sid__638("score");
        let mut t___11384: NullsLast = NullsLast::new();
        let q__1675: Query = from(t___11382.clone()).order_by_nulls(t___11383.clone(), false, NullsPosition::new(t___11384.clone()));
        let mut t___11389: bool = Some(q__1675.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY score DESC NULLS LAST");
        #[derive(Clone)]
        struct ClosureGroup___191 {}
        impl ClosureGroup___191 {
            fn fn__11381(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("nulls last".to_string());
            }
        }
        let closure_group = ClosureGroup___191 {};
        let fn__11381 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11381())
        };
        test___177.assert(t___11389, fn__11381.clone());
        test___177.soft_fail_to_hard()
    }
    #[test]
    fn mixedOrderByAndOrderByNulls__2304() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___178 = temper_std::testing::Test::new();
        let mut t___11370: SafeIdentifier = sid__638("users");
        let mut t___11371: SafeIdentifier = sid__638("name");
        let q__1677: Query = from(t___11370.clone()).order_by(t___11371.clone(), true).order_by_nulls(sid__638("email"), true, NullsPosition::new(NullsFirst::new()));
        let mut t___11379: bool = Some(q__1677.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC, email ASC NULLS FIRST");
        #[derive(Clone)]
        struct ClosureGroup___192 {}
        impl ClosureGroup___192 {
            fn fn__11369(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed order".to_string());
            }
        }
        let closure_group = ClosureGroup___192 {};
        let fn__11369 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11369())
        };
        test___178.assert(t___11379, fn__11369.clone());
        test___178.soft_fail_to_hard()
    }
    #[test]
    fn crossJoin__2305() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___179 = temper_std::testing::Test::new();
        let mut t___11361: SafeIdentifier = sid__638("users");
        let mut t___11362: SafeIdentifier = sid__638("colors");
        let q__1679: Query = from(t___11361.clone()).cross_join(t___11362.clone());
        let mut t___11367: bool = Some(q__1679.to_sql().to_string().as_str()) == Some("SELECT * FROM users CROSS JOIN colors");
        #[derive(Clone)]
        struct ClosureGroup___193 {}
        impl ClosureGroup___193 {
            fn fn__11360(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("cross join".to_string());
            }
        }
        let closure_group = ClosureGroup___193 {};
        let fn__11360 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11360())
        };
        test___179.assert(t___11367, fn__11360.clone());
        test___179.soft_fail_to_hard()
    }
    #[test]
    fn crossJoinCombinedWithOtherJoins__2306() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___180 = temper_std::testing::Test::new();
        let mut t___11347: SafeIdentifier = sid__638("users");
        let mut t___11348: SafeIdentifier = sid__638("orders");
        let mut t___11349: SqlBuilder = SqlBuilder::new();
        t___11349.append_safe("users.id = orders.user_id");
        let mut t___11351: SqlFragment = t___11349.accumulated();
        let q__1681: Query = from(t___11347.clone()).inner_join(t___11348.clone(), t___11351.clone()).cross_join(sid__638("colors"));
        let mut t___11358: bool = Some(q__1681.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id CROSS JOIN colors");
        #[derive(Clone)]
        struct ClosureGroup___194 {}
        impl ClosureGroup___194 {
            fn fn__11346(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("cross + inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___194 {};
        let fn__11346 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11346())
        };
        test___180.assert(t___11358, fn__11346.clone());
        test___180.soft_fail_to_hard()
    }
    #[test]
    fn lockForUpdate__2308() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___181 = temper_std::testing::Test::new();
        let mut t___11333: SafeIdentifier = sid__638("users");
        let mut t___11334: SqlBuilder = SqlBuilder::new();
        t___11334.append_safe("id = ");
        t___11334.append_int32(1);
        let mut t___11337: SqlFragment = t___11334.accumulated();
        let q__1683: Query = from(t___11333.clone()).r#where(t___11337.clone()).lock(LockMode::new(ForUpdate::new()));
        let mut t___11344: bool = Some(q__1683.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id = 1 FOR UPDATE");
        #[derive(Clone)]
        struct ClosureGroup___195 {}
        impl ClosureGroup___195 {
            fn fn__11332(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("for update".to_string());
            }
        }
        let closure_group = ClosureGroup___195 {};
        let fn__11332 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11332())
        };
        test___181.assert(t___11344, fn__11332.clone());
        test___181.soft_fail_to_hard()
    }
    #[test]
    fn lockForShare__2310() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___182 = temper_std::testing::Test::new();
        let mut t___11322: SafeIdentifier = sid__638("users");
        let mut t___11323: SafeIdentifier = sid__638("name");
        let q__1685: Query = from(t___11322.clone()).select([t___11323.clone()]).lock(LockMode::new(ForShare::new()));
        let mut t___11330: bool = Some(q__1685.to_sql().to_string().as_str()) == Some("SELECT name FROM users FOR SHARE");
        #[derive(Clone)]
        struct ClosureGroup___196 {}
        impl ClosureGroup___196 {
            fn fn__11321(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("for share".to_string());
            }
        }
        let closure_group = ClosureGroup___196 {};
        let fn__11321 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11321())
        };
        test___182.assert(t___11330, fn__11321.clone());
        test___182.soft_fail_to_hard()
    }
    #[test]
    fn lockWithFullQuery__2311() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___183 = temper_std::testing::Test::new();
        let mut t___11308: SafeIdentifier;
        let mut t___11309: SqlBuilder;
        let mut t___11312: SqlFragment;
        let mut t___11315: Query;
        let mut t___5438: Query;
        let q__1687: Query;
        'ok___14365: {
            'orelse___2448: {
                t___11308 = sid__638("accounts");
                t___11309 = SqlBuilder::new();
                t___11309.append_safe("id = ");
                t___11309.append_int32(42);
                t___11312 = t___11309.accumulated();
                t___5438 = match from(t___11308.clone()).r#where(t___11312.clone()).limit(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2448
                };
                t___11315 = t___5438.lock(LockMode::new(ForUpdate::new()));
                q__1687 = t___11315.clone();
                break 'ok___14365;
            }
            q__1687 = panic!();
        }
        let mut t___11319: bool = Some(q__1687.to_sql().to_string().as_str()) == Some("SELECT * FROM accounts WHERE id = 42 LIMIT 1 FOR UPDATE");
        #[derive(Clone)]
        struct ClosureGroup___197 {}
        impl ClosureGroup___197 {
            fn fn__11307(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("lock full query".to_string());
            }
        }
        let closure_group = ClosureGroup___197 {};
        let fn__11307 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11307())
        };
        test___183.assert(t___11319, fn__11307.clone());
        test___183.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__2313() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___190 = temper_std::testing::Test::new();
        let mut t___5427: SafeIdentifier;
        let id__1725: SafeIdentifier;
        'ok___14366: {
            'orelse___2449: {
                t___5427 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___2449
                };
                id__1725 = t___5427.clone();
                break 'ok___14366;
            }
            id__1725 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___11305: bool = Some(id__1725.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___198 {}
        impl ClosureGroup___198 {
            fn fn__11302(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___198 {};
        let fn__11302 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11302())
        };
        test___190.assert(t___11305, fn__11302.clone());
        test___190.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__2314() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___191 = temper_std::testing::Test::new();
        let didBubble__1727: bool;
        'ok___14367: {
            'orelse___2450: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___2450
                };
                didBubble__1727 = false;
                break 'ok___14367;
            }
            didBubble__1727 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___199 {}
        impl ClosureGroup___199 {
            fn fn__11299(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___199 {};
        let fn__11299 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11299())
        };
        test___191.assert(didBubble__1727, fn__11299.clone());
        test___191.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__2315() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___192 = temper_std::testing::Test::new();
        let didBubble__1729: bool;
        'ok___14368: {
            'orelse___2451: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___2451
                };
                didBubble__1729 = false;
                break 'ok___14368;
            }
            didBubble__1729 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___200 {}
        impl ClosureGroup___200 {
            fn fn__11296(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___200 {};
        let fn__11296 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11296())
        };
        test___192.assert(didBubble__1729, fn__11296.clone());
        test___192.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__2316() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___193 = temper_std::testing::Test::new();
        let cases__1731: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___201 {
            test___193: temper_std::testing::Test
        }
        impl ClosureGroup___201 {
            fn fn__11293(& self, c__1732: impl temper_core::ToArcString) {
                let c__1732 = c__1732.to_arc_string();
                let didBubble__1733: bool;
                'ok___14369: {
                    'orelse___2452: {
                        match safe_identifier(c__1732.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___2452
                        };
                        didBubble__1733 = false;
                        break 'ok___14369;
                    }
                    didBubble__1733 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___202 {
                    c__1732: std::sync::Arc<String>
                }
                impl ClosureGroup___202 {
                    fn fn__11290(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__1732.clone()));
                    }
                }
                let closure_group = ClosureGroup___202 {
                    c__1732: c__1732.clone()
                };
                let fn__11290 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__11290())
                };
                self.test___193.assert(didBubble__1733, fn__11290.clone());
            }
        }
        let closure_group = ClosureGroup___201 {
            test___193: test___193.clone()
        };
        let fn__11293 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1732: std::sync::Arc<String> | closure_group.fn__11293(c__1732))
        };
        temper_core::listed::list_for_each( & cases__1731, & ( * fn__11293.clone()));
        test___193.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__2317() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___194 = temper_std::testing::Test::new();
        let mut t___5404: SafeIdentifier;
        let mut t___5405: SafeIdentifier;
        let mut t___5406: SafeIdentifier;
        let mut t___5407: SafeIdentifier;
        let mut t___5410: SafeIdentifier;
        let mut t___5411: SafeIdentifier;
        let mut t___5415: FieldDef;
        'ok___14370: {
            'orelse___2453: {
                t___5404 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2453
                };
                t___5405 = t___5404.clone();
                break 'ok___14370;
            }
            t___5405 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___14371: {
            'orelse___2454: {
                t___5406 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2454
                };
                t___5407 = t___5406.clone();
                break 'ok___14371;
            }
            t___5407 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___11280: StringField = StringField::new();
        let mut t___11281: FieldDef = FieldDef::new(t___5407.clone(), FieldType::new(t___11280.clone()), false);
        'ok___14372: {
            'orelse___2455: {
                t___5410 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___2455
                };
                t___5411 = t___5410.clone();
                break 'ok___14372;
            }
            t___5411 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___11282: IntField = IntField::new();
        let mut t___11283: FieldDef = FieldDef::new(t___5411.clone(), FieldType::new(t___11282.clone()), false);
        let td__1735: TableDef = TableDef::new(t___5405.clone(), [t___11281.clone(), t___11283.clone()]);
        let f__1736: FieldDef;
        'ok___14373: {
            'orelse___2456: {
                t___5415 = match td__1735.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___2456
                };
                f__1736 = t___5415.clone();
                break 'ok___14373;
            }
            f__1736 = panic!();
        }
        let mut t___11288: bool = Some(f__1736.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___203 {}
        impl ClosureGroup___203 {
            fn fn__11279(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___203 {};
        let fn__11279 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11279())
        };
        test___194.assert(t___11288, fn__11279.clone());
        test___194.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__2318() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___195 = temper_std::testing::Test::new();
        let mut t___5395: SafeIdentifier;
        let mut t___5396: SafeIdentifier;
        let mut t___5397: SafeIdentifier;
        let mut t___5398: SafeIdentifier;
        'ok___14374: {
            'orelse___2457: {
                t___5395 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2457
                };
                t___5396 = t___5395.clone();
                break 'ok___14374;
            }
            t___5396 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___14375: {
            'orelse___2458: {
                t___5397 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2458
                };
                t___5398 = t___5397.clone();
                break 'ok___14375;
            }
            t___5398 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___11274: StringField = StringField::new();
        let mut t___11275: FieldDef = FieldDef::new(t___5398.clone(), FieldType::new(t___11274.clone()), false);
        let td__1738: TableDef = TableDef::new(t___5396.clone(), [t___11275.clone()]);
        let didBubble__1739: bool;
        'ok___14376: {
            'orelse___2459: {
                match td__1738.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___2459
                };
                didBubble__1739 = false;
                break 'ok___14376;
            }
            didBubble__1739 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___204 {}
        impl ClosureGroup___204 {
            fn fn__11273(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___204 {};
        let fn__11273 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11273())
        };
        test___195.assert(didBubble__1739, fn__11273.clone());
        test___195.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__2319() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___196 = temper_std::testing::Test::new();
        let mut t___5383: SafeIdentifier;
        let mut t___5384: SafeIdentifier;
        let mut t___5387: SafeIdentifier;
        let mut t___5388: SafeIdentifier;
        'ok___14377: {
            'orelse___2460: {
                t___5383 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___2460
                };
                t___5384 = t___5383.clone();
                break 'ok___14377;
            }
            t___5384 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___11262: StringField = StringField::new();
        let required__1741: FieldDef = FieldDef::new(t___5384.clone(), FieldType::new(t___11262.clone()), false);
        'ok___14378: {
            'orelse___2461: {
                t___5387 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___2461
                };
                t___5388 = t___5387.clone();
                break 'ok___14378;
            }
            t___5388 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___11264: StringField = StringField::new();
        let optional__1742: FieldDef = FieldDef::new(t___5388.clone(), FieldType::new(t___11264.clone()), true);
        let mut t___11268: bool = ! required__1741.nullable();
        #[derive(Clone)]
        struct ClosureGroup___205 {}
        impl ClosureGroup___205 {
            fn fn__11261(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___205 {};
        let fn__11261 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11261())
        };
        test___196.assert(t___11268, fn__11261.clone());
        let mut t___11270: bool = optional__1742.nullable();
        #[derive(Clone)]
        struct ClosureGroup___206 {}
        impl ClosureGroup___206 {
            fn fn__11260(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___206 {};
        let fn__11260 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11260())
        };
        test___196.assert(t___11270, fn__11260.clone());
        test___196.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__2320() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___200 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___207 {}
        impl ClosureGroup___207 {
            fn build__1868(& self, name__1870: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1870 = name__1870.to_arc_string();
                let mut t___11242: SqlBuilder = SqlBuilder::new();
                t___11242.append_safe("select * from hi where name = ");
                t___11242.append_string(name__1870.clone());
                return t___11242.accumulated().to_string();
            }
            fn buildWrong__1869(& self, name__1872: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1872 = name__1872.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__1872.clone()));
            }
        }
        let closure_group = ClosureGroup___207 {};
        let build__1868 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1870: std::sync::Arc<String> | closure_group.build__1868(name__1870))
        };
        let buildWrong__1869 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1872: std::sync::Arc<String> | closure_group.buildWrong__1869(name__1872))
        };
        let actual___2322: std::sync::Arc<String> = build__1868(std::sync::Arc::new("world".to_string()));
        let mut t___11252: bool = Some(actual___2322.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___208 {
            actual___2322: std::sync::Arc<String>
        }
        impl ClosureGroup___208 {
            fn fn__11249(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___2322.clone()));
            }
        }
        let closure_group = ClosureGroup___208 {
            actual___2322: actual___2322.clone()
        };
        let fn__11249 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11249())
        };
        test___200.assert(t___11252, fn__11249.clone());
        let bobbyTables__1874: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___2324: std::sync::Arc<String> = build__1868(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___11256: bool = Some(actual___2324.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___209 {
            actual___2324: std::sync::Arc<String>
        }
        impl ClosureGroup___209 {
            fn fn__11248(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___2324.clone()));
            }
        }
        let closure_group = ClosureGroup___209 {
            actual___2324: actual___2324.clone()
        };
        let fn__11248 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11248())
        };
        test___200.assert(t___11256, fn__11248.clone());
        #[derive(Clone)]
        struct ClosureGroup___210 {}
        impl ClosureGroup___210 {
            fn fn__11247(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___210 {};
        let fn__11247 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11247())
        };
        test___200.assert(true, fn__11247.clone());
        test___200.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__2328() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___201 = temper_std::testing::Test::new();
        let mut t___11210: SqlBuilder = SqlBuilder::new();
        t___11210.append_safe("v = ");
        t___11210.append_string("");
        let actual___2329: std::sync::Arc<String> = t___11210.accumulated().to_string();
        let mut t___11216: bool = Some(actual___2329.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___211 {
            actual___2329: std::sync::Arc<String>
        }
        impl ClosureGroup___211 {
            fn fn__11209(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___2329.clone()));
            }
        }
        let closure_group = ClosureGroup___211 {
            actual___2329: actual___2329.clone()
        };
        let fn__11209 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11209())
        };
        test___201.assert(t___11216, fn__11209.clone());
        let mut t___11218: SqlBuilder = SqlBuilder::new();
        t___11218.append_safe("v = ");
        t___11218.append_string("a''b");
        let actual___2332: std::sync::Arc<String> = t___11218.accumulated().to_string();
        let mut t___11224: bool = Some(actual___2332.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___212 {
            actual___2332: std::sync::Arc<String>
        }
        impl ClosureGroup___212 {
            fn fn__11208(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___2332.clone()));
            }
        }
        let closure_group = ClosureGroup___212 {
            actual___2332: actual___2332.clone()
        };
        let fn__11208 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11208())
        };
        test___201.assert(t___11224, fn__11208.clone());
        let mut t___11226: SqlBuilder = SqlBuilder::new();
        t___11226.append_safe("v = ");
        t___11226.append_string("Hello 世界");
        let actual___2335: std::sync::Arc<String> = t___11226.accumulated().to_string();
        let mut t___11232: bool = Some(actual___2335.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___213 {
            actual___2335: std::sync::Arc<String>
        }
        impl ClosureGroup___213 {
            fn fn__11207(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___2335.clone()));
            }
        }
        let closure_group = ClosureGroup___213 {
            actual___2335: actual___2335.clone()
        };
        let fn__11207 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11207())
        };
        test___201.assert(t___11232, fn__11207.clone());
        let mut t___11234: SqlBuilder = SqlBuilder::new();
        t___11234.append_safe("v = ");
        t___11234.append_string("Line1\x0aLine2");
        let actual___2338: std::sync::Arc<String> = t___11234.accumulated().to_string();
        let mut t___11240: bool = Some(actual___2338.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___214 {
            actual___2338: std::sync::Arc<String>
        }
        impl ClosureGroup___214 {
            fn fn__11206(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___2338.clone()));
            }
        }
        let closure_group = ClosureGroup___214 {
            actual___2338: actual___2338.clone()
        };
        let fn__11206 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11206())
        };
        test___201.assert(t___11240, fn__11206.clone());
        test___201.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__2341() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___202 = temper_std::testing::Test::new();
        let mut t___5328: temper_std::temporal::Date;
        let mut t___11181: SqlBuilder = SqlBuilder::new();
        t___11181.append_safe("select ");
        t___11181.append_int32(42);
        t___11181.append_safe(", ");
        t___11181.append_int64(43);
        t___11181.append_safe(", ");
        t___11181.append_float64(19.99f64);
        t___11181.append_safe(", ");
        t___11181.append_boolean(true);
        t___11181.append_safe(", ");
        t___11181.append_boolean(false);
        let actual___2342: std::sync::Arc<String> = t___11181.accumulated().to_string();
        let mut t___11195: bool = Some(actual___2342.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___215 {
            actual___2342: std::sync::Arc<String>
        }
        impl ClosureGroup___215 {
            fn fn__11180(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___2342.clone()));
            }
        }
        let closure_group = ClosureGroup___215 {
            actual___2342: actual___2342.clone()
        };
        let fn__11180 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11180())
        };
        test___202.assert(t___11195, fn__11180.clone());
        let date__1877: temper_std::temporal::Date;
        'ok___14379: {
            'orelse___2462: {
                t___5328 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___2462
                };
                date__1877 = t___5328.clone();
                break 'ok___14379;
            }
            date__1877 = panic!();
        }
        let mut t___11197: SqlBuilder = SqlBuilder::new();
        t___11197.append_safe("insert into t values (");
        t___11197.append_date(date__1877.clone());
        t___11197.append_safe(")");
        let actual___2345: std::sync::Arc<String> = t___11197.accumulated().to_string();
        let mut t___11204: bool = Some(actual___2345.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___216 {
            actual___2345: std::sync::Arc<String>
        }
        impl ClosureGroup___216 {
            fn fn__11179(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___2345.clone()));
            }
        }
        let closure_group = ClosureGroup___216 {
            actual___2345: actual___2345.clone()
        };
        let fn__11179 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11179())
        };
        test___202.assert(t___11204, fn__11179.clone());
        test___202.soft_fail_to_hard()
    }
    #[test]
    fn lists__2348() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___203 = temper_std::testing::Test::new();
        let mut t___5300: temper_std::temporal::Date;
        let mut t___5301: temper_std::temporal::Date;
        let mut t___5302: temper_std::temporal::Date;
        let mut t___5303: temper_std::temporal::Date;
        let mut t___11125: SqlBuilder = SqlBuilder::new();
        t___11125.append_safe("v IN (");
        t___11125.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___11125.append_safe(")");
        let actual___2349: std::sync::Arc<String> = t___11125.accumulated().to_string();
        let mut t___11132: bool = Some(actual___2349.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___217 {
            actual___2349: std::sync::Arc<String>
        }
        impl ClosureGroup___217 {
            fn fn__11124(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___2349.clone()));
            }
        }
        let closure_group = ClosureGroup___217 {
            actual___2349: actual___2349.clone()
        };
        let fn__11124 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11124())
        };
        test___203.assert(t___11132, fn__11124.clone());
        let mut t___11134: SqlBuilder = SqlBuilder::new();
        t___11134.append_safe("v IN (");
        t___11134.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___11134.append_safe(")");
        let actual___2352: std::sync::Arc<String> = t___11134.accumulated().to_string();
        let mut t___11141: bool = Some(actual___2352.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___218 {
            actual___2352: std::sync::Arc<String>
        }
        impl ClosureGroup___218 {
            fn fn__11123(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___2352.clone()));
            }
        }
        let closure_group = ClosureGroup___218 {
            actual___2352: actual___2352.clone()
        };
        let fn__11123 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11123())
        };
        test___203.assert(t___11141, fn__11123.clone());
        let mut t___11143: SqlBuilder = SqlBuilder::new();
        t___11143.append_safe("v IN (");
        t___11143.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___11143.append_safe(")");
        let actual___2355: std::sync::Arc<String> = t___11143.accumulated().to_string();
        let mut t___11150: bool = Some(actual___2355.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___219 {
            actual___2355: std::sync::Arc<String>
        }
        impl ClosureGroup___219 {
            fn fn__11122(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___2355.clone()));
            }
        }
        let closure_group = ClosureGroup___219 {
            actual___2355: actual___2355.clone()
        };
        let fn__11122 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11122())
        };
        test___203.assert(t___11150, fn__11122.clone());
        let mut t___11152: SqlBuilder = SqlBuilder::new();
        t___11152.append_safe("v IN (");
        t___11152.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___11152.append_safe(")");
        let actual___2358: std::sync::Arc<String> = t___11152.accumulated().to_string();
        let mut t___11159: bool = Some(actual___2358.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___220 {
            actual___2358: std::sync::Arc<String>
        }
        impl ClosureGroup___220 {
            fn fn__11121(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___2358.clone()));
            }
        }
        let closure_group = ClosureGroup___220 {
            actual___2358: actual___2358.clone()
        };
        let fn__11121 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11121())
        };
        test___203.assert(t___11159, fn__11121.clone());
        let mut t___11161: SqlBuilder = SqlBuilder::new();
        t___11161.append_safe("v IN (");
        t___11161.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___11161.append_safe(")");
        let actual___2361: std::sync::Arc<String> = t___11161.accumulated().to_string();
        let mut t___11168: bool = Some(actual___2361.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___221 {
            actual___2361: std::sync::Arc<String>
        }
        impl ClosureGroup___221 {
            fn fn__11120(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___2361.clone()));
            }
        }
        let closure_group = ClosureGroup___221 {
            actual___2361: actual___2361.clone()
        };
        let fn__11120 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11120())
        };
        test___203.assert(t___11168, fn__11120.clone());
        'ok___14380: {
            'orelse___2463: {
                t___5300 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___2463
                };
                t___5301 = t___5300.clone();
                break 'ok___14380;
            }
            t___5301 = panic!();
        }
        'ok___14381: {
            'orelse___2464: {
                t___5302 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___2464
                };
                t___5303 = t___5302.clone();
                break 'ok___14381;
            }
            t___5303 = panic!();
        }
        let dates__1879: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___5301.clone(), t___5303.clone()]);
        let mut t___11170: SqlBuilder = SqlBuilder::new();
        t___11170.append_safe("v IN (");
        t___11170.append_date_list(temper_core::ToListed::to_listed(dates__1879.clone()));
        t___11170.append_safe(")");
        let actual___2364: std::sync::Arc<String> = t___11170.accumulated().to_string();
        let mut t___11177: bool = Some(actual___2364.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___222 {
            actual___2364: std::sync::Arc<String>
        }
        impl ClosureGroup___222 {
            fn fn__11119(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___2364.clone()));
            }
        }
        let closure_group = ClosureGroup___222 {
            actual___2364: actual___2364.clone()
        };
        let fn__11119 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11119())
        };
        test___203.assert(t___11177, fn__11119.clone());
        test___203.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__2367() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___204 = temper_std::testing::Test::new();
        let nan__1881: f64;
        nan__1881 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___11111: SqlBuilder = SqlBuilder::new();
        t___11111.append_safe("v = ");
        t___11111.append_float64(nan__1881);
        let actual___2368: std::sync::Arc<String> = t___11111.accumulated().to_string();
        let mut t___11117: bool = Some(actual___2368.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___223 {
            actual___2368: std::sync::Arc<String>
        }
        impl ClosureGroup___223 {
            fn fn__11110(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___2368.clone()));
            }
        }
        let closure_group = ClosureGroup___223 {
            actual___2368: actual___2368.clone()
        };
        let fn__11110 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11110())
        };
        test___204.assert(t___11117, fn__11110.clone());
        test___204.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__2371() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___205 = temper_std::testing::Test::new();
        let inf__1883: f64;
        inf__1883 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___11102: SqlBuilder = SqlBuilder::new();
        t___11102.append_safe("v = ");
        t___11102.append_float64(inf__1883);
        let actual___2372: std::sync::Arc<String> = t___11102.accumulated().to_string();
        let mut t___11108: bool = Some(actual___2372.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___224 {
            actual___2372: std::sync::Arc<String>
        }
        impl ClosureGroup___224 {
            fn fn__11101(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___2372.clone()));
            }
        }
        let closure_group = ClosureGroup___224 {
            actual___2372: actual___2372.clone()
        };
        let fn__11101 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11101())
        };
        test___205.assert(t___11108, fn__11101.clone());
        test___205.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__2375() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___206 = temper_std::testing::Test::new();
        let ninf__1885: f64;
        ninf__1885 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___11093: SqlBuilder = SqlBuilder::new();
        t___11093.append_safe("v = ");
        t___11093.append_float64(ninf__1885);
        let actual___2376: std::sync::Arc<String> = t___11093.accumulated().to_string();
        let mut t___11099: bool = Some(actual___2376.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___225 {
            actual___2376: std::sync::Arc<String>
        }
        impl ClosureGroup___225 {
            fn fn__11092(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___2376.clone()));
            }
        }
        let closure_group = ClosureGroup___225 {
            actual___2376: actual___2376.clone()
        };
        let fn__11092 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11092())
        };
        test___206.assert(t___11099, fn__11092.clone());
        test___206.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__2379() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___207 = temper_std::testing::Test::new();
        let mut t___11068: SqlBuilder = SqlBuilder::new();
        t___11068.append_safe("v = ");
        t___11068.append_float64(3.14f64);
        let actual___2380: std::sync::Arc<String> = t___11068.accumulated().to_string();
        let mut t___11074: bool = Some(actual___2380.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___226 {
            actual___2380: std::sync::Arc<String>
        }
        impl ClosureGroup___226 {
            fn fn__11067(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___2380.clone()));
            }
        }
        let closure_group = ClosureGroup___226 {
            actual___2380: actual___2380.clone()
        };
        let fn__11067 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11067())
        };
        test___207.assert(t___11074, fn__11067.clone());
        let mut t___11076: SqlBuilder = SqlBuilder::new();
        t___11076.append_safe("v = ");
        t___11076.append_float64(0.0f64);
        let actual___2383: std::sync::Arc<String> = t___11076.accumulated().to_string();
        let mut t___11082: bool = Some(actual___2383.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___227 {
            actual___2383: std::sync::Arc<String>
        }
        impl ClosureGroup___227 {
            fn fn__11066(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___2383.clone()));
            }
        }
        let closure_group = ClosureGroup___227 {
            actual___2383: actual___2383.clone()
        };
        let fn__11066 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11066())
        };
        test___207.assert(t___11082, fn__11066.clone());
        let mut t___11084: SqlBuilder = SqlBuilder::new();
        t___11084.append_safe("v = ");
        t___11084.append_float64(-42.5f64);
        let actual___2386: std::sync::Arc<String> = t___11084.accumulated().to_string();
        let mut t___11090: bool = Some(actual___2386.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___228 {
            actual___2386: std::sync::Arc<String>
        }
        impl ClosureGroup___228 {
            fn fn__11065(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___2386.clone()));
            }
        }
        let closure_group = ClosureGroup___228 {
            actual___2386: actual___2386.clone()
        };
        let fn__11065 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11065())
        };
        test___207.assert(t___11090, fn__11065.clone());
        test___207.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__2389() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___208 = temper_std::testing::Test::new();
        let mut t___5196: temper_std::temporal::Date;
        let d__1888: temper_std::temporal::Date;
        'ok___14382: {
            'orelse___2465: {
                t___5196 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___2465
                };
                d__1888 = t___5196.clone();
                break 'ok___14382;
            }
            d__1888 = panic!();
        }
        let mut t___11057: SqlBuilder = SqlBuilder::new();
        t___11057.append_safe("v = ");
        t___11057.append_date(d__1888.clone());
        let actual___2390: std::sync::Arc<String> = t___11057.accumulated().to_string();
        let mut t___11063: bool = Some(actual___2390.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___229 {
            actual___2390: std::sync::Arc<String>
        }
        impl ClosureGroup___229 {
            fn fn__11056(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___2390.clone()));
            }
        }
        let closure_group = ClosureGroup___229 {
            actual___2390: actual___2390.clone()
        };
        let fn__11056 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11056())
        };
        test___208.assert(t___11063, fn__11056.clone());
        test___208.soft_fail_to_hard()
    }
    #[test]
    fn nesting__2393() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___209 = temper_std::testing::Test::new();
        let name__1890: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___11025: SqlBuilder = SqlBuilder::new();
        t___11025.append_safe("where p.last_name = ");
        t___11025.append_string("Someone");
        let condition__1891: SqlFragment = t___11025.accumulated();
        let mut t___11029: SqlBuilder = SqlBuilder::new();
        t___11029.append_safe("select p.id from person p ");
        t___11029.append_fragment(condition__1891.clone());
        let actual___2395: std::sync::Arc<String> = t___11029.accumulated().to_string();
        let mut t___11035: bool = Some(actual___2395.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___230 {
            actual___2395: std::sync::Arc<String>
        }
        impl ClosureGroup___230 {
            fn fn__11024(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___2395.clone()));
            }
        }
        let closure_group = ClosureGroup___230 {
            actual___2395: actual___2395.clone()
        };
        let fn__11024 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11024())
        };
        test___209.assert(t___11035, fn__11024.clone());
        let mut t___11037: SqlBuilder = SqlBuilder::new();
        t___11037.append_safe("select p.id from person p ");
        t___11037.append_part(SqlPart::new(condition__1891.to_source()));
        let actual___2398: std::sync::Arc<String> = t___11037.accumulated().to_string();
        let mut t___11044: bool = Some(actual___2398.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___231 {
            actual___2398: std::sync::Arc<String>
        }
        impl ClosureGroup___231 {
            fn fn__11023(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___2398.clone()));
            }
        }
        let closure_group = ClosureGroup___231 {
            actual___2398: actual___2398.clone()
        };
        let fn__11023 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11023())
        };
        test___209.assert(t___11044, fn__11023.clone());
        let parts__1892: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___11048: SqlBuilder = SqlBuilder::new();
        t___11048.append_safe("select ");
        t___11048.append_part_list(parts__1892.clone());
        let actual___2401: std::sync::Arc<String> = t___11048.accumulated().to_string();
        let mut t___11054: bool = Some(actual___2401.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___232 {
            actual___2401: std::sync::Arc<String>
        }
        impl ClosureGroup___232 {
            fn fn__11022(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___2401.clone()));
            }
        }
        let closure_group = ClosureGroup___232 {
            actual___2401: actual___2401.clone()
        };
        let fn__11022 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11022())
        };
        test___209.assert(t___11054, fn__11022.clone());
        test___209.soft_fail_to_hard()
    }
    use super::*;
}
