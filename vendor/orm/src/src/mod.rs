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
    pub fn new(field__670: impl temper_core::ToArcString, message__671: impl temper_core::ToArcString) -> ChangesetError {
        let field__670 = field__670.to_arc_string();
        let message__671 = message__671.to_arc_string();
        let field;
        let message;
        field = field__670.clone();
        message = message__671.clone();
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
    pub fn new(greaterThan__678: Option<f64>, lessThan__679: Option<f64>, greaterThanOrEqual__680: Option<f64>, lessThanOrEqual__681: Option<f64>, equalTo__682: Option<f64>) -> NumberValidationOpts {
        let greater_than;
        let less_than;
        let greater_than_or_equal;
        let less_than_or_equal;
        let equal_to;
        greater_than = greaterThan__678;
        less_than = lessThan__679;
        greater_than_or_equal = greaterThanOrEqual__680;
        less_than_or_equal = lessThanOrEqual__681;
        equal_to = equalTo__682;
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
    fn cast(& self, allowedFields__692: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__695: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__698: SafeIdentifier, min__699: i32, max__700: i32) -> Changeset;
    fn validate_int(& self, field__703: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__706: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__709: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__712: SafeIdentifier) -> Changeset;
    fn put_change(& self, field__715: SafeIdentifier, value__716: std::sync::Arc<String>) -> Changeset;
    fn get_change(& self, field__719: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>>;
    fn delete_change(& self, field__722: SafeIdentifier) -> Changeset;
    fn validate_inclusion(& self, field__725: SafeIdentifier, allowed__726: temper_core::List<std::sync::Arc<String>>) -> Changeset;
    fn validate_exclusion(& self, field__729: SafeIdentifier, disallowed__730: temper_core::List<std::sync::Arc<String>>) -> Changeset;
    fn validate_number(& self, field__733: SafeIdentifier, opts__734: NumberValidationOpts) -> Changeset;
    fn validate_acceptance(& self, field__737: SafeIdentifier) -> Changeset;
    fn validate_confirmation(& self, field__740: SafeIdentifier, confirmationField__741: SafeIdentifier) -> Changeset;
    fn validate_contains(& self, field__744: SafeIdentifier, substring__745: std::sync::Arc<String>) -> Changeset;
    fn validate_starts_with(& self, field__748: SafeIdentifier, prefix__749: std::sync::Arc<String>) -> Changeset;
    fn validate_ends_with(& self, field__752: SafeIdentifier, suffix__753: std::sync::Arc<String>) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__758: i32) -> temper_core::Result<SqlFragment>;
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
    pub fn cast(& self, allowedFields__774: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__774 = allowedFields__774.to_list();
        let mb__776: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__254: ChangesetImpl, mb__776: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__15462(& self, f__777: SafeIdentifier) {
                let mut t___15460: std::sync::Arc<String>;
                let mut t___15457: std::sync::Arc<String> = f__777.sql_value();
                let val__778: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__254.0.params, t___15457.clone(), std::sync::Arc::new("".to_string()));
                if ! val__778.is_empty() {
                    t___15460 = f__777.sql_value();
                    temper_core::MapBuilder::set( & self.mb__776, t___15460.clone(), val__778.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__254: self.clone(), mb__776: mb__776.clone()
        };
        let fn__15462 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__777: SafeIdentifier | closure_group.fn__15462(f__777))
        };
        temper_core::listed::list_for_each( & allowedFields__774, & ( * fn__15462.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__776), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__780: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__780 = fields__780.to_list();
        let return__422: Changeset;
        let mut t___15455: temper_core::List<ChangesetError>;
        let mut t___8795: TableDef;
        let mut t___8796: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8797: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__781: {
            if ! self.0.is_valid {
                return__422 = Changeset::new(self.clone());
                break 'fn__781;
            }
            let eb__782: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__783: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__255: ChangesetImpl, eb__782: temper_core::ListBuilder<ChangesetError>, valid__783: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__15451(& self, f__784: SafeIdentifier) {
                    let mut t___15449: ChangesetError;
                    let mut t___15446: std::sync::Arc<String> = f__784.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__255.0.changes, t___15446.clone()) {
                        t___15449 = ChangesetError::new(f__784.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__782, t___15449.clone(), None);
                        {
                            * self.valid__783.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__255: self.clone(), eb__782: eb__782.clone(), valid__783: valid__783.clone()
            };
            let fn__15451 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__784: SafeIdentifier | closure_group.fn__15451(f__784))
            };
            temper_core::listed::list_for_each( & fields__780, & ( * fn__15451.clone()));
            t___8795 = self.0.table_def.clone();
            t___8796 = self.0.params.clone();
            t___8797 = self.0.changes.clone();
            t___15455 = temper_core::ListedTrait::to_list( & eb__782);
            return__422 = Changeset::new(ChangesetImpl::new(t___8795.clone(), t___8796.clone(), t___8797.clone(), t___15455.clone(), temper_core::read_locked( & valid__783)));
        }
        return return__422.clone();
    }
    pub fn validate_length(& self, field__786: SafeIdentifier, min__787: i32, max__788: i32) -> Changeset {
        let return__423: Changeset;
        let mut t___15433: std::sync::Arc<String>;
        let mut t___15444: temper_core::List<ChangesetError>;
        let mut t___8778: bool;
        let mut t___8784: TableDef;
        let mut t___8785: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8786: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__789: {
            if ! self.0.is_valid {
                return__423 = Changeset::new(self.clone());
                break 'fn__789;
            }
            t___15433 = field__786.sql_value();
            let val__790: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15433.clone(), std::sync::Arc::new("".to_string()));
            let len__791: i32 = temper_core::string::count_between( & val__790, 0usize, val__790.len());
            if Some(len__791) < Some(min__787) {
                t___8778 = true;
            } else {
                t___8778 = Some(len__791) > Some(max__788);
            }
            if t___8778 {
                let msg__792: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__787, max__788));
                let eb__793: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__793, ChangesetError::new(field__786.sql_value(), msg__792.clone()), None);
                t___8784 = self.0.table_def.clone();
                t___8785 = self.0.params.clone();
                t___8786 = self.0.changes.clone();
                t___15444 = temper_core::ListedTrait::to_list( & eb__793);
                return__423 = Changeset::new(ChangesetImpl::new(t___8784.clone(), t___8785.clone(), t___8786.clone(), t___15444.clone(), false));
                break 'fn__789;
            }
            return__423 = Changeset::new(self.clone());
        }
        return return__423.clone();
    }
    pub fn validate_int(& self, field__795: SafeIdentifier) -> Changeset {
        let return__424: Changeset;
        let mut t___15424: std::sync::Arc<String>;
        let mut t___15431: temper_core::List<ChangesetError>;
        let mut t___8769: TableDef;
        let mut t___8770: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8771: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__796: {
            if ! self.0.is_valid {
                return__424 = Changeset::new(self.clone());
                break 'fn__796;
            }
            t___15424 = field__795.sql_value();
            let val__797: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15424.clone(), std::sync::Arc::new("".to_string()));
            if val__797.is_empty() {
                return__424 = Changeset::new(self.clone());
                break 'fn__796;
            }
            let parseOk__798: bool;
            'ok___15572: {
                'orelse___2532: {
                    match temper_core::string::to_int( & val__797, None) {
                        Ok(x) => x,
                        _ => break 'orelse___2532
                    };
                    parseOk__798 = true;
                    break 'ok___15572;
                }
                parseOk__798 = false;
            }
            if ! parseOk__798 {
                let eb__799: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__799, ChangesetError::new(field__795.sql_value(), "must be an integer"), None);
                t___8769 = self.0.table_def.clone();
                t___8770 = self.0.params.clone();
                t___8771 = self.0.changes.clone();
                t___15431 = temper_core::ListedTrait::to_list( & eb__799);
                return__424 = Changeset::new(ChangesetImpl::new(t___8769.clone(), t___8770.clone(), t___8771.clone(), t___15431.clone(), false));
                break 'fn__796;
            }
            return__424 = Changeset::new(self.clone());
        }
        return return__424.clone();
    }
    pub fn validate_int64(& self, field__801: SafeIdentifier) -> Changeset {
        let return__425: Changeset;
        let mut t___15415: std::sync::Arc<String>;
        let mut t___15422: temper_core::List<ChangesetError>;
        let mut t___8756: TableDef;
        let mut t___8757: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8758: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__802: {
            if ! self.0.is_valid {
                return__425 = Changeset::new(self.clone());
                break 'fn__802;
            }
            t___15415 = field__801.sql_value();
            let val__803: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15415.clone(), std::sync::Arc::new("".to_string()));
            if val__803.is_empty() {
                return__425 = Changeset::new(self.clone());
                break 'fn__802;
            }
            let parseOk__804: bool;
            'ok___15574: {
                'orelse___2533: {
                    match temper_core::string::to_int64( & val__803, None) {
                        Ok(x) => x,
                        _ => break 'orelse___2533
                    };
                    parseOk__804 = true;
                    break 'ok___15574;
                }
                parseOk__804 = false;
            }
            if ! parseOk__804 {
                let eb__805: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__805, ChangesetError::new(field__801.sql_value(), "must be a 64-bit integer"), None);
                t___8756 = self.0.table_def.clone();
                t___8757 = self.0.params.clone();
                t___8758 = self.0.changes.clone();
                t___15422 = temper_core::ListedTrait::to_list( & eb__805);
                return__425 = Changeset::new(ChangesetImpl::new(t___8756.clone(), t___8757.clone(), t___8758.clone(), t___15422.clone(), false));
                break 'fn__802;
            }
            return__425 = Changeset::new(self.clone());
        }
        return return__425.clone();
    }
    pub fn validate_float(& self, field__807: SafeIdentifier) -> Changeset {
        let return__426: Changeset;
        let mut t___15406: std::sync::Arc<String>;
        let mut t___15413: temper_core::List<ChangesetError>;
        let mut t___8743: TableDef;
        let mut t___8744: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8745: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__808: {
            if ! self.0.is_valid {
                return__426 = Changeset::new(self.clone());
                break 'fn__808;
            }
            t___15406 = field__807.sql_value();
            let val__809: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15406.clone(), std::sync::Arc::new("".to_string()));
            if val__809.is_empty() {
                return__426 = Changeset::new(self.clone());
                break 'fn__808;
            }
            let parseOk__810: bool;
            'ok___15576: {
                'orelse___2534: {
                    match temper_core::string::to_float64( & val__809) {
                        Ok(x) => x,
                        _ => break 'orelse___2534
                    };
                    parseOk__810 = true;
                    break 'ok___15576;
                }
                parseOk__810 = false;
            }
            if ! parseOk__810 {
                let eb__811: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__811, ChangesetError::new(field__807.sql_value(), "must be a number"), None);
                t___8743 = self.0.table_def.clone();
                t___8744 = self.0.params.clone();
                t___8745 = self.0.changes.clone();
                t___15413 = temper_core::ListedTrait::to_list( & eb__811);
                return__426 = Changeset::new(ChangesetImpl::new(t___8743.clone(), t___8744.clone(), t___8745.clone(), t___15413.clone(), false));
                break 'fn__808;
            }
            return__426 = Changeset::new(self.clone());
        }
        return return__426.clone();
    }
    pub fn validate_bool(& self, field__813: SafeIdentifier) -> Changeset {
        let return__427: Changeset;
        let mut t___15397: std::sync::Arc<String>;
        let mut t___15404: temper_core::List<ChangesetError>;
        let mut t___8718: bool;
        let mut t___8719: bool;
        let mut t___8721: bool;
        let mut t___8722: bool;
        let mut t___8724: bool;
        let mut t___8730: TableDef;
        let mut t___8731: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8732: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__814: {
            if ! self.0.is_valid {
                return__427 = Changeset::new(self.clone());
                break 'fn__814;
            }
            t___15397 = field__813.sql_value();
            let val__815: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15397.clone(), std::sync::Arc::new("".to_string()));
            if val__815.is_empty() {
                return__427 = Changeset::new(self.clone());
                break 'fn__814;
            }
            let isTrue__816: bool;
            if Some(val__815.as_str()) == Some("true") {
                isTrue__816 = true;
            } else {
                if Some(val__815.as_str()) == Some("1") {
                    t___8719 = true;
                } else {
                    if Some(val__815.as_str()) == Some("yes") {
                        t___8718 = true;
                    } else {
                        t___8718 = Some(val__815.as_str()) == Some("on");
                    }
                    t___8719 = t___8718;
                }
                isTrue__816 = t___8719;
            }
            let isFalse__817: bool;
            if Some(val__815.as_str()) == Some("false") {
                isFalse__817 = true;
            } else {
                if Some(val__815.as_str()) == Some("0") {
                    t___8722 = true;
                } else {
                    if Some(val__815.as_str()) == Some("no") {
                        t___8721 = true;
                    } else {
                        t___8721 = Some(val__815.as_str()) == Some("off");
                    }
                    t___8722 = t___8721;
                }
                isFalse__817 = t___8722;
            }
            if ! isTrue__816 {
                t___8724 = ! isFalse__817;
            } else {
                t___8724 = false;
            }
            if t___8724 {
                let eb__818: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__818, ChangesetError::new(field__813.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___8730 = self.0.table_def.clone();
                t___8731 = self.0.params.clone();
                t___8732 = self.0.changes.clone();
                t___15404 = temper_core::ListedTrait::to_list( & eb__818);
                return__427 = Changeset::new(ChangesetImpl::new(t___8730.clone(), t___8731.clone(), t___8732.clone(), t___15404.clone(), false));
                break 'fn__814;
            }
            return__427 = Changeset::new(self.clone());
        }
        return return__427.clone();
    }
    pub fn put_change(& self, field__820: SafeIdentifier, value__821: impl temper_core::ToArcString) -> Changeset {
        let value__821 = value__821.to_arc_string();
        let mut t___15385: i32;
        let mb__823: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        let pairs__824: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        let mut i__825: i32 = 0;
        'loop___15685: loop {
            t___15385 = temper_core::ListedTrait::len( & pairs__824);
            if ! (Some(i__825) < Some(t___15385)) {
                break;
            }
            temper_core::MapBuilder::set( & mb__823, temper_core::ListedTrait::get( & pairs__824, i__825).key(), temper_core::ListedTrait::get( & pairs__824, i__825).value());
            i__825 = i__825.wrapping_add(1);
        }
        temper_core::MapBuilder::set( & mb__823, field__820.sql_value(), value__821.clone());
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__823), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn get_change(& self, field__827: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>> {
        let mut t___15379: std::sync::Arc<String> = field__827.sql_value();
        if ! temper_core::MappedTrait::has( & self.0.changes, t___15379.clone()) {
            return Err(temper_core::Error::new());
        }
        let mut t___15381: std::sync::Arc<String> = field__827.sql_value();
        return Ok(temper_core::MappedTrait::get_or( & self.0.changes, t___15381.clone(), std::sync::Arc::new("".to_string())));
    }
    pub fn delete_change(& self, field__830: SafeIdentifier) -> Changeset {
        let mut t___15366: i32;
        let mb__832: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        let pairs__833: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        let mut i__834: i32 = 0;
        'loop___15686: loop {
            t___15366 = temper_core::ListedTrait::len( & pairs__833);
            if ! (Some(i__834) < Some(t___15366)) {
                break;
            }
            if Some(temper_core::ListedTrait::get( & pairs__833, i__834).key().as_str()) != Some(field__830.sql_value().as_str()) {
                temper_core::MapBuilder::set( & mb__832, temper_core::ListedTrait::get( & pairs__833, i__834).key(), temper_core::ListedTrait::get( & pairs__833, i__834).value());
            }
            i__834 = i__834.wrapping_add(1);
        }
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__832), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_inclusion(& self, field__836: SafeIdentifier, allowed__837: impl temper_core::ToList<std::sync::Arc<String>>) -> Changeset {
        let allowed__837 = allowed__837.to_list();
        let return__431: Changeset;
        let mut t___15352: std::sync::Arc<String>;
        let mut t___15354: std::sync::Arc<String>;
        let mut t___15362: temper_core::List<ChangesetError>;
        let mut t___8680: TableDef;
        let mut t___8681: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8682: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__838: {
            if ! self.0.is_valid {
                return__431 = Changeset::new(self.clone());
                break 'fn__838;
            }
            t___15352 = field__836.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___15352.clone()) {
                return__431 = Changeset::new(self.clone());
                break 'fn__838;
            }
            t___15354 = field__836.sql_value();
            let val__839: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15354.clone(), std::sync::Arc::new("".to_string()));
            let mut found__840: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___3 {
                val__839: std::sync::Arc<String>, found__840: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___3 {
                fn fn__15351(& self, a__841: impl temper_core::ToArcString) {
                    let a__841 = a__841.to_arc_string();
                    if Some(a__841.as_str()) == Some(self.val__839.as_str()) {
                        {
                            * self.found__840.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___3 {
                val__839: val__839.clone(), found__840: found__840.clone()
            };
            let fn__15351 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | a__841: std::sync::Arc<String> | closure_group.fn__15351(a__841))
            };
            temper_core::listed::list_for_each( & allowed__837, & ( * fn__15351.clone()));
            if ! temper_core::read_locked( & found__840) {
                let eb__842: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__842, ChangesetError::new(field__836.sql_value(), "is not included in the list"), None);
                t___8680 = self.0.table_def.clone();
                t___8681 = self.0.params.clone();
                t___8682 = self.0.changes.clone();
                t___15362 = temper_core::ListedTrait::to_list( & eb__842);
                return__431 = Changeset::new(ChangesetImpl::new(t___8680.clone(), t___8681.clone(), t___8682.clone(), t___15362.clone(), false));
                break 'fn__838;
            }
            return__431 = Changeset::new(self.clone());
        }
        return return__431.clone();
    }
    pub fn validate_exclusion(& self, field__844: SafeIdentifier, disallowed__845: impl temper_core::ToList<std::sync::Arc<String>>) -> Changeset {
        let disallowed__845 = disallowed__845.to_list();
        let return__432: Changeset;
        let mut t___15339: std::sync::Arc<String>;
        let mut t___15341: std::sync::Arc<String>;
        let mut t___15349: temper_core::List<ChangesetError>;
        let mut t___8666: TableDef;
        let mut t___8667: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8668: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__846: {
            if ! self.0.is_valid {
                return__432 = Changeset::new(self.clone());
                break 'fn__846;
            }
            t___15339 = field__844.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___15339.clone()) {
                return__432 = Changeset::new(self.clone());
                break 'fn__846;
            }
            t___15341 = field__844.sql_value();
            let val__847: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15341.clone(), std::sync::Arc::new("".to_string()));
            let mut found__848: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
            #[derive(Clone)]
            struct ClosureGroup___4 {
                val__847: std::sync::Arc<String>, found__848: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___4 {
                fn fn__15338(& self, d__849: impl temper_core::ToArcString) {
                    let d__849 = d__849.to_arc_string();
                    if Some(d__849.as_str()) == Some(self.val__847.as_str()) {
                        {
                            * self.found__848.write().unwrap() = true;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___4 {
                val__847: val__847.clone(), found__848: found__848.clone()
            };
            let fn__15338 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | d__849: std::sync::Arc<String> | closure_group.fn__15338(d__849))
            };
            temper_core::listed::list_for_each( & disallowed__845, & ( * fn__15338.clone()));
            if temper_core::read_locked( & found__848) {
                let eb__850: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__850, ChangesetError::new(field__844.sql_value(), "is reserved"), None);
                t___8666 = self.0.table_def.clone();
                t___8667 = self.0.params.clone();
                t___8668 = self.0.changes.clone();
                t___15349 = temper_core::ListedTrait::to_list( & eb__850);
                return__432 = Changeset::new(ChangesetImpl::new(t___8666.clone(), t___8667.clone(), t___8668.clone(), t___15349.clone(), false));
                break 'fn__846;
            }
            return__432 = Changeset::new(self.clone());
        }
        return return__432.clone();
    }
    pub fn validate_number(& self, field__852: SafeIdentifier, opts__853: NumberValidationOpts) -> Changeset {
        let return__433: Changeset;
        let mut t___15288: std::sync::Arc<String>;
        let mut t___15290: std::sync::Arc<String>;
        let mut t___15296: temper_core::List<ChangesetError>;
        let mut t___15304: temper_core::List<ChangesetError>;
        let mut t___15312: temper_core::List<ChangesetError>;
        let mut t___15320: temper_core::List<ChangesetError>;
        let mut t___15328: temper_core::List<ChangesetError>;
        let mut t___15336: temper_core::List<ChangesetError>;
        let mut t___8599: TableDef;
        let mut t___8600: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8601: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8603: f64;
        let mut t___8612: TableDef;
        let mut t___8613: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8614: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8622: TableDef;
        let mut t___8623: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8624: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8632: TableDef;
        let mut t___8633: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8634: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8642: TableDef;
        let mut t___8643: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8644: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8652: TableDef;
        let mut t___8653: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8654: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__854: {
            if ! self.0.is_valid {
                return__433 = Changeset::new(self.clone());
                break 'fn__854;
            }
            t___15288 = field__852.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___15288.clone()) {
                return__433 = Changeset::new(self.clone());
                break 'fn__854;
            }
            t___15290 = field__852.sql_value();
            let val__855: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15290.clone(), std::sync::Arc::new("".to_string()));
            let parseOk__856: bool;
            'ok___15581: {
                'orelse___2535: {
                    match temper_core::string::to_float64( & val__855) {
                        Ok(x) => x,
                        _ => break 'orelse___2535
                    };
                    parseOk__856 = true;
                    break 'ok___15581;
                }
                parseOk__856 = false;
            }
            if ! parseOk__856 {
                let eb__857: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__857, ChangesetError::new(field__852.sql_value(), "must be a number"), None);
                t___8599 = self.0.table_def.clone();
                t___8600 = self.0.params.clone();
                t___8601 = self.0.changes.clone();
                t___15296 = temper_core::ListedTrait::to_list( & eb__857);
                return__433 = Changeset::new(ChangesetImpl::new(t___8599.clone(), t___8600.clone(), t___8601.clone(), t___15296.clone(), false));
                break 'fn__854;
            }
            let num__858: f64;
            'ok___15582: {
                'orelse___2536: {
                    t___8603 = match temper_core::string::to_float64( & val__855) {
                        Ok(x) => x,
                        _ => break 'orelse___2536
                    };
                    num__858 = t___8603;
                    break 'ok___15582;
                }
                num__858 = 0.0f64;
            }
            let gt__859: Option<f64> = opts__853.greater_than();
            if ! gt__859.is_none() {
                let gt___2610: f64 = gt__859.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__858), Some(gt___2610)) > 0) {
                    let eb__860: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__860, ChangesetError::new(field__852.sql_value(), std::sync::Arc::new(format!("must be greater than {}", temper_core::float64::to_string(gt___2610)))), None);
                    t___8612 = self.0.table_def.clone();
                    t___8613 = self.0.params.clone();
                    t___8614 = self.0.changes.clone();
                    t___15304 = temper_core::ListedTrait::to_list( & eb__860);
                    return__433 = Changeset::new(ChangesetImpl::new(t___8612.clone(), t___8613.clone(), t___8614.clone(), t___15304.clone(), false));
                    break 'fn__854;
                }
            }
            let lt__861: Option<f64> = opts__853.less_than();
            if ! lt__861.is_none() {
                let lt___2611: f64 = lt__861.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__858), Some(lt___2611)) < 0) {
                    let eb__862: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__862, ChangesetError::new(field__852.sql_value(), std::sync::Arc::new(format!("must be less than {}", temper_core::float64::to_string(lt___2611)))), None);
                    t___8622 = self.0.table_def.clone();
                    t___8623 = self.0.params.clone();
                    t___8624 = self.0.changes.clone();
                    t___15312 = temper_core::ListedTrait::to_list( & eb__862);
                    return__433 = Changeset::new(ChangesetImpl::new(t___8622.clone(), t___8623.clone(), t___8624.clone(), t___15312.clone(), false));
                    break 'fn__854;
                }
            }
            let gte__863: Option<f64> = opts__853.greater_than_or_equal();
            if ! gte__863.is_none() {
                let gte___2612: f64 = gte__863.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__858), Some(gte___2612)) >= 0) {
                    let eb__864: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__864, ChangesetError::new(field__852.sql_value(), std::sync::Arc::new(format!("must be greater than or equal to {}", temper_core::float64::to_string(gte___2612)))), None);
                    t___8632 = self.0.table_def.clone();
                    t___8633 = self.0.params.clone();
                    t___8634 = self.0.changes.clone();
                    t___15320 = temper_core::ListedTrait::to_list( & eb__864);
                    return__433 = Changeset::new(ChangesetImpl::new(t___8632.clone(), t___8633.clone(), t___8634.clone(), t___15320.clone(), false));
                    break 'fn__854;
                }
            }
            let lte__865: Option<f64> = opts__853.less_than_or_equal();
            if ! lte__865.is_none() {
                let lte___2613: f64 = lte__865.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__858), Some(lte___2613)) <= 0) {
                    let eb__866: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__866, ChangesetError::new(field__852.sql_value(), std::sync::Arc::new(format!("must be less than or equal to {}", temper_core::float64::to_string(lte___2613)))), None);
                    t___8642 = self.0.table_def.clone();
                    t___8643 = self.0.params.clone();
                    t___8644 = self.0.changes.clone();
                    t___15328 = temper_core::ListedTrait::to_list( & eb__866);
                    return__433 = Changeset::new(ChangesetImpl::new(t___8642.clone(), t___8643.clone(), t___8644.clone(), t___15328.clone(), false));
                    break 'fn__854;
                }
            }
            let eq__867: Option<f64> = opts__853.equal_to();
            if ! eq__867.is_none() {
                let eq___2614: f64 = eq__867.unwrap();
                if ! (temper_core::float64::cmp_option(Some(num__858), Some(eq___2614)) == 0) {
                    let eb__868: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                    temper_core::listed::add( & eb__868, ChangesetError::new(field__852.sql_value(), std::sync::Arc::new(format!("must be equal to {}", temper_core::float64::to_string(eq___2614)))), None);
                    t___8652 = self.0.table_def.clone();
                    t___8653 = self.0.params.clone();
                    t___8654 = self.0.changes.clone();
                    t___15336 = temper_core::ListedTrait::to_list( & eb__868);
                    return__433 = Changeset::new(ChangesetImpl::new(t___8652.clone(), t___8653.clone(), t___8654.clone(), t___15336.clone(), false));
                    break 'fn__854;
                }
            }
            return__433 = Changeset::new(self.clone());
        }
        return return__433.clone();
    }
    pub fn validate_acceptance(& self, field__870: SafeIdentifier) -> Changeset {
        let return__434: Changeset;
        let mut t___15278: std::sync::Arc<String>;
        let mut t___15280: std::sync::Arc<String>;
        let mut t___15286: temper_core::List<ChangesetError>;
        let mut t___8577: bool;
        let mut t___8578: bool;
        let mut t___8585: TableDef;
        let mut t___8586: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8587: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__871: {
            if ! self.0.is_valid {
                return__434 = Changeset::new(self.clone());
                break 'fn__871;
            }
            t___15278 = field__870.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___15278.clone()) {
                return__434 = Changeset::new(self.clone());
                break 'fn__871;
            }
            t___15280 = field__870.sql_value();
            let val__872: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15280.clone(), std::sync::Arc::new("".to_string()));
            let accepted__873: bool;
            if Some(val__872.as_str()) == Some("true") {
                accepted__873 = true;
            } else {
                if Some(val__872.as_str()) == Some("1") {
                    t___8578 = true;
                } else {
                    if Some(val__872.as_str()) == Some("yes") {
                        t___8577 = true;
                    } else {
                        t___8577 = Some(val__872.as_str()) == Some("on");
                    }
                    t___8578 = t___8577;
                }
                accepted__873 = t___8578;
            }
            if ! accepted__873 {
                let eb__874: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__874, ChangesetError::new(field__870.sql_value(), "must be accepted"), None);
                t___8585 = self.0.table_def.clone();
                t___8586 = self.0.params.clone();
                t___8587 = self.0.changes.clone();
                t___15286 = temper_core::ListedTrait::to_list( & eb__874);
                return__434 = Changeset::new(ChangesetImpl::new(t___8585.clone(), t___8586.clone(), t___8587.clone(), t___15286.clone(), false));
                break 'fn__871;
            }
            return__434 = Changeset::new(self.clone());
        }
        return return__434.clone();
    }
    pub fn validate_confirmation(& self, field__876: SafeIdentifier, confirmationField__877: SafeIdentifier) -> Changeset {
        let return__435: Changeset;
        let mut t___15266: std::sync::Arc<String>;
        let mut t___15268: std::sync::Arc<String>;
        let mut t___15270: std::sync::Arc<String>;
        let mut t___15276: temper_core::List<ChangesetError>;
        let mut t___8569: TableDef;
        let mut t___8570: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8571: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__878: {
            if ! self.0.is_valid {
                return__435 = Changeset::new(self.clone());
                break 'fn__878;
            }
            t___15266 = field__876.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___15266.clone()) {
                return__435 = Changeset::new(self.clone());
                break 'fn__878;
            }
            t___15268 = field__876.sql_value();
            let val__879: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15268.clone(), std::sync::Arc::new("".to_string()));
            t___15270 = confirmationField__877.sql_value();
            let conf__880: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15270.clone(), std::sync::Arc::new("".to_string()));
            if Some(val__879.as_str()) != Some(conf__880.as_str()) {
                let eb__881: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__881, ChangesetError::new(confirmationField__877.sql_value(), "does not match"), None);
                t___8569 = self.0.table_def.clone();
                t___8570 = self.0.params.clone();
                t___8571 = self.0.changes.clone();
                t___15276 = temper_core::ListedTrait::to_list( & eb__881);
                return__435 = Changeset::new(ChangesetImpl::new(t___8569.clone(), t___8570.clone(), t___8571.clone(), t___15276.clone(), false));
                break 'fn__878;
            }
            return__435 = Changeset::new(self.clone());
        }
        return return__435.clone();
    }
    pub fn validate_contains(& self, field__883: SafeIdentifier, substring__884: impl temper_core::ToArcString) -> Changeset {
        let substring__884 = substring__884.to_arc_string();
        let return__436: Changeset;
        let mut t___15254: std::sync::Arc<String>;
        let mut t___15256: std::sync::Arc<String>;
        let mut t___15264: temper_core::List<ChangesetError>;
        let mut t___8554: TableDef;
        let mut t___8555: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8556: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__885: {
            if ! self.0.is_valid {
                return__436 = Changeset::new(self.clone());
                break 'fn__885;
            }
            t___15254 = field__883.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___15254.clone()) {
                return__436 = Changeset::new(self.clone());
                break 'fn__885;
            }
            t___15256 = field__883.sql_value();
            let val__886: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15256.clone(), std::sync::Arc::new("".to_string()));
            if ! temper_core::string::index_of( & val__886, substring__884.clone(), None).is_some() {
                let eb__887: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__887, ChangesetError::new(field__883.sql_value(), "must contain the given substring"), None);
                t___8554 = self.0.table_def.clone();
                t___8555 = self.0.params.clone();
                t___8556 = self.0.changes.clone();
                t___15264 = temper_core::ListedTrait::to_list( & eb__887);
                return__436 = Changeset::new(ChangesetImpl::new(t___8554.clone(), t___8555.clone(), t___8556.clone(), t___15264.clone(), false));
                break 'fn__885;
            }
            return__436 = Changeset::new(self.clone());
        }
        return return__436.clone();
    }
    pub fn validate_starts_with(& self, field__889: SafeIdentifier, prefix__890: impl temper_core::ToArcString) -> Changeset {
        let prefix__890 = prefix__890.to_arc_string();
        let return__437: Changeset;
        let mut t___15241: std::sync::Arc<String>;
        let mut t___15243: std::sync::Arc<String>;
        let mut t___15247: i32;
        let mut t___15252: temper_core::List<ChangesetError>;
        let mut t___8538: TableDef;
        let mut t___8539: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8540: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__891: {
            if ! self.0.is_valid {
                return__437 = Changeset::new(self.clone());
                break 'fn__891;
            }
            t___15241 = field__889.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___15241.clone()) {
                return__437 = Changeset::new(self.clone());
                break 'fn__891;
            }
            t___15243 = field__889.sql_value();
            let val__892: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15243.clone(), std::sync::Arc::new("".to_string()));
            let idx__893: Option<usize> = temper_core::string::index_of( & val__892, prefix__890.clone(), None);
            let starts__894: bool;
            if idx__893.is_some() {
                t___15247 = temper_core::string::count_between( & val__892, 0usize, temper_core::string::cast_as_index(idx__893).unwrap());
                starts__894 = Some(t___15247) == Some(0);
            } else {
                starts__894 = false;
            }
            if ! starts__894 {
                let eb__895: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__895, ChangesetError::new(field__889.sql_value(), "must start with the given prefix"), None);
                t___8538 = self.0.table_def.clone();
                t___8539 = self.0.params.clone();
                t___8540 = self.0.changes.clone();
                t___15252 = temper_core::ListedTrait::to_list( & eb__895);
                return__437 = Changeset::new(ChangesetImpl::new(t___8538.clone(), t___8539.clone(), t___8540.clone(), t___15252.clone(), false));
                break 'fn__891;
            }
            return__437 = Changeset::new(self.clone());
        }
        return return__437.clone();
    }
    pub fn validate_ends_with(& self, field__897: SafeIdentifier, suffix__898: impl temper_core::ToArcString) -> Changeset {
        let suffix__898 = suffix__898.to_arc_string();
        let return__438: Changeset;
        let mut t___15213: std::sync::Arc<String>;
        let mut t___15215: std::sync::Arc<String>;
        let mut t___15220: usize;
        let mut t___15226: temper_core::List<ChangesetError>;
        let mut t___15228: usize;
        let mut t___15229: bool;
        let mut t___15233: usize;
        let mut t___15234: usize;
        let mut t___15239: temper_core::List<ChangesetError>;
        let mut t___8503: TableDef;
        let mut t___8504: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8505: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8509: bool;
        let mut t___8520: TableDef;
        let mut t___8521: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___8522: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__899: {
            if ! self.0.is_valid {
                return__438 = Changeset::new(self.clone());
                break 'fn__899;
            }
            t___15213 = field__897.sql_value();
            if ! temper_core::MappedTrait::has( & self.0.changes, t___15213.clone()) {
                return__438 = Changeset::new(self.clone());
                break 'fn__899;
            }
            t___15215 = field__897.sql_value();
            let val__900: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___15215.clone(), std::sync::Arc::new("".to_string()));
            let valLen__901: i32 = temper_core::string::count_between( & val__900, 0usize, val__900.len());
            t___15220 = suffix__898.len();
            let suffixLen__902: i32 = temper_core::string::count_between( & suffix__898, 0usize, t___15220);
            if Some(valLen__901) < Some(suffixLen__902) {
                let eb__903: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__903, ChangesetError::new(field__897.sql_value(), "must end with the given suffix"), None);
                t___8503 = self.0.table_def.clone();
                t___8504 = self.0.params.clone();
                t___8505 = self.0.changes.clone();
                t___15226 = temper_core::ListedTrait::to_list( & eb__903);
                return__438 = Changeset::new(ChangesetImpl::new(t___8503.clone(), t___8504.clone(), t___8505.clone(), t___15226.clone(), false));
                break 'fn__899;
            }
            let skipCount__904: i32 = valLen__901.wrapping_sub(suffixLen__902);
            let mut strIdx__905: usize = 0usize;
            let mut i__906: i32 = 0;
            'loop___15692: while Some(i__906) < Some(skipCount__904) {
                t___15228 = temper_core::string::next( & val__900, strIdx__905);
                strIdx__905 = t___15228;
                i__906 = i__906.wrapping_add(1);
            }
            let mut sufIdx__907: usize = 0usize;
            let mut matches__908: bool = true;
            'loop___15693: loop {
                if matches__908 {
                    t___15229 = temper_core::string::has_index( & suffix__898, sufIdx__907);
                    t___8509 = t___15229;
                } else {
                    t___8509 = false;
                }
                if ! t___8509 {
                    break;
                }
                if ! temper_core::string::has_index( & val__900, strIdx__905) {
                    matches__908 = false;
                } else {
                    if Some(temper_core::string::get( & val__900, strIdx__905)) != Some(temper_core::string::get( & suffix__898, sufIdx__907)) {
                        matches__908 = false;
                    } else {
                        t___15233 = temper_core::string::next( & val__900, strIdx__905);
                        strIdx__905 = t___15233;
                        t___15234 = temper_core::string::next( & suffix__898, sufIdx__907);
                        sufIdx__907 = t___15234;
                    }
                }
            }
            if ! matches__908 {
                let eb__909: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__909, ChangesetError::new(field__897.sql_value(), "must end with the given suffix"), None);
                t___8520 = self.0.table_def.clone();
                t___8521 = self.0.params.clone();
                t___8522 = self.0.changes.clone();
                t___15239 = temper_core::ListedTrait::to_list( & eb__909);
                return__438 = Changeset::new(ChangesetImpl::new(t___8520.clone(), t___8521.clone(), t___8522.clone(), t___15239.clone(), false));
                break 'fn__899;
            }
            return__438 = Changeset::new(self.clone());
        }
        return return__438.clone();
    }
    fn parse_bool_sql_part(& self, val__911: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__911 = val__911.to_arc_string();
        let return__439: SqlBoolean;
        let mut t___8480: bool;
        let mut t___8481: bool;
        let mut t___8482: bool;
        let mut t___8484: bool;
        let mut t___8485: bool;
        let mut t___8486: bool;
        'fn__912: {
            if Some(val__911.as_str()) == Some("true") {
                t___8482 = true;
            } else {
                if Some(val__911.as_str()) == Some("1") {
                    t___8481 = true;
                } else {
                    if Some(val__911.as_str()) == Some("yes") {
                        t___8480 = true;
                    } else {
                        t___8480 = Some(val__911.as_str()) == Some("on");
                    }
                    t___8481 = t___8480;
                }
                t___8482 = t___8481;
            }
            if t___8482 {
                return__439 = SqlBoolean::new(true);
                break 'fn__912;
            }
            if Some(val__911.as_str()) == Some("false") {
                t___8486 = true;
            } else {
                if Some(val__911.as_str()) == Some("0") {
                    t___8485 = true;
                } else {
                    if Some(val__911.as_str()) == Some("no") {
                        t___8484 = true;
                    } else {
                        t___8484 = Some(val__911.as_str()) == Some("off");
                    }
                    t___8485 = t___8484;
                }
                t___8486 = t___8485;
            }
            if t___8486 {
                return__439 = SqlBoolean::new(false);
                break 'fn__912;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__439.clone());
    }
    fn value_to_sql_part(& self, fieldDef__914: FieldDef, val__915: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__915 = val__915.to_arc_string();
        let return__440: SqlPart;
        let mut t___8467: i32;
        let mut t___8470: i64;
        let mut t___8473: f64;
        let mut t___8478: temper_std::temporal::Date;
        'fn__916: {
            let ft__917: FieldType = fieldDef__914.field_type();
            if temper_core::is::<StringField>(ft__917.clone()) {
                return__440 = SqlPart::new(SqlString::new(val__915.clone()));
                break 'fn__916;
            }
            if temper_core::is::<IntField>(ft__917.clone()) {
                t___8467 = temper_core::string::to_int( & val__915, None) ? ;
                return__440 = SqlPart::new(SqlInt32::new(t___8467));
                break 'fn__916;
            }
            if temper_core::is::<Int64Field>(ft__917.clone()) {
                t___8470 = temper_core::string::to_int64( & val__915, None) ? ;
                return__440 = SqlPart::new(SqlInt64::new(t___8470));
                break 'fn__916;
            }
            if temper_core::is::<FloatField>(ft__917.clone()) {
                t___8473 = temper_core::string::to_float64( & val__915) ? ;
                return__440 = SqlPart::new(SqlFloat64::new(t___8473));
                break 'fn__916;
            }
            if temper_core::is::<BoolField>(ft__917.clone()) {
                return__440 = SqlPart::new(self.parse_bool_sql_part(val__915.clone()) ? );
                break 'fn__916;
            }
            if temper_core::is::<DateField>(ft__917.clone()) {
                t___8478 = temper_std::temporal::Date::from_iso_string(val__915.clone()) ? ;
                return__440 = SqlPart::new(SqlDate::new(t___8478.clone()));
                break 'fn__916;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__440.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___15145: i32;
        let mut t___15152: std::sync::Arc<String>;
        let mut t___15157: i32;
        let mut t___15159: std::sync::Arc<String>;
        let mut t___15164: std::sync::Arc<String>;
        let mut t___15167: i32;
        let mut t___15173: std::sync::Arc<String>;
        let mut t___15193: i32;
        let mut t___8417: bool;
        let mut t___8418: bool;
        let mut t___8425: FieldDef;
        let mut t___8431: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__920: i32 = 0;
        'loop___15694: loop {
            'continue___15556: {
                t___15145 = temper_core::ListedTrait::len( & self.0.table_def.fields());
                if ! (Some(i__920) < Some(t___15145)) {
                    break 'loop___15694;
                }
                let f__921: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__920);
                if f__921.r#virtual() {
                    break 'continue___15556;
                }
                let dv__922: Option<SqlPart> = f__921.default_value();
                if ! f__921.nullable() {
                    t___15152 = f__921.name().sql_value();
                    if ! temper_core::MappedTrait::has( & self.0.changes, t___15152.clone()) {
                        t___8417 = dv__922.is_none();
                    } else {
                        t___8417 = false;
                    }
                    t___8418 = t___8417;
                } else {
                    t___8418 = false;
                }
                if t___8418 {
                    return Err(temper_core::Error::new());
                }
            }
            i__920 = i__920.wrapping_add(1);
        }
        let colNames__923: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__924: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let pairs__925: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        let mut i__926: i32 = 0;
        'loop___15696: loop {
            'continue___15557: {
                t___15157 = temper_core::ListedTrait::len( & pairs__925);
                if ! (Some(i__926) < Some(t___15157)) {
                    break 'loop___15696;
                }
                let pair__927: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__925, i__926);
                t___15159 = pair__927.key();
                t___8425 = self.0.table_def.field(t___15159.clone()) ? ;
                let fd__928: FieldDef = t___8425.clone();
                if fd__928.r#virtual() {
                    break 'continue___15557;
                }
                temper_core::listed::add( & colNames__923, fd__928.name().sql_value(), None);
                t___15164 = pair__927.value();
                t___8431 = self.value_to_sql_part(fd__928.clone(), t___15164.clone()) ? ;
                temper_core::listed::add( & valParts__924, t___8431.clone(), None);
            }
            i__926 = i__926.wrapping_add(1);
        }
        let mut i__929: i32 = 0;
        'loop___15697: loop {
            'continue___15558: {
                t___15167 = temper_core::ListedTrait::len( & self.0.table_def.fields());
                if ! (Some(i__929) < Some(t___15167)) {
                    break 'loop___15697;
                }
                let f__930: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__929);
                if f__930.r#virtual() {
                    break 'continue___15558;
                }
                let dv__931: Option<SqlPart> = f__930.default_value();
                if ! dv__931.is_none() {
                    let dv___2622: SqlPart = dv__931.clone().unwrap();
                    t___15173 = f__930.name().sql_value();
                    if ! temper_core::MappedTrait::has( & self.0.changes, t___15173.clone()) {
                        temper_core::listed::add( & colNames__923, f__930.name().sql_value(), None);
                        temper_core::listed::add( & valParts__924, dv___2622.clone(), None);
                    }
                }
            }
            i__929 = i__929.wrapping_add(1);
        }
        if Some(temper_core::ListedTrait::len( & valParts__924)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__932: SqlBuilder = SqlBuilder::new();
        b__932.append_safe("INSERT INTO ");
        b__932.append_safe(self.0.table_def.table_name().sql_value());
        b__932.append_safe(" (");
        let mut t___15186: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__923);
        #[derive(Clone)]
        struct ClosureGroup___5 {}
        impl ClosureGroup___5 {
            fn fn__15143(& self, c__933: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__933 = c__933.to_arc_string();
                return c__933.clone();
            }
        }
        let closure_group = ClosureGroup___5 {};
        let fn__15143 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__933: std::sync::Arc<String> | closure_group.fn__15143(c__933))
        };
        b__932.append_safe(temper_core::listed::join( & t___15186, std::sync::Arc::new(", ".to_string()), & ( * fn__15143.clone())));
        b__932.append_safe(") VALUES (");
        b__932.append_part(temper_core::ListedTrait::get( & valParts__924, 0));
        let mut j__934: i32 = 1;
        'loop___15699: loop {
            t___15193 = temper_core::ListedTrait::len( & valParts__924);
            if ! (Some(j__934) < Some(t___15193)) {
                break;
            }
            b__932.append_safe(", ");
            b__932.append_part(temper_core::ListedTrait::get( & valParts__924, j__934));
            j__934 = j__934.wrapping_add(1);
        }
        b__932.append_safe(")");
        return Ok(b__932.accumulated());
    }
    pub fn to_update_sql(& self, id__936: i32) -> temper_core::Result<SqlFragment> {
        let mut t___15126: i32;
        let mut t___15128: std::sync::Arc<String>;
        let mut t___15135: std::sync::Arc<String>;
        let mut t___8392: FieldDef;
        let mut t___8399: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__938: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__938)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__939: SqlBuilder = SqlBuilder::new();
        b__939.append_safe("UPDATE ");
        b__939.append_safe(self.0.table_def.table_name().sql_value());
        b__939.append_safe(" SET ");
        let mut setCount__940: i32 = 0;
        let mut i__941: i32 = 0;
        'loop___15700: loop {
            'continue___15559: {
                t___15126 = temper_core::ListedTrait::len( & pairs__938);
                if ! (Some(i__941) < Some(t___15126)) {
                    break 'loop___15700;
                }
                let pair__942: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__938, i__941);
                t___15128 = pair__942.key();
                t___8392 = self.0.table_def.field(t___15128.clone()) ? ;
                let fd__943: FieldDef = t___8392.clone();
                if fd__943.r#virtual() {
                    break 'continue___15559;
                }
                if Some(setCount__940) > Some(0) {
                    b__939.append_safe(", ");
                }
                b__939.append_safe(fd__943.name().sql_value());
                b__939.append_safe(" = ");
                t___15135 = pair__942.value();
                t___8399 = self.value_to_sql_part(fd__943.clone(), t___15135.clone()) ? ;
                b__939.append_part(t___8399.clone());
                setCount__940 = setCount__940.wrapping_add(1);
            }
            i__941 = i__941.wrapping_add(1);
        }
        if Some(setCount__940) == Some(0) {
            return Err(temper_core::Error::new());
        }
        b__939.append_safe(" WHERE ");
        b__939.append_safe(self.0.table_def.pk_name());
        b__939.append_safe(" = ");
        b__939.append_int32(id__936);
        return Ok(b__939.accumulated());
    }
    pub fn new(_tableDef__945: TableDef, _params__946: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__947: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__948: impl temper_core::ToList<ChangesetError>, _isValid__949: bool) -> ChangesetImpl {
        let _errors__948 = _errors__948.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__945.clone();
        params = _params__946.clone();
        changes = _changes__947.clone();
        errors = _errors__948.clone();
        is_valid = _isValid__949;
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
    fn cast(& self, allowedFields__774: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__774)
    }
    fn validate_required(& self, fields__780: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__780)
    }
    fn validate_length(& self, field__786: SafeIdentifier, min__787: i32, max__788: i32) -> Changeset {
        self.validate_length(field__786, min__787, max__788)
    }
    fn validate_int(& self, field__795: SafeIdentifier) -> Changeset {
        self.validate_int(field__795)
    }
    fn validate_int64(& self, field__801: SafeIdentifier) -> Changeset {
        self.validate_int64(field__801)
    }
    fn validate_float(& self, field__807: SafeIdentifier) -> Changeset {
        self.validate_float(field__807)
    }
    fn validate_bool(& self, field__813: SafeIdentifier) -> Changeset {
        self.validate_bool(field__813)
    }
    fn put_change(& self, field__820: SafeIdentifier, value__821: std::sync::Arc<String>) -> Changeset {
        self.put_change(field__820, value__821)
    }
    fn get_change(& self, field__827: SafeIdentifier) -> temper_core::Result<std::sync::Arc<String>> {
        self.get_change(field__827)
    }
    fn delete_change(& self, field__830: SafeIdentifier) -> Changeset {
        self.delete_change(field__830)
    }
    fn validate_inclusion(& self, field__836: SafeIdentifier, allowed__837: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        self.validate_inclusion(field__836, allowed__837)
    }
    fn validate_exclusion(& self, field__844: SafeIdentifier, disallowed__845: temper_core::List<std::sync::Arc<String>>) -> Changeset {
        self.validate_exclusion(field__844, disallowed__845)
    }
    fn validate_number(& self, field__852: SafeIdentifier, opts__853: NumberValidationOpts) -> Changeset {
        self.validate_number(field__852, opts__853)
    }
    fn validate_acceptance(& self, field__870: SafeIdentifier) -> Changeset {
        self.validate_acceptance(field__870)
    }
    fn validate_confirmation(& self, field__876: SafeIdentifier, confirmationField__877: SafeIdentifier) -> Changeset {
        self.validate_confirmation(field__876, confirmationField__877)
    }
    fn validate_contains(& self, field__883: SafeIdentifier, substring__884: std::sync::Arc<String>) -> Changeset {
        self.validate_contains(field__883, substring__884)
    }
    fn validate_starts_with(& self, field__889: SafeIdentifier, prefix__890: std::sync::Arc<String>) -> Changeset {
        self.validate_starts_with(field__889, prefix__890)
    }
    fn validate_ends_with(& self, field__897: SafeIdentifier, suffix__898: std::sync::Arc<String>) -> Changeset {
        self.validate_ends_with(field__897, suffix__898)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__936: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__936)
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
    pub fn new(joinType__1218: JoinType, table__1219: SafeIdentifier, onCondition__1220: Option<SqlFragment>) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__1218.clone();
        table = table__1219.clone();
        on_condition = onCondition__1220.clone();
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
    pub fn new(field__1233: SafeIdentifier, ascending__1234: bool, nullsPos__1235: Option<NullsPosition>) -> OrderClause {
        let field;
        let ascending;
        let nulls_pos;
        field = field__1233.clone();
        ascending = ascending__1234;
        nulls_pos = nullsPos__1235.clone();
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
    pub fn new(_condition__1254: SqlFragment) -> AndCondition {
        let condition;
        condition = _condition__1254.clone();
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
    pub fn new(_condition__1261: SqlFragment) -> OrCondition {
        let condition;
        condition = _condition__1261.clone();
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
    pub fn r#where(& self, condition__1275: SqlFragment) -> Query {
        let nb__1277: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1277, WhereClause::new(AndCondition::new(condition__1275.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1277), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn or_where(& self, condition__1279: SqlFragment) -> Query {
        let nb__1281: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1281, WhereClause::new(OrCondition::new(condition__1279.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1281), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn where_null(& self, field__1283: SafeIdentifier) -> Query {
        let b__1285: SqlBuilder = SqlBuilder::new();
        b__1285.append_safe(field__1283.sql_value());
        b__1285.append_safe(" IS NULL");
        let mut t___14044: SqlFragment = b__1285.accumulated();
        return self.r#where(t___14044.clone());
    }
    pub fn where_not_null(& self, field__1287: SafeIdentifier) -> Query {
        let b__1289: SqlBuilder = SqlBuilder::new();
        b__1289.append_safe(field__1287.sql_value());
        b__1289.append_safe(" IS NOT NULL");
        let mut t___14038: SqlFragment = b__1289.accumulated();
        return self.r#where(t___14038.clone());
    }
    pub fn where_in(& self, field__1291: SafeIdentifier, values__1292: impl temper_core::ToList<SqlPart>) -> Query {
        let values__1292 = values__1292.to_list();
        let return__512: Query;
        let mut t___14019: SqlFragment;
        let mut t___14027: i32;
        let mut t___14032: SqlFragment;
        'fn__1293: {
            if temper_core::ListedTrait::is_empty( & values__1292) {
                let b__1294: SqlBuilder = SqlBuilder::new();
                b__1294.append_safe("1 = 0");
                t___14019 = b__1294.accumulated();
                return__512 = self.r#where(t___14019.clone());
                break 'fn__1293;
            }
            let b__1295: SqlBuilder = SqlBuilder::new();
            b__1295.append_safe(field__1291.sql_value());
            b__1295.append_safe(" IN (");
            b__1295.append_part(temper_core::ListedTrait::get( & values__1292, 0));
            let mut i__1296: i32 = 1;
            'loop___15711: loop {
                t___14027 = temper_core::ListedTrait::len( & values__1292);
                if ! (Some(i__1296) < Some(t___14027)) {
                    break;
                }
                b__1295.append_safe(", ");
                b__1295.append_part(temper_core::ListedTrait::get( & values__1292, i__1296));
                i__1296 = i__1296.wrapping_add(1);
            }
            b__1295.append_safe(")");
            t___14032 = b__1295.accumulated();
            return__512 = self.r#where(t___14032.clone());
        }
        return return__512.clone();
    }
    pub fn where_in_subquery(& self, field__1298: SafeIdentifier, sub__1299: Query) -> Query {
        let b__1301: SqlBuilder = SqlBuilder::new();
        b__1301.append_safe(field__1298.sql_value());
        b__1301.append_safe(" IN (");
        b__1301.append_fragment(sub__1299.to_sql());
        b__1301.append_safe(")");
        let mut t___14014: SqlFragment = b__1301.accumulated();
        return self.r#where(t___14014.clone());
    }
    pub fn where_not(& self, condition__1303: SqlFragment) -> Query {
        let b__1305: SqlBuilder = SqlBuilder::new();
        b__1305.append_safe("NOT (");
        b__1305.append_fragment(condition__1303.clone());
        b__1305.append_safe(")");
        let mut t___14005: SqlFragment = b__1305.accumulated();
        return self.r#where(t___14005.clone());
    }
    pub fn where_between(& self, field__1307: SafeIdentifier, low__1308: SqlPart, high__1309: SqlPart) -> Query {
        let b__1311: SqlBuilder = SqlBuilder::new();
        b__1311.append_safe(field__1307.sql_value());
        b__1311.append_safe(" BETWEEN ");
        b__1311.append_part(low__1308.clone());
        b__1311.append_safe(" AND ");
        b__1311.append_part(high__1309.clone());
        let mut t___13999: SqlFragment = b__1311.accumulated();
        return self.r#where(t___13999.clone());
    }
    pub fn where_like(& self, field__1313: SafeIdentifier, pattern__1314: impl temper_core::ToArcString) -> Query {
        let pattern__1314 = pattern__1314.to_arc_string();
        let b__1316: SqlBuilder = SqlBuilder::new();
        b__1316.append_safe(field__1313.sql_value());
        b__1316.append_safe(" LIKE ");
        b__1316.append_string(pattern__1314.clone());
        let mut t___13990: SqlFragment = b__1316.accumulated();
        return self.r#where(t___13990.clone());
    }
    pub fn where_i_like(& self, field__1318: SafeIdentifier, pattern__1319: impl temper_core::ToArcString) -> Query {
        let pattern__1319 = pattern__1319.to_arc_string();
        let b__1321: SqlBuilder = SqlBuilder::new();
        b__1321.append_safe(field__1318.sql_value());
        b__1321.append_safe(" ILIKE ");
        b__1321.append_string(pattern__1319.clone());
        let mut t___13983: SqlFragment = b__1321.accumulated();
        return self.r#where(t___13983.clone());
    }
    pub fn select(& self, fields__1323: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__1323 = fields__1323.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__1323.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn select_expr(& self, exprs__1326: impl temper_core::ToList<SqlFragment>) -> Query {
        let exprs__1326 = exprs__1326.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, exprs__1326.clone(), self.0.lock_mode.clone());
    }
    pub fn order_by(& self, field__1329: SafeIdentifier, ascending__1330: bool) -> Query {
        let nb__1332: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__1332, OrderClause::new(field__1329.clone(), ascending__1330, None), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__1332), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn order_by_nulls(& self, field__1334: SafeIdentifier, ascending__1335: bool, nulls__1336: NullsPosition) -> Query {
        let nb__1338: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__1338, OrderClause::new(field__1334.clone(), ascending__1335, Some(nulls__1336.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__1338), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn limit(& self, n__1340: i32) -> temper_core::Result<Query> {
        if Some(n__1340) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__1340), self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone()));
    }
    pub fn offset(& self, n__1343: i32) -> temper_core::Result<Query> {
        if Some(n__1343) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__1343), self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone()));
    }
    pub fn join(& self, joinType__1346: JoinType, table__1347: SafeIdentifier, onCondition__1348: SqlFragment) -> Query {
        let nb__1350: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__1350, JoinClause::new(joinType__1346.clone(), table__1347.clone(), Some(onCondition__1348.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__1350), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn inner_join(& self, table__1352: SafeIdentifier, onCondition__1353: SqlFragment) -> Query {
        let mut t___13945: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___13945.clone()), table__1352.clone(), onCondition__1353.clone());
    }
    pub fn left_join(& self, table__1356: SafeIdentifier, onCondition__1357: SqlFragment) -> Query {
        let mut t___13943: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___13943.clone()), table__1356.clone(), onCondition__1357.clone());
    }
    pub fn right_join(& self, table__1360: SafeIdentifier, onCondition__1361: SqlFragment) -> Query {
        let mut t___13941: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___13941.clone()), table__1360.clone(), onCondition__1361.clone());
    }
    pub fn full_join(& self, table__1364: SafeIdentifier, onCondition__1365: SqlFragment) -> Query {
        let mut t___13939: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___13939.clone()), table__1364.clone(), onCondition__1365.clone());
    }
    pub fn cross_join(& self, table__1368: SafeIdentifier) -> Query {
        let nb__1370: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__1370, JoinClause::new(JoinType::new(CrossJoin::new()), table__1368.clone(), None), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__1370), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn group_by(& self, field__1372: SafeIdentifier) -> Query {
        let nb__1374: temper_core::ListBuilder<SafeIdentifier> = temper_core::ListedTrait::to_list_builder( & self.0.group_by_fields);
        temper_core::listed::add( & nb__1374, field__1372.clone(), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1374), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn having(& self, condition__1376: SqlFragment) -> Query {
        let nb__1378: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__1378, WhereClause::new(AndCondition::new(condition__1376.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__1378), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn or_having(& self, condition__1380: SqlFragment) -> Query {
        let nb__1382: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__1382, WhereClause::new(OrCondition::new(condition__1380.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__1382), self.0.is_distinct, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn distinct(& self) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), true, self.0.select_exprs.clone(), self.0.lock_mode.clone());
    }
    pub fn lock(& self, mode__1386: LockMode) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone(), Some(mode__1386.clone()));
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___13830: i32;
        let mut t___13849: i32;
        let mut t___13868: i32;
        let b__1390: SqlBuilder = SqlBuilder::new();
        if self.0.is_distinct {
            b__1390.append_safe("SELECT DISTINCT ");
        } else {
            b__1390.append_safe("SELECT ");
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.select_exprs) {
            b__1390.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, 0));
            let mut i__1391: i32 = 1;
            'loop___15748: loop {
                t___13830 = temper_core::ListedTrait::len( & self.0.select_exprs);
                if ! (Some(i__1391) < Some(t___13830)) {
                    break;
                }
                b__1390.append_safe(", ");
                b__1390.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, i__1391));
                i__1391 = i__1391.wrapping_add(1);
            }
        } else {
            if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
                b__1390.append_safe("*");
            } else {
                #[derive(Clone)]
                struct ClosureGroup___6 {}
                impl ClosureGroup___6 {
                    fn fn__13823(& self, f__1392: SafeIdentifier) -> std::sync::Arc<String> {
                        return f__1392.sql_value();
                    }
                }
                let closure_group = ClosureGroup___6 {};
                let fn__13823 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | f__1392: SafeIdentifier | closure_group.fn__13823(f__1392))
                };
                b__1390.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__13823.clone())));
            }
        }
        b__1390.append_safe(" FROM ");
        b__1390.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___7 {
            b__1390: SqlBuilder
        }
        impl ClosureGroup___7 {
            fn fn__13822(& self, jc__1393: JoinClause) {
                self.b__1390.append_safe(" ");
                let mut t___13810: std::sync::Arc<String> = jc__1393.join_type().keyword();
                self.b__1390.append_safe(t___13810.clone());
                self.b__1390.append_safe(" ");
                let mut t___13814: std::sync::Arc<String> = jc__1393.table().sql_value();
                self.b__1390.append_safe(t___13814.clone());
                let oc__1394: Option<SqlFragment> = jc__1393.on_condition();
                if ! oc__1394.is_none() {
                    let oc___2623: SqlFragment = oc__1394.clone().unwrap();
                    self.b__1390.append_safe(" ON ");
                    self.b__1390.append_fragment(oc___2623.clone());
                }
            }
        }
        let closure_group = ClosureGroup___7 {
            b__1390: b__1390.clone()
        };
        let fn__13822 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__1393: JoinClause | closure_group.fn__13822(jc__1393))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__13822.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__1390.append_safe(" WHERE ");
            b__1390.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__1395: i32 = 1;
            'loop___15750: loop {
                t___13849 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__1395) < Some(t___13849)) {
                    break;
                }
                b__1390.append_safe(" ");
                b__1390.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1395).keyword());
                b__1390.append_safe(" ");
                b__1390.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1395).condition());
                i__1395 = i__1395.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__1390.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___8 {}
            impl ClosureGroup___8 {
                fn fn__13821(& self, f__1396: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__1396.sql_value();
                }
            }
            let closure_group = ClosureGroup___8 {};
            let fn__13821 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__1396: SafeIdentifier | closure_group.fn__13821(f__1396))
            };
            b__1390.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__13821.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__1390.append_safe(" HAVING ");
            b__1390.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__1397: i32 = 1;
            'loop___15751: loop {
                t___13868 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__1397) < Some(t___13868)) {
                    break;
                }
                b__1390.append_safe(" ");
                b__1390.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__1397).keyword());
                b__1390.append_safe(" ");
                b__1390.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__1397).condition());
                i__1397 = i__1397.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__1390.append_safe(" ORDER BY ");
            let mut first__1398: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___9 {
                first__1398: std::sync::Arc<std::sync::RwLock<bool>>, b__1390: SqlBuilder
            }
            impl ClosureGroup___9 {
                fn fn__13820(& self, orc__1399: OrderClause) {
                    let mut t___13805: std::sync::Arc<String>;
                    let mut t___7240: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__1398) {
                        self.b__1390.append_safe(", ");
                    }
                    {
                        * self.first__1398.write().unwrap() = false;
                    }
                    let mut t___13800: std::sync::Arc<String> = orc__1399.field().sql_value();
                    self.b__1390.append_safe(t___13800.clone());
                    if orc__1399.ascending() {
                        t___7240 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___7240 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__1390.append_safe(t___7240.clone());
                    let np__1400: Option<NullsPosition> = orc__1399.nulls_pos();
                    if ! np__1400.is_none() {
                        t___13805 = np__1400.clone().unwrap().keyword();
                        self.b__1390.append_safe(t___13805.clone());
                    }
                }
            }
            let closure_group = ClosureGroup___9 {
                first__1398: first__1398.clone(), b__1390: b__1390.clone()
            };
            let fn__13820 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | orc__1399: OrderClause | closure_group.fn__13820(orc__1399))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__13820.clone()));
        }
        let lv__1401: Option<i32> = self.0.limit_val;
        if ! lv__1401.is_none() {
            let lv___2625: i32 = lv__1401.unwrap();
            b__1390.append_safe(" LIMIT ");
            b__1390.append_int32(lv___2625);
        }
        let ov__1402: Option<i32> = self.0.offset_val;
        if ! ov__1402.is_none() {
            let ov___2626: i32 = ov__1402.unwrap();
            b__1390.append_safe(" OFFSET ");
            b__1390.append_int32(ov___2626);
        }
        let lm__1403: Option<LockMode> = self.0.lock_mode.clone();
        if ! lm__1403.is_none() {
            b__1390.append_safe(lm__1403.clone().unwrap().keyword());
        }
        return b__1390.accumulated();
    }
    pub fn count_sql(& self) -> SqlFragment {
        let mut t___13769: i32;
        let mut t___13788: i32;
        let b__1406: SqlBuilder = SqlBuilder::new();
        b__1406.append_safe("SELECT COUNT(*) FROM ");
        b__1406.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___10 {
            b__1406: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__13757(& self, jc__1407: JoinClause) {
                self.b__1406.append_safe(" ");
                let mut t___13747: std::sync::Arc<String> = jc__1407.join_type().keyword();
                self.b__1406.append_safe(t___13747.clone());
                self.b__1406.append_safe(" ");
                let mut t___13751: std::sync::Arc<String> = jc__1407.table().sql_value();
                self.b__1406.append_safe(t___13751.clone());
                let oc2__1408: Option<SqlFragment> = jc__1407.on_condition();
                if ! oc2__1408.is_none() {
                    let oc2___2628: SqlFragment = oc2__1408.clone().unwrap();
                    self.b__1406.append_safe(" ON ");
                    self.b__1406.append_fragment(oc2___2628.clone());
                }
            }
        }
        let closure_group = ClosureGroup___10 {
            b__1406: b__1406.clone()
        };
        let fn__13757 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__1407: JoinClause | closure_group.fn__13757(jc__1407))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__13757.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__1406.append_safe(" WHERE ");
            b__1406.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__1409: i32 = 1;
            'loop___15757: loop {
                t___13769 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__1409) < Some(t___13769)) {
                    break;
                }
                b__1406.append_safe(" ");
                b__1406.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1409).keyword());
                b__1406.append_safe(" ");
                b__1406.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1409).condition());
                i__1409 = i__1409.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__1406.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___11 {}
            impl ClosureGroup___11 {
                fn fn__13756(& self, f__1410: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__1410.sql_value();
                }
            }
            let closure_group = ClosureGroup___11 {};
            let fn__13756 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__1410: SafeIdentifier | closure_group.fn__13756(f__1410))
            };
            b__1406.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__13756.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__1406.append_safe(" HAVING ");
            b__1406.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__1411: i32 = 1;
            'loop___15758: loop {
                t___13788 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__1411) < Some(t___13788)) {
                    break;
                }
                b__1406.append_safe(" ");
                b__1406.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__1411).keyword());
                b__1406.append_safe(" ");
                b__1406.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__1411).condition());
                i__1411 = i__1411.wrapping_add(1);
            }
        }
        return b__1406.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__1413: i32) -> temper_core::Result<SqlFragment> {
        let return__537: SqlFragment;
        let mut t___7190: Query;
        if Some(defaultLimit__1413) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__537 = self.to_sql();
        } else {
            t___7190 = self.limit(defaultLimit__1413) ? ;
            return__537 = t___7190.to_sql();
        }
        return Ok(return__537.clone());
    }
    pub fn new(tableName__1416: SafeIdentifier, conditions__1417: impl temper_core::ToList<WhereClause>, selectedFields__1418: impl temper_core::ToList<SafeIdentifier>, orderClauses__1419: impl temper_core::ToList<OrderClause>, limitVal__1420: Option<i32>, offsetVal__1421: Option<i32>, joinClauses__1422: impl temper_core::ToList<JoinClause>, groupByFields__1423: impl temper_core::ToList<SafeIdentifier>, havingConditions__1424: impl temper_core::ToList<WhereClause>, isDistinct__1425: bool, selectExprs__1426: impl temper_core::ToList<SqlFragment>, lockMode__1427: Option<LockMode>) -> Query {
        let conditions__1417 = conditions__1417.to_list();
        let selectedFields__1418 = selectedFields__1418.to_list();
        let orderClauses__1419 = orderClauses__1419.to_list();
        let joinClauses__1422 = joinClauses__1422.to_list();
        let groupByFields__1423 = groupByFields__1423.to_list();
        let havingConditions__1424 = havingConditions__1424.to_list();
        let selectExprs__1426 = selectExprs__1426.to_list();
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
        table_name = tableName__1416.clone();
        conditions = conditions__1417.clone();
        selected_fields = selectedFields__1418.clone();
        order_clauses = orderClauses__1419.clone();
        limit_val = limitVal__1420;
        offset_val = offsetVal__1421;
        join_clauses = joinClauses__1422.clone();
        group_by_fields = groupByFields__1423.clone();
        having_conditions = havingConditions__1424.clone();
        is_distinct = isDistinct__1425;
        select_exprs = selectExprs__1426.clone();
        lock_mode = lockMode__1427.clone();
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
    pub fn new(field__1477: SafeIdentifier, value__1478: SqlPart) -> SetClause {
        let field;
        let value;
        field = field__1477.clone();
        value = value__1478.clone();
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
    pub fn set(& self, field__1484: SafeIdentifier, value__1485: SqlPart) -> UpdateQuery {
        let nb__1487: temper_core::ListBuilder<SetClause> = temper_core::ListedTrait::to_list_builder( & self.0.set_clauses);
        temper_core::listed::add( & nb__1487, SetClause::new(field__1484.clone(), value__1485.clone()), None);
        return UpdateQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1487), self.0.conditions.clone(), self.0.limit_val);
    }
    pub fn r#where(& self, condition__1489: SqlFragment) -> UpdateQuery {
        let nb__1491: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1491, WhereClause::new(AndCondition::new(condition__1489.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1491), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1493: SqlFragment) -> UpdateQuery {
        let nb__1495: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1495, WhereClause::new(OrCondition::new(condition__1493.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__1495), self.0.limit_val);
    }
    pub fn limit(& self, n__1497: i32) -> temper_core::Result<UpdateQuery> {
        if Some(n__1497) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), self.0.conditions.clone(), Some(n__1497)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___13604: i32;
        let mut t___13618: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        if temper_core::ListedTrait::is_empty( & self.0.set_clauses) {
            return Err(temper_core::Error::new());
        }
        let b__1501: SqlBuilder = SqlBuilder::new();
        b__1501.append_safe("UPDATE ");
        b__1501.append_safe(self.0.table_name.sql_value());
        b__1501.append_safe(" SET ");
        b__1501.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, 0).field().sql_value());
        b__1501.append_safe(" = ");
        b__1501.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, 0).value());
        let mut i__1502: i32 = 1;
        'loop___15768: loop {
            t___13604 = temper_core::ListedTrait::len( & self.0.set_clauses);
            if ! (Some(i__1502) < Some(t___13604)) {
                break;
            }
            b__1501.append_safe(", ");
            b__1501.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, i__1502).field().sql_value());
            b__1501.append_safe(" = ");
            b__1501.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, i__1502).value());
            i__1502 = i__1502.wrapping_add(1);
        }
        b__1501.append_safe(" WHERE ");
        b__1501.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
        let mut i__1503: i32 = 1;
        'loop___15769: loop {
            t___13618 = temper_core::ListedTrait::len( & self.0.conditions);
            if ! (Some(i__1503) < Some(t___13618)) {
                break;
            }
            b__1501.append_safe(" ");
            b__1501.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1503).keyword());
            b__1501.append_safe(" ");
            b__1501.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1503).condition());
            i__1503 = i__1503.wrapping_add(1);
        }
        let lv__1504: Option<i32> = self.0.limit_val;
        if ! lv__1504.is_none() {
            let lv___2629: i32 = lv__1504.unwrap();
            b__1501.append_safe(" LIMIT ");
            b__1501.append_int32(lv___2629);
        }
        return Ok(b__1501.accumulated());
    }
    pub fn new(tableName__1506: SafeIdentifier, setClauses__1507: impl temper_core::ToList<SetClause>, conditions__1508: impl temper_core::ToList<WhereClause>, limitVal__1509: Option<i32>) -> UpdateQuery {
        let setClauses__1507 = setClauses__1507.to_list();
        let conditions__1508 = conditions__1508.to_list();
        let table_name;
        let set_clauses;
        let conditions;
        let limit_val;
        table_name = tableName__1506.clone();
        set_clauses = setClauses__1507.clone();
        conditions = conditions__1508.clone();
        limit_val = limitVal__1509;
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
    pub fn r#where(& self, condition__1514: SqlFragment) -> DeleteQuery {
        let nb__1516: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1516, WhereClause::new(AndCondition::new(condition__1514.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1516), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1518: SqlFragment) -> DeleteQuery {
        let nb__1520: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1520, WhereClause::new(OrCondition::new(condition__1518.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1520), self.0.limit_val);
    }
    pub fn limit(& self, n__1522: i32) -> temper_core::Result<DeleteQuery> {
        if Some(n__1522) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(DeleteQuery::new(self.0.table_name.clone(), self.0.conditions.clone(), Some(n__1522)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___13564: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        let b__1526: SqlBuilder = SqlBuilder::new();
        b__1526.append_safe("DELETE FROM ");
        b__1526.append_safe(self.0.table_name.sql_value());
        b__1526.append_safe(" WHERE ");
        b__1526.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
        let mut i__1527: i32 = 1;
        'loop___15775: loop {
            t___13564 = temper_core::ListedTrait::len( & self.0.conditions);
            if ! (Some(i__1527) < Some(t___13564)) {
                break;
            }
            b__1526.append_safe(" ");
            b__1526.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1527).keyword());
            b__1526.append_safe(" ");
            b__1526.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1527).condition());
            i__1527 = i__1527.wrapping_add(1);
        }
        let lv__1528: Option<i32> = self.0.limit_val;
        if ! lv__1528.is_none() {
            let lv___2630: i32 = lv__1528.unwrap();
            b__1526.append_safe(" LIMIT ");
            b__1526.append_int32(lv___2630);
        }
        return Ok(b__1526.accumulated());
    }
    pub fn new(tableName__1530: SafeIdentifier, conditions__1531: impl temper_core::ToList<WhereClause>, limitVal__1532: Option<i32>) -> DeleteQuery {
        let conditions__1531 = conditions__1531.to_list();
        let table_name;
        let conditions;
        let limit_val;
        table_name = tableName__1530.clone();
        conditions = conditions__1531.clone();
        limit_val = limitVal__1532;
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
    pub fn new(_value__1767: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__1767 = _value__1767.to_arc_string();
        let value;
        value = _value__1767.clone();
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
    name: SafeIdentifier, field_type: FieldType, nullable: bool, default_value: Option<SqlPart>, r#virtual: bool
}
#[derive(Clone)]
pub struct FieldDef(std::sync::Arc<FieldDefStruct>);
#[derive(Clone)]
pub struct FieldDefBuilder {
    pub name: SafeIdentifier, pub field_type: FieldType, pub nullable: bool, pub default_value: Option<SqlPart>, pub r#virtual: bool
}
impl FieldDefBuilder {
    pub fn build(self) -> FieldDef {
        FieldDef::new(self.name, self.field_type, self.nullable, self.default_value, self.r#virtual)
    }
}
impl FieldDef {
    pub fn new(name__1787: SafeIdentifier, fieldType__1788: FieldType, nullable__1789: bool, defaultValue__1790: Option<SqlPart>, virtual__1791: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        let default_value;
        let r#virtual;
        name = name__1787.clone();
        field_type = fieldType__1788.clone();
        nullable = nullable__1789;
        default_value = defaultValue__1790.clone();
        r#virtual = virtual__1791;
        let selfish = FieldDef(std::sync::Arc::new(FieldDefStruct {
                    name, field_type, nullable, default_value, r#virtual
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
    pub fn default_value(& self) -> Option<SqlPart> {
        return self.0.default_value.clone();
    }
    pub fn r#virtual(& self) -> bool {
        return self.0.r#virtual;
    }
}
temper_core::impl_any_value_trait!(FieldDef, []);
struct TableDefStruct {
    table_name: SafeIdentifier, fields: temper_core::List<FieldDef>, primary_key: Option<SafeIdentifier>
}
#[derive(Clone)]
pub struct TableDef(std::sync::Arc<TableDefStruct>);
#[derive(Clone)]
pub struct TableDefBuilder {
    pub table_name: SafeIdentifier, pub fields: temper_core::List<FieldDef>, pub primary_key: Option<SafeIdentifier>
}
impl TableDefBuilder {
    pub fn build(self) -> TableDef {
        TableDef::new(self.table_name, self.fields, self.primary_key)
    }
}
impl TableDef {
    pub fn field(& self, name__1796: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__1796 = name__1796.to_arc_string();
        let return__603: FieldDef;
        'fn__1797: {
            let this__9149: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__9150: i32 = temper_core::ListedTrait::len( & this__9149);
            let mut i__9151: i32 = 0;
            'loop___15781: while Some(i__9151) < Some(n__9150) {
                let el__9152: FieldDef = temper_core::ListedTrait::get( & this__9149, i__9151);
                i__9151 = i__9151.wrapping_add(1);
                let f__1798: FieldDef = el__9152.clone();
                if Some(f__1798.name().sql_value().as_str()) == Some(name__1796.as_str()) {
                    return__603 = f__1798.clone();
                    break 'fn__1797;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__603.clone());
    }
    pub fn pk_name(& self) -> std::sync::Arc<String> {
        let return__604: std::sync::Arc<String>;
        'fn__1800: {
            let pk__1801: Option<SafeIdentifier> = self.0.primary_key.clone();
            if ! pk__1801.is_none() {
                let pk___2609: SafeIdentifier = pk__1801.clone().unwrap();
                return__604 = pk___2609.sql_value();
                break 'fn__1800;
            }
            return__604 = std::sync::Arc::new("id".to_string());
        }
        return return__604.clone();
    }
    pub fn new(tableName__1803: SafeIdentifier, fields__1804: impl temper_core::ToList<FieldDef>, primaryKey__1805: Option<SafeIdentifier>) -> TableDef {
        let fields__1804 = fields__1804.to_list();
        let table_name;
        let fields;
        let primary_key;
        table_name = tableName__1803.clone();
        fields = fields__1804.clone();
        primary_key = primaryKey__1805.clone();
        let selfish = TableDef(std::sync::Arc::new(TableDefStruct {
                    table_name, fields, primary_key
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn fields(& self) -> temper_core::List<FieldDef> {
        return self.0.fields.clone();
    }
    pub fn primary_key(& self) -> Option<SafeIdentifier> {
        return self.0.primary_key.clone();
    }
}
temper_core::impl_any_value_trait!(TableDef, []);
struct SqlBuilderStruct {
    buffer: temper_core::ListBuilder<SqlPart>
}
#[derive(Clone)]
pub struct SqlBuilder(std::sync::Arc<SqlBuilderStruct>);
impl SqlBuilder {
    pub fn append_safe(& self, sqlSource__1840: impl temper_core::ToArcString) {
        let sqlSource__1840 = sqlSource__1840.to_arc_string();
        let mut t___15520: SqlSource = SqlSource::new(sqlSource__1840.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___15520.clone()), None);
    }
    pub fn append_fragment(& self, fragment__1843: SqlFragment) {
        let mut t___15518: temper_core::List<SqlPart> = fragment__1843.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___15518.clone()), None);
    }
    pub fn append_part(& self, part__1846: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__1846.clone(), None);
    }
    pub fn append_part_list(& self, values__1849: impl temper_core::ToList<SqlPart>) {
        let values__1849 = values__1849.to_list();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__341: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__15514(& self, x__1851: SqlPart) {
                self.this__341.append_part(x__1851.clone());
            }
        }
        let closure_group = ClosureGroup___12 {
            this__341: self.clone()
        };
        let fn__15514 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1851: SqlPart | closure_group.fn__15514(x__1851))
        };
        self.append_list(temper_core::ToListed::to_listed(values__1849.clone()), fn__15514.clone());
    }
    pub fn append_boolean(& self, value__1853: bool) {
        let mut t___15511: SqlBoolean = SqlBoolean::new(value__1853);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___15511.clone()), None);
    }
    pub fn append_boolean_list(& self, values__1856: impl temper_core::ToListed<bool>) {
        let values__1856 = values__1856.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__343: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__15508(& self, x__1858: bool) {
                self.this__343.append_boolean(x__1858);
            }
        }
        let closure_group = ClosureGroup___13 {
            this__343: self.clone()
        };
        let fn__15508 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1858: bool | closure_group.fn__15508(x__1858))
        };
        self.append_list(values__1856.clone(), fn__15508.clone());
    }
    pub fn append_date(& self, value__1860: temper_std::temporal::Date) {
        let mut t___15505: SqlDate = SqlDate::new(value__1860.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___15505.clone()), None);
    }
    pub fn append_date_list(& self, values__1863: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__1863 = values__1863.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            this__345: SqlBuilder
        }
        impl ClosureGroup___14 {
            fn fn__15502(& self, x__1865: temper_std::temporal::Date) {
                self.this__345.append_date(x__1865.clone());
            }
        }
        let closure_group = ClosureGroup___14 {
            this__345: self.clone()
        };
        let fn__15502 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1865: temper_std::temporal::Date | closure_group.fn__15502(x__1865))
        };
        self.append_list(values__1863.clone(), fn__15502.clone());
    }
    pub fn append_float64(& self, value__1867: f64) {
        let mut t___15499: SqlFloat64 = SqlFloat64::new(value__1867);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___15499.clone()), None);
    }
    pub fn append_float64_list(& self, values__1870: impl temper_core::ToListed<f64>) {
        let values__1870 = values__1870.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___15 {
            this__347: SqlBuilder
        }
        impl ClosureGroup___15 {
            fn fn__15496(& self, x__1872: f64) {
                self.this__347.append_float64(x__1872);
            }
        }
        let closure_group = ClosureGroup___15 {
            this__347: self.clone()
        };
        let fn__15496 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1872: f64 | closure_group.fn__15496(x__1872))
        };
        self.append_list(values__1870.clone(), fn__15496.clone());
    }
    pub fn append_int32(& self, value__1874: i32) {
        let mut t___15493: SqlInt32 = SqlInt32::new(value__1874);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___15493.clone()), None);
    }
    pub fn append_int32_list(& self, values__1877: impl temper_core::ToListed<i32>) {
        let values__1877 = values__1877.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___16 {
            this__349: SqlBuilder
        }
        impl ClosureGroup___16 {
            fn fn__15490(& self, x__1879: i32) {
                self.this__349.append_int32(x__1879);
            }
        }
        let closure_group = ClosureGroup___16 {
            this__349: self.clone()
        };
        let fn__15490 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1879: i32 | closure_group.fn__15490(x__1879))
        };
        self.append_list(values__1877.clone(), fn__15490.clone());
    }
    pub fn append_int64(& self, value__1881: i64) {
        let mut t___15487: SqlInt64 = SqlInt64::new(value__1881);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___15487.clone()), None);
    }
    pub fn append_int64_list(& self, values__1884: impl temper_core::ToListed<i64>) {
        let values__1884 = values__1884.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___17 {
            this__351: SqlBuilder
        }
        impl ClosureGroup___17 {
            fn fn__15484(& self, x__1886: i64) {
                self.this__351.append_int64(x__1886);
            }
        }
        let closure_group = ClosureGroup___17 {
            this__351: self.clone()
        };
        let fn__15484 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1886: i64 | closure_group.fn__15484(x__1886))
        };
        self.append_list(values__1884.clone(), fn__15484.clone());
    }
    pub fn append_string(& self, value__1888: impl temper_core::ToArcString) {
        let value__1888 = value__1888.to_arc_string();
        let mut t___15481: SqlString = SqlString::new(value__1888.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___15481.clone()), None);
    }
    pub fn append_string_list(& self, values__1891: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__1891 = values__1891.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___18 {
            this__353: SqlBuilder
        }
        impl ClosureGroup___18 {
            fn fn__15478(& self, x__1893: impl temper_core::ToArcString) {
                let x__1893 = x__1893.to_arc_string();
                self.this__353.append_string(x__1893.clone());
            }
        }
        let closure_group = ClosureGroup___18 {
            this__353: self.clone()
        };
        let fn__15478 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1893: std::sync::Arc<String> | closure_group.fn__15478(x__1893))
        };
        self.append_list(values__1891.clone(), fn__15478.clone());
    }
    fn append_list<T>(& self, values__1895: impl temper_core::ToListed<T>, appendValue__1896: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__1895 = values__1895.to_listed();
        let mut t___15473: i32;
        let mut t___15475: T;
        let mut i__1898: i32 = 0;
        'loop___15785: loop {
            t___15473 = temper_core::ListedTrait::len( & ( * values__1895));
            if ! (Some(i__1898) < Some(t___15473)) {
                break;
            }
            if Some(i__1898) > Some(0) {
                self.append_safe(", ");
            }
            t___15475 = temper_core::ListedTrait::get( & ( * values__1895), i__1898);
            appendValue__1896(t___15475.clone());
            i__1898 = i__1898.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___15470: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___15470.clone();
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
        let mut t___15544: i32;
        let builder__1910: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__1911: i32 = 0;
        'loop___15786: loop {
            t___15544 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__1911) < Some(t___15544)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__1911).format_to(builder__1910.clone());
            i__1911 = i__1911.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__1910);
    }
    pub fn new(parts__1913: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__1913 = parts__1913.to_list();
        let parts;
        parts = parts__1913.clone();
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
    SqlSource(SqlSource), SqlBoolean(SqlBoolean), SqlString(SqlString), SqlInt32(SqlInt32), SqlInt64(SqlInt64), SqlFloat64(SqlFloat64), SqlDate(SqlDate), SqlDefault(SqlDefault)
}
pub trait SqlPartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SqlPartEnum;
    fn clone_boxed(& self) -> SqlPart;
    fn format_to(& self, builder__1915: std::sync::Arc<std::sync::RwLock<String>>);
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
    pub fn format_to(& self, builder__1919: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1919, self.0.source.clone());
    }
    pub fn new(source__1922: impl temper_core::ToArcString) -> SqlSource {
        let source__1922 = source__1922.to_arc_string();
        let source;
        source = source__1922.clone();
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
    fn format_to(& self, builder__1919: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1919)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__1925: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___8850: std::sync::Arc<String>;
        if self.0.value {
            t___8850 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___8850 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__1925, t___8850.clone());
    }
    pub fn new(value__1928: bool) -> SqlBoolean {
        let value;
        value = value__1928;
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
    fn format_to(& self, builder__1925: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1925)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__1931: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1931, "'");
        let mut t___15525: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___19 {
            builder__1931: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___19 {
            fn fn__15523(& self, c__1933: i32) {
                if Some(c__1933) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1931, "''");
                } else {
                    'ok___15596: {
                        'orelse___2531: {
                            match temper_core::string::builder::append_code_point( & self.builder__1931, c__1933) {
                                Ok(x) => x,
                                _ => break 'orelse___2531
                            };
                            break 'ok___15596;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___19 {
            builder__1931: builder__1931.clone()
        };
        let fn__15523 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1933: i32 | closure_group.fn__15523(c__1933))
        };
        temper_core::string::for_each( & t___15525, & ( * fn__15523.clone()));
        temper_core::string::builder::append( & builder__1931, "'");
    }
    pub fn new(value__1935: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__1935.clone();
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
    fn format_to(& self, builder__1931: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1931)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__1938: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___8839: bool;
        let mut t___8840: bool;
        let s__1940: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__1940.as_str()) == Some("NaN") {
            t___8840 = true;
        } else {
            if Some(s__1940.as_str()) == Some("Infinity") {
                t___8839 = true;
            } else {
                t___8839 = Some(s__1940.as_str()) == Some("-Infinity");
            }
            t___8840 = t___8839;
        }
        if t___8840 {
            temper_core::string::builder::append( & builder__1938, "NULL");
        } else {
            temper_core::string::builder::append( & builder__1938, s__1940.clone());
        }
    }
    pub fn new(value__1942: f64) -> SqlFloat64 {
        let value;
        value = value__1942;
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
    fn format_to(& self, builder__1938: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1938)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__1945: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___15534: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1945, t___15534.clone());
    }
    pub fn new(value__1948: i32) -> SqlInt32 {
        let value;
        value = value__1948;
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
    fn format_to(& self, builder__1945: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1945)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__1951: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___15532: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1951, t___15532.clone());
    }
    pub fn new(value__1954: i64) -> SqlInt64 {
        let value;
        value = value__1954;
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
    fn format_to(& self, builder__1951: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1951)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlDefaultStruct {}
#[derive(Clone)]
pub struct SqlDefault(std::sync::Arc<SqlDefaultStruct>);
impl SqlDefault {
    pub fn format_to(& self, builder__1956: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1956, "DEFAULT");
    }
    pub fn new() -> SqlDefault {
        let selfish = SqlDefault(std::sync::Arc::new(SqlDefaultStruct {}));
        return selfish;
    }
}
impl SqlPartTrait for SqlDefault {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlDefault(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__1956: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1956)
    }
}
temper_core::impl_any_value_trait!(SqlDefault, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__1961: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1961, "'");
        #[derive(Clone)]
        struct ClosureGroup___20 {
            builder__1961: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___20 {
            fn fn__15537(& self, c__1963: i32) {
                if Some(c__1963) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1961, "''");
                } else {
                    'ok___15600: {
                        'orelse___2530: {
                            match temper_core::string::builder::append_code_point( & self.builder__1961, c__1963) {
                                Ok(x) => x,
                                _ => break 'orelse___2530
                            };
                            break 'ok___15600;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___20 {
            builder__1961: builder__1961.clone()
        };
        let fn__15537 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1963: i32 | closure_group.fn__15537(c__1963))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__15537.clone()));
        temper_core::string::builder::append( & builder__1961, "'");
    }
    pub fn new(value__1965: impl temper_core::ToArcString) -> SqlString {
        let value__1965 = value__1965.to_arc_string();
        let value;
        value = value__1965.clone();
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
    fn format_to(& self, builder__1961: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1961)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__950: TableDef, params__951: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___15116: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__950.clone(), params__951.clone(), t___15116.clone(), [], true));
}
fn isIdentStart__663(c__1768: i32) -> bool {
    let return__581: bool;
    let mut t___8366: bool;
    let mut t___8367: bool;
    if Some(c__1768) >= Some(97) {
        t___8366 = Some(c__1768) <= Some(122);
    } else {
        t___8366 = false;
    }
    if t___8366 {
        return__581 = true;
    } else {
        if Some(c__1768) >= Some(65) {
            t___8367 = Some(c__1768) <= Some(90);
        } else {
            t___8367 = false;
        }
        if t___8367 {
            return__581 = true;
        } else {
            return__581 = Some(c__1768) == Some(95);
        }
    }
    return return__581;
}
fn isIdentPart__664(c__1770: i32) -> bool {
    let return__582: bool;
    if isIdentStart__663(c__1770) {
        return__582 = true;
    } else {
        if Some(c__1770) >= Some(48) {
            return__582 = Some(c__1770) <= Some(57);
        } else {
            return__582 = false;
        }
    }
    return return__582;
}
pub fn safe_identifier(name__1772: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__1772 = name__1772.to_arc_string();
    let mut t___15114: usize;
    if name__1772.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__1774: usize = 0usize;
    if ! isIdentStart__663(temper_core::string::get( & name__1772, idx__1774)) {
        return Err(temper_core::Error::new());
    }
    let mut t___15111: usize = temper_core::string::next( & name__1772, idx__1774);
    idx__1774 = t___15111;
    'loop___15787: loop {
        if ! temper_core::string::has_index( & name__1772, idx__1774) {
            break;
        }
        if ! isIdentPart__664(temper_core::string::get( & name__1772, idx__1774)) {
            return Err(temper_core::Error::new());
        }
        t___15114 = temper_core::string::next( & name__1772, idx__1774);
        idx__1774 = t___15114;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__1772.clone())));
}
fn csid__660(name__953: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__953 = name__953.to_arc_string();
    let return__444: SafeIdentifier;
    let mut t___8354: SafeIdentifier;
    'ok___15602: {
        'orelse___2537: {
            t___8354 = match safe_identifier(name__953.clone()) {
                Ok(x) => x,
                _ => break 'orelse___2537
            };
            return__444 = t___8354.clone();
            break 'ok___15602;
        }
        return__444 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__444.clone();
}
fn userTable__661() -> TableDef {
    return TableDef::new(csid__660("users"), [FieldDef::new(csid__660("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("email"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("age"), FieldType::new(IntField::new()), true, None, false), FieldDef::new(csid__660("score"), FieldType::new(FloatField::new()), true, None, false), FieldDef::new(csid__660("active"), FieldType::new(BoolField::new()), true, None, false)], None);
}
pub fn timestamps() -> temper_core::Result<temper_core::List<FieldDef>> {
    let mut t___7625: SafeIdentifier;
    t___7625 = safe_identifier("inserted_at") ? ;
    let mut t___14213: FieldDef = FieldDef::new(t___7625.clone(), FieldType::new(DateField::new()), true, Some(SqlPart::new(SqlDefault::new())), false);
    let mut t___7629: SafeIdentifier;
    t___7629 = safe_identifier("updated_at") ? ;
    return Ok(std::sync::Arc::new(vec![t___14213.clone(), FieldDef::new(t___7629.clone(), FieldType::new(DateField::new()), true, Some(SqlPart::new(SqlDefault::new())), false)]));
}
pub fn delete_sql(tableDef__1193: TableDef, id__1194: i32) -> SqlFragment {
    let b__1196: SqlBuilder = SqlBuilder::new();
    b__1196.append_safe("DELETE FROM ");
    b__1196.append_safe(tableDef__1193.table_name().sql_value());
    b__1196.append_safe(" WHERE ");
    b__1196.append_safe(tableDef__1193.pk_name());
    b__1196.append_safe(" = ");
    b__1196.append_int32(id__1194);
    return b__1196.accumulated();
}
pub fn from(tableName__1428: SafeIdentifier) -> Query {
    return Query::new(tableName__1428.clone(), [], [], [], None, None, [], [], [], false, [], None);
}
pub fn col(table__1430: SafeIdentifier, column__1431: SafeIdentifier) -> SqlFragment {
    let b__1433: SqlBuilder = SqlBuilder::new();
    b__1433.append_safe(table__1430.sql_value());
    b__1433.append_safe(".");
    b__1433.append_safe(column__1431.sql_value());
    return b__1433.accumulated();
}
pub fn count_all() -> SqlFragment {
    let b__1435: SqlBuilder = SqlBuilder::new();
    b__1435.append_safe("COUNT(*)");
    return b__1435.accumulated();
}
pub fn count_col(field__1436: SafeIdentifier) -> SqlFragment {
    let b__1438: SqlBuilder = SqlBuilder::new();
    b__1438.append_safe("COUNT(");
    b__1438.append_safe(field__1436.sql_value());
    b__1438.append_safe(")");
    return b__1438.accumulated();
}
pub fn sum_col(field__1439: SafeIdentifier) -> SqlFragment {
    let b__1441: SqlBuilder = SqlBuilder::new();
    b__1441.append_safe("SUM(");
    b__1441.append_safe(field__1439.sql_value());
    b__1441.append_safe(")");
    return b__1441.accumulated();
}
pub fn avg_col(field__1442: SafeIdentifier) -> SqlFragment {
    let b__1444: SqlBuilder = SqlBuilder::new();
    b__1444.append_safe("AVG(");
    b__1444.append_safe(field__1442.sql_value());
    b__1444.append_safe(")");
    return b__1444.accumulated();
}
pub fn min_col(field__1445: SafeIdentifier) -> SqlFragment {
    let b__1447: SqlBuilder = SqlBuilder::new();
    b__1447.append_safe("MIN(");
    b__1447.append_safe(field__1445.sql_value());
    b__1447.append_safe(")");
    return b__1447.accumulated();
}
pub fn max_col(field__1448: SafeIdentifier) -> SqlFragment {
    let b__1450: SqlBuilder = SqlBuilder::new();
    b__1450.append_safe("MAX(");
    b__1450.append_safe(field__1448.sql_value());
    b__1450.append_safe(")");
    return b__1450.accumulated();
}
pub fn union_sql(a__1451: Query, b__1452: Query) -> SqlFragment {
    let sb__1454: SqlBuilder = SqlBuilder::new();
    sb__1454.append_safe("(");
    sb__1454.append_fragment(a__1451.to_sql());
    sb__1454.append_safe(") UNION (");
    sb__1454.append_fragment(b__1452.to_sql());
    sb__1454.append_safe(")");
    return sb__1454.accumulated();
}
pub fn union_all_sql(a__1455: Query, b__1456: Query) -> SqlFragment {
    let sb__1458: SqlBuilder = SqlBuilder::new();
    sb__1458.append_safe("(");
    sb__1458.append_fragment(a__1455.to_sql());
    sb__1458.append_safe(") UNION ALL (");
    sb__1458.append_fragment(b__1456.to_sql());
    sb__1458.append_safe(")");
    return sb__1458.accumulated();
}
pub fn intersect_sql(a__1459: Query, b__1460: Query) -> SqlFragment {
    let sb__1462: SqlBuilder = SqlBuilder::new();
    sb__1462.append_safe("(");
    sb__1462.append_fragment(a__1459.to_sql());
    sb__1462.append_safe(") INTERSECT (");
    sb__1462.append_fragment(b__1460.to_sql());
    sb__1462.append_safe(")");
    return sb__1462.accumulated();
}
pub fn except_sql(a__1463: Query, b__1464: Query) -> SqlFragment {
    let sb__1466: SqlBuilder = SqlBuilder::new();
    sb__1466.append_safe("(");
    sb__1466.append_fragment(a__1463.to_sql());
    sb__1466.append_safe(") EXCEPT (");
    sb__1466.append_fragment(b__1464.to_sql());
    sb__1466.append_safe(")");
    return sb__1466.accumulated();
}
pub fn subquery(q__1467: Query, alias__1468: SafeIdentifier) -> SqlFragment {
    let b__1470: SqlBuilder = SqlBuilder::new();
    b__1470.append_safe("(");
    b__1470.append_fragment(q__1467.to_sql());
    b__1470.append_safe(") AS ");
    b__1470.append_safe(alias__1468.sql_value());
    return b__1470.accumulated();
}
pub fn exists_sql(q__1471: Query) -> SqlFragment {
    let b__1473: SqlBuilder = SqlBuilder::new();
    b__1473.append_safe("EXISTS (");
    b__1473.append_fragment(q__1471.to_sql());
    b__1473.append_safe(")");
    return b__1473.accumulated();
}
pub fn update(tableName__1533: SafeIdentifier) -> UpdateQuery {
    return UpdateQuery::new(tableName__1533.clone(), [], [], None);
}
pub fn delete_from(tableName__1535: SafeIdentifier) -> DeleteQuery {
    return DeleteQuery::new(tableName__1535.clone(), [], None);
}
fn sid__662(name__1537: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__1537 = name__1537.to_arc_string();
    let return__574: SafeIdentifier;
    let mut t___6965: SafeIdentifier;
    'ok___15622: {
        'orelse___2556: {
            t___6965 = match safe_identifier(name__1537.clone()) {
                Ok(x) => x,
                _ => break 'orelse___2556
            };
            return__574 = t___6965.clone();
            break 'ok___15622;
        }
        return__574 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__574.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__2120() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__957: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___15072: TableDef = userTable__661();
        let mut t___15073: SafeIdentifier = csid__660("name");
        let mut t___15074: SafeIdentifier = csid__660("email");
        let cs__958: Changeset = changeset(t___15072.clone(), params__957.clone()).cast(std::sync::Arc::new(vec![t___15073.clone(), t___15074.clone()]));
        let mut t___15077: bool = temper_core::MappedTrait::has( & cs__958.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__15067(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__15067 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15067())
        };
        test___32.assert(t___15077, fn__15067.clone());
        let mut t___15081: bool = temper_core::MappedTrait::has( & cs__958.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__15066(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__15066 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15066())
        };
        test___32.assert(t___15081, fn__15066.clone());
        let mut t___15087: bool = ! temper_core::MappedTrait::has( & cs__958.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__15065(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__15065 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15065())
        };
        test___32.assert(t___15087, fn__15065.clone());
        let mut t___15089: bool = cs__958.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__15064(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__15064 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15064())
        };
        test___32.assert(t___15089, fn__15064.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__2121() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__960: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___15050: TableDef = userTable__661();
        let mut t___15051: SafeIdentifier = csid__660("name");
        let cs__961: Changeset = changeset(t___15050.clone(), params__960.clone()).cast(std::sync::Arc::new(vec![t___15051.clone()])).cast(std::sync::Arc::new(vec![csid__660("email")]));
        let mut t___15058: bool = ! temper_core::MappedTrait::has( & cs__961.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__15046(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__15046 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15046())
        };
        test___33.assert(t___15058, fn__15046.clone());
        let mut t___15061: bool = temper_core::MappedTrait::has( & cs__961.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__15045(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__15045 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15045())
        };
        test___33.assert(t___15061, fn__15045.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__2122() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__963: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___15032: TableDef = userTable__661();
        let mut t___15033: SafeIdentifier = csid__660("name");
        let mut t___15034: SafeIdentifier = csid__660("email");
        let cs__964: Changeset = changeset(t___15032.clone(), params__963.clone()).cast(std::sync::Arc::new(vec![t___15033.clone(), t___15034.clone()]));
        let mut t___15039: bool = ! temper_core::MappedTrait::has( & cs__964.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__15028(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__15028 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15028())
        };
        test___34.assert(t___15039, fn__15028.clone());
        let mut t___15042: bool = temper_core::MappedTrait::has( & cs__964.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__15027(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__15027 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15027())
        };
        test___34.assert(t___15042, fn__15027.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__2123() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let params__966: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___15014: TableDef = userTable__661();
        let mut t___15015: SafeIdentifier = csid__660("name");
        let cs__967: Changeset = changeset(t___15014.clone(), params__966.clone()).cast(std::sync::Arc::new(vec![t___15015.clone()])).validate_required(std::sync::Arc::new(vec![csid__660("name")]));
        let mut t___15019: bool = cs__967.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__15011(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__15011 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15011())
        };
        test___35.assert(t___15019, fn__15011.clone());
        let mut t___15025: bool = Some(temper_core::ListedTrait::len( & cs__967.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__15010(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__15010 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__15010())
        };
        test___35.assert(t___15025, fn__15010.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__2124() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let params__969: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___14990: TableDef = userTable__661();
        let mut t___14991: SafeIdentifier = csid__660("name");
        let cs__970: Changeset = changeset(t___14990.clone(), params__969.clone()).cast(std::sync::Arc::new(vec![t___14991.clone()])).validate_required(std::sync::Arc::new(vec![csid__660("name")]));
        let mut t___14997: bool = ! cs__970.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__14988(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__14988 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14988())
        };
        test___36.assert(t___14997, fn__14988.clone());
        let mut t___15002: bool = Some(temper_core::ListedTrait::len( & cs__970.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__14987(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__14987 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14987())
        };
        test___36.assert(t___15002, fn__14987.clone());
        let mut t___15008: bool = Some(temper_core::ListedTrait::get( & cs__970.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__14986(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__14986 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14986())
        };
        test___36.assert(t___15008, fn__14986.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__2125() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        let params__972: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___14978: TableDef = userTable__661();
        let mut t___14979: SafeIdentifier = csid__660("name");
        let cs__973: Changeset = changeset(t___14978.clone(), params__972.clone()).cast(std::sync::Arc::new(vec![t___14979.clone()])).validate_length(csid__660("name"), 2, 50);
        let mut t___14983: bool = cs__973.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__14975(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__14975 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14975())
        };
        test___37.assert(t___14983, fn__14975.clone());
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__2126() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        let params__975: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___14966: TableDef = userTable__661();
        let mut t___14967: SafeIdentifier = csid__660("name");
        let cs__976: Changeset = changeset(t___14966.clone(), params__975.clone()).cast(std::sync::Arc::new(vec![t___14967.clone()])).validate_length(csid__660("name"), 2, 50);
        let mut t___14973: bool = ! cs__976.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__14963(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__14963 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14963())
        };
        test___38.assert(t___14973, fn__14963.clone());
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__2127() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        let params__978: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___14954: TableDef = userTable__661();
        let mut t___14955: SafeIdentifier = csid__660("name");
        let cs__979: Changeset = changeset(t___14954.clone(), params__978.clone()).cast(std::sync::Arc::new(vec![t___14955.clone()])).validate_length(csid__660("name"), 2, 10);
        let mut t___14961: bool = ! cs__979.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__14951(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__14951 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14951())
        };
        test___39.assert(t___14961, fn__14951.clone());
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__2128() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let params__981: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___14943: TableDef = userTable__661();
        let mut t___14944: SafeIdentifier = csid__660("age");
        let cs__982: Changeset = changeset(t___14943.clone(), params__981.clone()).cast(std::sync::Arc::new(vec![t___14944.clone()])).validate_int(csid__660("age"));
        let mut t___14948: bool = cs__982.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__14940(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__14940 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14940())
        };
        test___40.assert(t___14948, fn__14940.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__2129() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let params__984: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___14931: TableDef = userTable__661();
        let mut t___14932: SafeIdentifier = csid__660("age");
        let cs__985: Changeset = changeset(t___14931.clone(), params__984.clone()).cast(std::sync::Arc::new(vec![t___14932.clone()])).validate_int(csid__660("age"));
        let mut t___14938: bool = ! cs__985.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___38 {}
        impl ClosureGroup___38 {
            fn fn__14928(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___38 {};
        let fn__14928 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14928())
        };
        test___41.assert(t___14938, fn__14928.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__2130() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let params__987: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___14920: TableDef = userTable__661();
        let mut t___14921: SafeIdentifier = csid__660("score");
        let cs__988: Changeset = changeset(t___14920.clone(), params__987.clone()).cast(std::sync::Arc::new(vec![t___14921.clone()])).validate_float(csid__660("score"));
        let mut t___14925: bool = cs__988.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__14917(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__14917 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14917())
        };
        test___42.assert(t___14925, fn__14917.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__2131() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let params__990: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___14909: TableDef = userTable__661();
        let mut t___14910: SafeIdentifier = csid__660("age");
        let cs__991: Changeset = changeset(t___14909.clone(), params__990.clone()).cast(std::sync::Arc::new(vec![t___14910.clone()])).validate_int64(csid__660("age"));
        let mut t___14914: bool = cs__991.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___40 {}
        impl ClosureGroup___40 {
            fn fn__14906(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___40 {};
        let fn__14906 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14906())
        };
        test___43.assert(t___14914, fn__14906.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__2132() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let params__993: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___14897: TableDef = userTable__661();
        let mut t___14898: SafeIdentifier = csid__660("age");
        let cs__994: Changeset = changeset(t___14897.clone(), params__993.clone()).cast(std::sync::Arc::new(vec![t___14898.clone()])).validate_int64(csid__660("age"));
        let mut t___14904: bool = ! cs__994.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___41 {}
        impl ClosureGroup___41 {
            fn fn__14894(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___41 {};
        let fn__14894 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14894())
        };
        test___44.assert(t___14904, fn__14894.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__2133() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            test___45: temper_std::testing::Test
        }
        impl ClosureGroup___42 {
            fn fn__14891(& self, v__996: impl temper_core::ToArcString) {
                let v__996 = v__996.to_arc_string();
                let params__997: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__996.clone())]);
                let mut t___14883: TableDef = userTable__661();
                let mut t___14884: SafeIdentifier = csid__660("active");
                let cs__998: Changeset = changeset(t___14883.clone(), params__997.clone()).cast(std::sync::Arc::new(vec![t___14884.clone()])).validate_bool(csid__660("active"));
                let mut t___14888: bool = cs__998.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___43 {
                    v__996: std::sync::Arc<String>
                }
                impl ClosureGroup___43 {
                    fn fn__14880(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__996.clone()));
                    }
                }
                let closure_group = ClosureGroup___43 {
                    v__996: v__996.clone()
                };
                let fn__14880 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__14880())
                };
                self.test___45.assert(t___14888, fn__14880.clone());
            }
        }
        let closure_group = ClosureGroup___42 {
            test___45: test___45.clone()
        };
        let fn__14891 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__996: std::sync::Arc<String> | closure_group.fn__14891(v__996))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__14891.clone()));
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__2134() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            test___46: temper_std::testing::Test
        }
        impl ClosureGroup___44 {
            fn fn__14877(& self, v__1000: impl temper_core::ToArcString) {
                let v__1000 = v__1000.to_arc_string();
                let params__1001: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__1000.clone())]);
                let mut t___14869: TableDef = userTable__661();
                let mut t___14870: SafeIdentifier = csid__660("active");
                let cs__1002: Changeset = changeset(t___14869.clone(), params__1001.clone()).cast(std::sync::Arc::new(vec![t___14870.clone()])).validate_bool(csid__660("active"));
                let mut t___14874: bool = cs__1002.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___45 {
                    v__1000: std::sync::Arc<String>
                }
                impl ClosureGroup___45 {
                    fn fn__14866(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__1000.clone()));
                    }
                }
                let closure_group = ClosureGroup___45 {
                    v__1000: v__1000.clone()
                };
                let fn__14866 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__14866())
                };
                self.test___46.assert(t___14874, fn__14866.clone());
            }
        }
        let closure_group = ClosureGroup___44 {
            test___46: test___46.clone()
        };
        let fn__14877 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__1000: std::sync::Arc<String> | closure_group.fn__14877(v__1000))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__14877.clone()));
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__2135() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            test___47: temper_std::testing::Test
        }
        impl ClosureGroup___46 {
            fn fn__14863(& self, v__1004: impl temper_core::ToArcString) {
                let v__1004 = v__1004.to_arc_string();
                let params__1005: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__1004.clone())]);
                let mut t___14854: TableDef = userTable__661();
                let mut t___14855: SafeIdentifier = csid__660("active");
                let cs__1006: Changeset = changeset(t___14854.clone(), params__1005.clone()).cast(std::sync::Arc::new(vec![t___14855.clone()])).validate_bool(csid__660("active"));
                let mut t___14861: bool = ! cs__1006.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___47 {
                    v__1004: std::sync::Arc<String>
                }
                impl ClosureGroup___47 {
                    fn fn__14851(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__1004.clone()));
                    }
                }
                let closure_group = ClosureGroup___47 {
                    v__1004: v__1004.clone()
                };
                let fn__14851 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__14851())
                };
                self.test___47.assert(t___14861, fn__14851.clone());
            }
        }
        let closure_group = ClosureGroup___46 {
            test___47: test___47.clone()
        };
        let fn__14863 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__1004: std::sync::Arc<String> | closure_group.fn__14863(v__1004))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__14863.clone()));
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__2136() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___8155: SqlFragment;
        let params__1008: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___14839: TableDef = userTable__661();
        let mut t___14840: SafeIdentifier = csid__660("name");
        let mut t___14841: SafeIdentifier = csid__660("email");
        let cs__1009: Changeset = changeset(t___14839.clone(), params__1008.clone()).cast(std::sync::Arc::new(vec![t___14840.clone(), t___14841.clone()])).validate_required(std::sync::Arc::new(vec![csid__660("name"), csid__660("email")]));
        let sqlFrag__1010: SqlFragment;
        'ok___15604: {
            'orelse___2538: {
                t___8155 = match cs__1009.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2538
                };
                sqlFrag__1010 = t___8155.clone();
                break 'ok___15604;
            }
            sqlFrag__1010 = panic!();
        }
        let s__1011: std::sync::Arc<String> = sqlFrag__1010.to_string();
        let mut t___14848: bool = temper_core::string::index_of( & s__1011, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___48 {
            s__1011: std::sync::Arc<String>
        }
        impl ClosureGroup___48 {
            fn fn__14835(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__1011.clone()));
            }
        }
        let closure_group = ClosureGroup___48 {
            s__1011: s__1011.clone()
        };
        let fn__14835 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14835())
        };
        test___48.assert(t___14848, fn__14835.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__2137() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___8134: SqlFragment;
        let params__1013: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___14819: TableDef = userTable__661();
        let mut t___14820: SafeIdentifier = csid__660("name");
        let mut t___14821: SafeIdentifier = csid__660("email");
        let cs__1014: Changeset = changeset(t___14819.clone(), params__1013.clone()).cast(std::sync::Arc::new(vec![t___14820.clone(), t___14821.clone()])).validate_required(std::sync::Arc::new(vec![csid__660("name"), csid__660("email")]));
        let sqlFrag__1015: SqlFragment;
        'ok___15605: {
            'orelse___2539: {
                t___8134 = match cs__1014.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2539
                };
                sqlFrag__1015 = t___8134.clone();
                break 'ok___15605;
            }
            sqlFrag__1015 = panic!();
        }
        let s__1016: std::sync::Arc<String> = sqlFrag__1015.to_string();
        let mut t___14828: bool = temper_core::string::index_of( & s__1016, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___49 {
            s__1016: std::sync::Arc<String>
        }
        impl ClosureGroup___49 {
            fn fn__14815(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__1016.clone()));
            }
        }
        let closure_group = ClosureGroup___49 {
            s__1016: s__1016.clone()
        };
        let fn__14815 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14815())
        };
        test___49.assert(t___14828, fn__14815.clone());
        let mut t___14832: bool = temper_core::string::index_of( & s__1016, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___50 {
            s__1016: std::sync::Arc<String>
        }
        impl ClosureGroup___50 {
            fn fn__14814(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__1016.clone()));
            }
        }
        let closure_group = ClosureGroup___50 {
            s__1016: s__1016.clone()
        };
        let fn__14814 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14814())
        };
        test___49.assert(t___14832, fn__14814.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__2138() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___8117: SqlFragment;
        let params__1018: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___14801: TableDef = userTable__661();
        let mut t___14802: SafeIdentifier = csid__660("name");
        let mut t___14803: SafeIdentifier = csid__660("email");
        let mut t___14804: SafeIdentifier = csid__660("age");
        let cs__1019: Changeset = changeset(t___14801.clone(), params__1018.clone()).cast(std::sync::Arc::new(vec![t___14802.clone(), t___14803.clone(), t___14804.clone()])).validate_required(std::sync::Arc::new(vec![csid__660("name"), csid__660("email")]));
        let sqlFrag__1020: SqlFragment;
        'ok___15606: {
            'orelse___2540: {
                t___8117 = match cs__1019.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2540
                };
                sqlFrag__1020 = t___8117.clone();
                break 'ok___15606;
            }
            sqlFrag__1020 = panic!();
        }
        let s__1021: std::sync::Arc<String> = sqlFrag__1020.to_string();
        let mut t___14811: bool = temper_core::string::index_of( & s__1021, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___51 {
            s__1021: std::sync::Arc<String>
        }
        impl ClosureGroup___51 {
            fn fn__14796(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__1021.clone()));
            }
        }
        let closure_group = ClosureGroup___51 {
            s__1021: s__1021.clone()
        };
        let fn__14796 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14796())
        };
        test___50.assert(t___14811, fn__14796.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__2139() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let params__1023: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___14789: TableDef = userTable__661();
        let mut t___14790: SafeIdentifier = csid__660("name");
        let cs__1024: Changeset = changeset(t___14789.clone(), params__1023.clone()).cast(std::sync::Arc::new(vec![t___14790.clone()])).validate_required(std::sync::Arc::new(vec![csid__660("name")]));
        let didBubble__1025: bool;
        'ok___15607: {
            'orelse___2541: {
                match cs__1024.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2541
                };
                didBubble__1025 = false;
                break 'ok___15607;
            }
            didBubble__1025 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__14787(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__14787 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14787())
        };
        test___51.assert(didBubble__1025, fn__14787.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__2140() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let strictTable__1027: TableDef = TableDef::new(csid__660("posts"), [FieldDef::new(csid__660("title"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("body"), FieldType::new(StringField::new()), true, None, false)], None);
        let params__1028: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___14780: SafeIdentifier = csid__660("body");
        let cs__1029: Changeset = changeset(strictTable__1027.clone(), params__1028.clone()).cast(std::sync::Arc::new(vec![t___14780.clone()]));
        let mut t___14782: bool = cs__1029.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___53 {}
        impl ClosureGroup___53 {
            fn fn__14769(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___53 {};
        let fn__14769 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14769())
        };
        test___52.assert(t___14782, fn__14769.clone());
        let didBubble__1030: bool;
        'ok___15608: {
            'orelse___2542: {
                match cs__1029.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2542
                };
                didBubble__1030 = false;
                break 'ok___15608;
            }
            didBubble__1030 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__14768(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__14768 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14768())
        };
        test___52.assert(didBubble__1030, fn__14768.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__2141() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let mut t___8077: SqlFragment;
        let params__1032: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___14759: TableDef = userTable__661();
        let mut t___14760: SafeIdentifier = csid__660("name");
        let cs__1033: Changeset = changeset(t___14759.clone(), params__1032.clone()).cast(std::sync::Arc::new(vec![t___14760.clone()])).validate_required(std::sync::Arc::new(vec![csid__660("name")]));
        let sqlFrag__1034: SqlFragment;
        'ok___15609: {
            'orelse___2543: {
                t___8077 = match cs__1033.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___2543
                };
                sqlFrag__1034 = t___8077.clone();
                break 'ok___15609;
            }
            sqlFrag__1034 = panic!();
        }
        let s__1035: std::sync::Arc<String> = sqlFrag__1034.to_string();
        let mut t___14766: bool = Some(s__1035.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___55 {
            s__1035: std::sync::Arc<String>
        }
        impl ClosureGroup___55 {
            fn fn__14756(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1035.clone()));
            }
        }
        let closure_group = ClosureGroup___55 {
            s__1035: s__1035.clone()
        };
        let fn__14756 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14756())
        };
        test___53.assert(t___14766, fn__14756.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__2142() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let params__1037: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___14749: TableDef = userTable__661();
        let mut t___14750: SafeIdentifier = csid__660("name");
        let cs__1038: Changeset = changeset(t___14749.clone(), params__1037.clone()).cast(std::sync::Arc::new(vec![t___14750.clone()])).validate_required(std::sync::Arc::new(vec![csid__660("name")]));
        let didBubble__1039: bool;
        'ok___15610: {
            'orelse___2544: {
                match cs__1038.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2544
                };
                didBubble__1039 = false;
                break 'ok___15610;
            }
            didBubble__1039 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__14747(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__14747 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14747())
        };
        test___54.assert(didBubble__1039, fn__14747.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn putChangeAddsANewField__2143() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let params__1041: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___14733: TableDef = userTable__661();
        let mut t___14734: SafeIdentifier = csid__660("name");
        let cs__1042: Changeset = changeset(t___14733.clone(), params__1041.clone()).cast(std::sync::Arc::new(vec![t___14734.clone()])).put_change(csid__660("email"), std::sync::Arc::new("alice@example.com".to_string()));
        let mut t___14739: bool = temper_core::MappedTrait::has( & cs__1042.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__14730(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__14730 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14730())
        };
        test___55.assert(t___14739, fn__14730.clone());
        let mut t___14745: bool = Some(temper_core::MappedTrait::get_or( & cs__1042.changes(), std::sync::Arc::new("email".to_string()), std::sync::Arc::new("".to_string())).as_str()) == Some("alice@example.com");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__14729(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email value".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__14729 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14729())
        };
        test___55.assert(t___14745, fn__14729.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn putChangeOverwritesExistingField__2144() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let params__1044: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___14719: TableDef = userTable__661();
        let mut t___14720: SafeIdentifier = csid__660("name");
        let cs__1045: Changeset = changeset(t___14719.clone(), params__1044.clone()).cast(std::sync::Arc::new(vec![t___14720.clone()])).put_change(csid__660("name"), std::sync::Arc::new("Bob".to_string()));
        let mut t___14727: bool = Some(temper_core::MappedTrait::get_or( & cs__1045.changes(), std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())).as_str()) == Some("Bob");
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__14716(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be overwritten".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__14716 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14716())
        };
        test___56.assert(t___14727, fn__14716.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn putChangeValueAppearsInToInsertSql__2145() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___8032: SqlFragment;
        let mut t___8033: SqlFragment;
        let params__1047: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___14705: TableDef = userTable__661();
        let mut t___14706: SafeIdentifier = csid__660("name");
        let mut t___14707: SafeIdentifier = csid__660("email");
        let cs__1048: Changeset = changeset(t___14705.clone(), params__1047.clone()).cast(std::sync::Arc::new(vec![t___14706.clone(), t___14707.clone()])).put_change(csid__660("name"), std::sync::Arc::new("Bob".to_string()));
        'ok___15611: {
            'orelse___2545: {
                t___8032 = match cs__1048.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2545
                };
                t___8033 = t___8032.clone();
                break 'ok___15611;
            }
            t___8033 = panic!();
        }
        let s__1049: std::sync::Arc<String> = t___8033.to_string();
        let mut t___14713: bool = temper_core::string::index_of( & s__1049, "'Bob'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___60 {
            s__1049: std::sync::Arc<String>
        }
        impl ClosureGroup___60 {
            fn fn__14701(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should use putChange value: {}", self.s__1049.clone()));
            }
        }
        let closure_group = ClosureGroup___60 {
            s__1049: s__1049.clone()
        };
        let fn__14701 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14701())
        };
        test___57.assert(t___14713, fn__14701.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn getChangeReturnsValueForExistingField__2146() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let mut t___8020: std::sync::Arc<String>;
        let params__1051: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___14694: TableDef = userTable__661();
        let mut t___14695: SafeIdentifier = csid__660("name");
        let cs__1052: Changeset = changeset(t___14694.clone(), params__1051.clone()).cast(std::sync::Arc::new(vec![t___14695.clone()]));
        let val__1053: std::sync::Arc<String>;
        'ok___15612: {
            'orelse___2546: {
                t___8020 = match cs__1052.get_change(csid__660("name")) {
                    Ok(x) => x,
                    _ => break 'orelse___2546
                };
                val__1053 = t___8020.clone();
                break 'ok___15612;
            }
            val__1053 = panic!();
        }
        let mut t___14699: bool = Some(val__1053.as_str()) == Some("Alice");
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__14691(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should return Alice".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__14691 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14691())
        };
        test___58.assert(t___14699, fn__14691.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn getChangeBubblesOnMissingField__2147() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let params__1055: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___14685: TableDef = userTable__661();
        let mut t___14686: SafeIdentifier = csid__660("name");
        let cs__1056: Changeset = changeset(t___14685.clone(), params__1055.clone()).cast(std::sync::Arc::new(vec![t___14686.clone()]));
        let didBubble__1057: bool;
        'ok___15613: {
            'orelse___2547: {
                match cs__1056.get_change(csid__660("email")) {
                    Ok(x) => x,
                    _ => break 'orelse___2547
                };
                didBubble__1057 = false;
                break 'ok___15613;
            }
            didBubble__1057 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__14682(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should bubble for missing field".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__14682 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14682())
        };
        test___59.assert(didBubble__1057, fn__14682.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn deleteChangeRemovesField__2148() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let params__1059: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___14667: TableDef = userTable__661();
        let mut t___14668: SafeIdentifier = csid__660("name");
        let mut t___14669: SafeIdentifier = csid__660("email");
        let cs__1060: Changeset = changeset(t___14667.clone(), params__1059.clone()).cast(std::sync::Arc::new(vec![t___14668.clone(), t___14669.clone()])).delete_change(csid__660("email"));
        let mut t___14676: bool = ! temper_core::MappedTrait::has( & cs__1060.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___63 {}
        impl ClosureGroup___63 {
            fn fn__14663(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be removed".to_string());
            }
        }
        let closure_group = ClosureGroup___63 {};
        let fn__14663 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14663())
        };
        test___60.assert(t___14676, fn__14663.clone());
        let mut t___14679: bool = temper_core::MappedTrait::has( & cs__1060.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___64 {}
        impl ClosureGroup___64 {
            fn fn__14662(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should remain".to_string());
            }
        }
        let closure_group = ClosureGroup___64 {};
        let fn__14662 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14662())
        };
        test___60.assert(t___14679, fn__14662.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn deleteChangeOnNonexistentFieldIsNoOp__2149() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let params__1062: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___14650: TableDef = userTable__661();
        let mut t___14651: SafeIdentifier = csid__660("name");
        let cs__1063: Changeset = changeset(t___14650.clone(), params__1062.clone()).cast(std::sync::Arc::new(vec![t___14651.clone()])).delete_change(csid__660("email"));
        let mut t___14656: bool = temper_core::MappedTrait::has( & cs__1063.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__14647(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should still be present".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__14647 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14647())
        };
        test___61.assert(t___14656, fn__14647.clone());
        let mut t___14659: bool = cs__1063.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___66 {}
        impl ClosureGroup___66 {
            fn fn__14646(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___66 {};
        let fn__14646 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14646())
        };
        test___61.assert(t___14659, fn__14646.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionPassesWhenValueInList__2150() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let params__1065: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("admin".to_string()))]);
        let mut t___14638: TableDef = userTable__661();
        let mut t___14639: SafeIdentifier = csid__660("name");
        let cs__1066: Changeset = changeset(t___14638.clone(), params__1065.clone()).cast(std::sync::Arc::new(vec![t___14639.clone()])).validate_inclusion(csid__660("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string()), std::sync::Arc::new("guest".to_string())]));
        let mut t___14643: bool = cs__1066.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___67 {}
        impl ClosureGroup___67 {
            fn fn__14635(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___67 {};
        let fn__14635 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14635())
        };
        test___62.assert(t___14643, fn__14635.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionFailsWhenValueNotInList__2151() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let params__1068: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("hacker".to_string()))]);
        let mut t___14620: TableDef = userTable__661();
        let mut t___14621: SafeIdentifier = csid__660("name");
        let cs__1069: Changeset = changeset(t___14620.clone(), params__1068.clone()).cast(std::sync::Arc::new(vec![t___14621.clone()])).validate_inclusion(csid__660("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string()), std::sync::Arc::new("guest".to_string())]));
        let mut t___14627: bool = ! cs__1069.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__14617(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__14617 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14617())
        };
        test___63.assert(t___14627, fn__14617.clone());
        let mut t___14633: bool = Some(temper_core::ListedTrait::get( & cs__1069.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___69 {}
        impl ClosureGroup___69 {
            fn fn__14616(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on name".to_string());
            }
        }
        let closure_group = ClosureGroup___69 {};
        let fn__14616 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14616())
        };
        test___63.assert(t___14633, fn__14616.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn validateInclusionSkipsWhenFieldNotInChanges__2152() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let params__1071: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___14608: TableDef = userTable__661();
        let mut t___14609: SafeIdentifier = csid__660("name");
        let cs__1072: Changeset = changeset(t___14608.clone(), params__1071.clone()).cast(std::sync::Arc::new(vec![t___14609.clone()])).validate_inclusion(csid__660("name"), std::sync::Arc::new(vec![std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("user".to_string())]));
        let mut t___14613: bool = cs__1072.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___70 {}
        impl ClosureGroup___70 {
            fn fn__14606(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___70 {};
        let fn__14606 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14606())
        };
        test___64.assert(t___14613, fn__14606.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionPassesWhenValueNotInList__2153() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let params__1074: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___14598: TableDef = userTable__661();
        let mut t___14599: SafeIdentifier = csid__660("name");
        let cs__1075: Changeset = changeset(t___14598.clone(), params__1074.clone()).cast(std::sync::Arc::new(vec![t___14599.clone()])).validate_exclusion(csid__660("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("superuser".to_string())]));
        let mut t___14603: bool = cs__1075.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__14595(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__14595 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14595())
        };
        test___65.assert(t___14603, fn__14595.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionFailsWhenValueInList__2154() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let params__1077: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("admin".to_string()))]);
        let mut t___14580: TableDef = userTable__661();
        let mut t___14581: SafeIdentifier = csid__660("name");
        let cs__1078: Changeset = changeset(t___14580.clone(), params__1077.clone()).cast(std::sync::Arc::new(vec![t___14581.clone()])).validate_exclusion(csid__660("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("superuser".to_string())]));
        let mut t___14587: bool = ! cs__1078.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__14577(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__14577 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14577())
        };
        test___66.assert(t___14587, fn__14577.clone());
        let mut t___14593: bool = Some(temper_core::ListedTrait::get( & cs__1078.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__14576(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on name".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__14576 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14576())
        };
        test___66.assert(t___14593, fn__14576.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn validateExclusionSkipsWhenFieldNotInChanges__2155() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let params__1080: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___14568: TableDef = userTable__661();
        let mut t___14569: SafeIdentifier = csid__660("name");
        let cs__1081: Changeset = changeset(t___14568.clone(), params__1080.clone()).cast(std::sync::Arc::new(vec![t___14569.clone()])).validate_exclusion(csid__660("name"), std::sync::Arc::new(vec![std::sync::Arc::new("root".to_string()), std::sync::Arc::new("admin".to_string())]));
        let mut t___14573: bool = cs__1081.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__14566(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__14566 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14566())
        };
        test___67.assert(t___14573, fn__14566.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanPasses__2156() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let params__1083: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___14557: TableDef = userTable__661();
        let mut t___14558: SafeIdentifier = csid__660("age");
        let cs__1084: Changeset = changeset(t___14557.clone(), params__1083.clone()).cast(std::sync::Arc::new(vec![t___14558.clone()])).validate_number(csid__660("age"), NumberValidationOpts::new(Some(18.0f64), None, None, None, None));
        let mut t___14563: bool = cs__1084.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__14554(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("25 > 18 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__14554 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14554())
        };
        test___68.assert(t___14563, fn__14554.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanFails__2157() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let params__1086: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("15".to_string()))]);
        let mut t___14544: TableDef = userTable__661();
        let mut t___14545: SafeIdentifier = csid__660("age");
        let cs__1087: Changeset = changeset(t___14544.clone(), params__1086.clone()).cast(std::sync::Arc::new(vec![t___14545.clone()])).validate_number(csid__660("age"), NumberValidationOpts::new(Some(18.0f64), None, None, None, None));
        let mut t___14552: bool = ! cs__1087.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__14541(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("15 > 18 should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__14541 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14541())
        };
        test___69.assert(t___14552, fn__14541.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanPasses__2158() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let params__1089: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("8.5".to_string()))]);
        let mut t___14532: TableDef = userTable__661();
        let mut t___14533: SafeIdentifier = csid__660("score");
        let cs__1090: Changeset = changeset(t___14532.clone(), params__1089.clone()).cast(std::sync::Arc::new(vec![t___14533.clone()])).validate_number(csid__660("score"), NumberValidationOpts::new(None, Some(10.0f64), None, None, None));
        let mut t___14538: bool = cs__1090.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__14529(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("8.5 < 10 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__14529 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14529())
        };
        test___70.assert(t___14538, fn__14529.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberLessThanFails__2159() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let params__1092: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("12.0".to_string()))]);
        let mut t___14519: TableDef = userTable__661();
        let mut t___14520: SafeIdentifier = csid__660("score");
        let cs__1093: Changeset = changeset(t___14519.clone(), params__1092.clone()).cast(std::sync::Arc::new(vec![t___14520.clone()])).validate_number(csid__660("score"), NumberValidationOpts::new(None, Some(10.0f64), None, None, None));
        let mut t___14527: bool = ! cs__1093.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__14516(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("12 < 10 should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__14516 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14516())
        };
        test___71.assert(t___14527, fn__14516.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberGreaterThanOrEqualBoundary__2160() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let params__1095: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("18".to_string()))]);
        let mut t___14507: TableDef = userTable__661();
        let mut t___14508: SafeIdentifier = csid__660("age");
        let cs__1096: Changeset = changeset(t___14507.clone(), params__1095.clone()).cast(std::sync::Arc::new(vec![t___14508.clone()])).validate_number(csid__660("age"), NumberValidationOpts::new(None, None, Some(18.0f64), None, None));
        let mut t___14513: bool = cs__1096.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__14504(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("18 >= 18 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__14504 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14504())
        };
        test___72.assert(t___14513, fn__14504.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberCombinedOptions__2161() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let params__1098: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("5.0".to_string()))]);
        let mut t___14495: TableDef = userTable__661();
        let mut t___14496: SafeIdentifier = csid__660("score");
        let cs__1099: Changeset = changeset(t___14495.clone(), params__1098.clone()).cast(std::sync::Arc::new(vec![t___14496.clone()])).validate_number(csid__660("score"), NumberValidationOpts::new(Some(0.0f64), Some(10.0f64), None, None, None));
        let mut t___14501: bool = cs__1099.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___80 {}
        impl ClosureGroup___80 {
            fn fn__14492(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("5 > 0 and < 10 should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___80 {};
        let fn__14492 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14492())
        };
        test___73.assert(t___14501, fn__14492.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberNonNumericValue__2162() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let params__1101: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("abc".to_string()))]);
        let mut t___14476: TableDef = userTable__661();
        let mut t___14477: SafeIdentifier = csid__660("age");
        let cs__1102: Changeset = changeset(t___14476.clone(), params__1101.clone()).cast(std::sync::Arc::new(vec![t___14477.clone()])).validate_number(csid__660("age"), NumberValidationOpts::new(Some(0.0f64), None, None, None, None));
        let mut t___14484: bool = ! cs__1102.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___81 {}
        impl ClosureGroup___81 {
            fn fn__14473(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("non-numeric should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___81 {};
        let fn__14473 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14473())
        };
        test___74.assert(t___14484, fn__14473.clone());
        let mut t___14490: bool = Some(temper_core::ListedTrait::get( & cs__1102.errors(), 0).message().as_str()) == Some("must be a number");
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__14472(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct error message".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__14472 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14472())
        };
        test___74.assert(t___14490, fn__14472.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn validateNumberSkipsWhenFieldNotInChanges__2163() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let params__1104: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___14463: TableDef = userTable__661();
        let mut t___14464: SafeIdentifier = csid__660("age");
        let cs__1105: Changeset = changeset(t___14463.clone(), params__1104.clone()).cast(std::sync::Arc::new(vec![t___14464.clone()])).validate_number(csid__660("age"), NumberValidationOpts::new(Some(0.0f64), None, None, None, None));
        let mut t___14469: bool = cs__1105.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__14461(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__14461 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14461())
        };
        test___75.assert(t___14469, fn__14461.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn validateAcceptancePassesForTrueValues__2164() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___84 {
            test___76: temper_std::testing::Test
        }
        impl ClosureGroup___84 {
            fn fn__14458(& self, v__1107: impl temper_core::ToArcString) {
                let v__1107 = v__1107.to_arc_string();
                let params__1108: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__1107.clone())]);
                let mut t___14450: TableDef = userTable__661();
                let mut t___14451: SafeIdentifier = csid__660("active");
                let cs__1109: Changeset = changeset(t___14450.clone(), params__1108.clone()).cast(std::sync::Arc::new(vec![t___14451.clone()])).validate_acceptance(csid__660("active"));
                let mut t___14455: bool = cs__1109.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___85 {
                    v__1107: std::sync::Arc<String>
                }
                impl ClosureGroup___85 {
                    fn fn__14447(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__1107.clone()));
                    }
                }
                let closure_group = ClosureGroup___85 {
                    v__1107: v__1107.clone()
                };
                let fn__14447 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__14447())
                };
                self.test___76.assert(t___14455, fn__14447.clone());
            }
        }
        let closure_group = ClosureGroup___84 {
            test___76: test___76.clone()
        };
        let fn__14458 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__1107: std::sync::Arc<String> | closure_group.fn__14458(v__1107))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__14458.clone()));
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn validateAcceptanceFailsForNonTrueValues__2165() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let params__1111: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), std::sync::Arc::new("false".to_string()))]);
        let mut t___14432: TableDef = userTable__661();
        let mut t___14433: SafeIdentifier = csid__660("active");
        let cs__1112: Changeset = changeset(t___14432.clone(), params__1111.clone()).cast(std::sync::Arc::new(vec![t___14433.clone()])).validate_acceptance(csid__660("active"));
        let mut t___14439: bool = ! cs__1112.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn fn__14429(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("false should not be accepted".to_string());
            }
        }
        let closure_group = ClosureGroup___86 {};
        let fn__14429 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14429())
        };
        test___77.assert(t___14439, fn__14429.clone());
        let mut t___14445: bool = Some(temper_core::ListedTrait::get( & cs__1112.errors(), 0).message().as_str()) == Some("must be accepted");
        #[derive(Clone)]
        struct ClosureGroup___87 {}
        impl ClosureGroup___87 {
            fn fn__14428(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("correct message".to_string());
            }
        }
        let closure_group = ClosureGroup___87 {};
        let fn__14428 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14428())
        };
        test___77.assert(t___14445, fn__14428.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationPassesWhenFieldsMatch__2166() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let tbl__1114: TableDef = TableDef::new(csid__660("users"), [FieldDef::new(csid__660("password"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("password_confirmation"), FieldType::new(StringField::new()), true, None, false)], None);
        let params__1115: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string())), (std::sync::Arc::new("password_confirmation".to_string()), std::sync::Arc::new("secret123".to_string()))]);
        let mut t___14419: SafeIdentifier = csid__660("password");
        let mut t___14420: SafeIdentifier = csid__660("password_confirmation");
        let cs__1116: Changeset = changeset(tbl__1114.clone(), params__1115.clone()).cast(std::sync::Arc::new(vec![t___14419.clone(), t___14420.clone()])).validate_confirmation(csid__660("password"), csid__660("password_confirmation"));
        let mut t___14425: bool = cs__1116.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___88 {}
        impl ClosureGroup___88 {
            fn fn__14407(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("matching fields should pass".to_string());
            }
        }
        let closure_group = ClosureGroup___88 {};
        let fn__14407 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14407())
        };
        test___78.assert(t___14425, fn__14407.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationFailsWhenFieldsDiffer__2167() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let tbl__1118: TableDef = TableDef::new(csid__660("users"), [FieldDef::new(csid__660("password"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("password_confirmation"), FieldType::new(StringField::new()), true, None, false)], None);
        let params__1119: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string())), (std::sync::Arc::new("password_confirmation".to_string()), std::sync::Arc::new("wrong456".to_string()))]);
        let mut t___14391: SafeIdentifier = csid__660("password");
        let mut t___14392: SafeIdentifier = csid__660("password_confirmation");
        let cs__1120: Changeset = changeset(tbl__1118.clone(), params__1119.clone()).cast(std::sync::Arc::new(vec![t___14391.clone(), t___14392.clone()])).validate_confirmation(csid__660("password"), csid__660("password_confirmation"));
        let mut t___14399: bool = ! cs__1120.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__14379(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mismatched fields should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__14379 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14379())
        };
        test___79.assert(t___14399, fn__14379.clone());
        let mut t___14405: bool = Some(temper_core::ListedTrait::get( & cs__1120.errors(), 0).field().as_str()) == Some("password_confirmation");
        #[derive(Clone)]
        struct ClosureGroup___90 {}
        impl ClosureGroup___90 {
            fn fn__14378(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error on confirmation field".to_string());
            }
        }
        let closure_group = ClosureGroup___90 {};
        let fn__14378 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14378())
        };
        test___79.assert(t___14405, fn__14378.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn validateConfirmationFailsWhenConfirmationMissing__2168() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let tbl__1122: TableDef = TableDef::new(csid__660("users"), [FieldDef::new(csid__660("password"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("password_confirmation"), FieldType::new(StringField::new()), true, None, false)], None);
        let params__1123: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("password".to_string()), std::sync::Arc::new("secret123".to_string()))]);
        let mut t___14369: SafeIdentifier = csid__660("password");
        let cs__1124: Changeset = changeset(tbl__1122.clone(), params__1123.clone()).cast(std::sync::Arc::new(vec![t___14369.clone()])).validate_confirmation(csid__660("password"), csid__660("password_confirmation"));
        let mut t___14376: bool = ! cs__1124.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___91 {}
        impl ClosureGroup___91 {
            fn fn__14358(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("missing confirmation should fail".to_string());
            }
        }
        let closure_group = ClosureGroup___91 {};
        let fn__14358 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14358())
        };
        test___80.assert(t___14376, fn__14358.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsPassesWhenSubstringFound__2169() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let params__1126: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___14350: TableDef = userTable__661();
        let mut t___14351: SafeIdentifier = csid__660("email");
        let cs__1127: Changeset = changeset(t___14350.clone(), params__1126.clone()).cast(std::sync::Arc::new(vec![t___14351.clone()])).validate_contains(csid__660("email"), std::sync::Arc::new("@".to_string()));
        let mut t___14355: bool = cs__1127.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___92 {}
        impl ClosureGroup___92 {
            fn fn__14347(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass when @ present".to_string());
            }
        }
        let closure_group = ClosureGroup___92 {};
        let fn__14347 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14347())
        };
        test___81.assert(t___14355, fn__14347.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsFailsWhenSubstringNotFound__2170() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let params__1129: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice-example.com".to_string()))]);
        let mut t___14338: TableDef = userTable__661();
        let mut t___14339: SafeIdentifier = csid__660("email");
        let cs__1130: Changeset = changeset(t___14338.clone(), params__1129.clone()).cast(std::sync::Arc::new(vec![t___14339.clone()])).validate_contains(csid__660("email"), std::sync::Arc::new("@".to_string()));
        let mut t___14345: bool = ! cs__1130.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___93 {}
        impl ClosureGroup___93 {
            fn fn__14335(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail when @ absent".to_string());
            }
        }
        let closure_group = ClosureGroup___93 {};
        let fn__14335 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14335())
        };
        test___82.assert(t___14345, fn__14335.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn validateContainsSkipsWhenFieldNotInChanges__2171() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let params__1132: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___14327: TableDef = userTable__661();
        let mut t___14328: SafeIdentifier = csid__660("email");
        let cs__1133: Changeset = changeset(t___14327.clone(), params__1132.clone()).cast(std::sync::Arc::new(vec![t___14328.clone()])).validate_contains(csid__660("email"), std::sync::Arc::new("@".to_string()));
        let mut t___14332: bool = cs__1133.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___94 {}
        impl ClosureGroup___94 {
            fn fn__14325(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid when field absent".to_string());
            }
        }
        let closure_group = ClosureGroup___94 {};
        let fn__14325 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14325())
        };
        test___83.assert(t___14332, fn__14325.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn validateStartsWithPasses__2172() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let params__1135: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Dr. Smith".to_string()))]);
        let mut t___14317: TableDef = userTable__661();
        let mut t___14318: SafeIdentifier = csid__660("name");
        let cs__1136: Changeset = changeset(t___14317.clone(), params__1135.clone()).cast(std::sync::Arc::new(vec![t___14318.clone()])).validate_starts_with(csid__660("name"), std::sync::Arc::new("Dr.".to_string()));
        let mut t___14322: bool = cs__1136.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___95 {}
        impl ClosureGroup___95 {
            fn fn__14314(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass for Dr. prefix".to_string());
            }
        }
        let closure_group = ClosureGroup___95 {};
        let fn__14314 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14314())
        };
        test___84.assert(t___14322, fn__14314.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn validateStartsWithFails__2173() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let params__1138: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Mr. Smith".to_string()))]);
        let mut t___14305: TableDef = userTable__661();
        let mut t___14306: SafeIdentifier = csid__660("name");
        let cs__1139: Changeset = changeset(t___14305.clone(), params__1138.clone()).cast(std::sync::Arc::new(vec![t___14306.clone()])).validate_starts_with(csid__660("name"), std::sync::Arc::new("Dr.".to_string()));
        let mut t___14312: bool = ! cs__1139.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___96 {}
        impl ClosureGroup___96 {
            fn fn__14302(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail for Mr. prefix".to_string());
            }
        }
        let closure_group = ClosureGroup___96 {};
        let fn__14302 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14302())
        };
        test___85.assert(t___14312, fn__14302.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithPasses__2174() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let params__1141: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___14294: TableDef = userTable__661();
        let mut t___14295: SafeIdentifier = csid__660("email");
        let cs__1142: Changeset = changeset(t___14294.clone(), params__1141.clone()).cast(std::sync::Arc::new(vec![t___14295.clone()])).validate_ends_with(csid__660("email"), std::sync::Arc::new(".com".to_string()));
        let mut t___14299: bool = cs__1142.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___97 {}
        impl ClosureGroup___97 {
            fn fn__14291(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should pass for .com suffix".to_string());
            }
        }
        let closure_group = ClosureGroup___97 {};
        let fn__14291 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14291())
        };
        test___86.assert(t___14299, fn__14291.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithFails__2175() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let params__1144: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.org".to_string()))]);
        let mut t___14282: TableDef = userTable__661();
        let mut t___14283: SafeIdentifier = csid__660("email");
        let cs__1145: Changeset = changeset(t___14282.clone(), params__1144.clone()).cast(std::sync::Arc::new(vec![t___14283.clone()])).validate_ends_with(csid__660("email"), std::sync::Arc::new(".com".to_string()));
        let mut t___14289: bool = ! cs__1145.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___98 {}
        impl ClosureGroup___98 {
            fn fn__14279(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should fail for .org when expecting .com".to_string());
            }
        }
        let closure_group = ClosureGroup___98 {};
        let fn__14279 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14279())
        };
        test___87.assert(t___14289, fn__14279.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn validateEndsWithHandlesRepeatedSuffixCorrectly__2176() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let params__1147: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("abcabc".to_string()))]);
        let mut t___14271: TableDef = userTable__661();
        let mut t___14272: SafeIdentifier = csid__660("name");
        let cs__1148: Changeset = changeset(t___14271.clone(), params__1147.clone()).cast(std::sync::Arc::new(vec![t___14272.clone()])).validate_ends_with(csid__660("name"), std::sync::Arc::new("abc".to_string()));
        let mut t___14276: bool = cs__1148.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___99 {}
        impl ClosureGroup___99 {
            fn fn__14268(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("abcabc should end with abc".to_string());
            }
        }
        let closure_group = ClosureGroup___99 {};
        let fn__14268 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14268())
        };
        test___88.assert(t___14276, fn__14268.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlUsesDefaultValueWhenFieldNotInChanges__2177() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let mut t___7668: SqlFragment;
        let mut t___7669: SqlFragment;
        let tbl__1150: TableDef = TableDef::new(csid__660("posts"), [FieldDef::new(csid__660("title"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("status"), FieldType::new(StringField::new()), false, Some(SqlPart::new(SqlDefault::new())), false)], None);
        let params__1151: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("title".to_string()), std::sync::Arc::new("Hello".to_string()))]);
        let mut t___14252: SafeIdentifier = csid__660("title");
        let cs__1152: Changeset = changeset(tbl__1150.clone(), params__1151.clone()).cast(std::sync::Arc::new(vec![t___14252.clone()]));
        'ok___15614: {
            'orelse___2548: {
                t___7668 = match cs__1152.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2548
                };
                t___7669 = t___7668.clone();
                break 'ok___15614;
            }
            t___7669 = panic!();
        }
        let s__1153: std::sync::Arc<String> = t___7669.to_string();
        let mut t___14256: bool = temper_core::string::index_of( & s__1153, "INSERT INTO posts", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___100 {
            s__1153: std::sync::Arc<String>
        }
        impl ClosureGroup___100 {
            fn fn__14240(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__1153.clone()));
            }
        }
        let closure_group = ClosureGroup___100 {
            s__1153: s__1153.clone()
        };
        let fn__14240 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14240())
        };
        test___89.assert(t___14256, fn__14240.clone());
        let mut t___14260: bool = temper_core::string::index_of( & s__1153, "'Hello'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___101 {
            s__1153: std::sync::Arc<String>
        }
        impl ClosureGroup___101 {
            fn fn__14239(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has title value: {}", self.s__1153.clone()));
            }
        }
        let closure_group = ClosureGroup___101 {
            s__1153: s__1153.clone()
        };
        let fn__14239 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14239())
        };
        test___89.assert(t___14260, fn__14239.clone());
        let mut t___14264: bool = temper_core::string::index_of( & s__1153, "DEFAULT", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___102 {
            s__1153: std::sync::Arc<String>
        }
        impl ClosureGroup___102 {
            fn fn__14238(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("status should use DEFAULT: {}", self.s__1153.clone()));
            }
        }
        let closure_group = ClosureGroup___102 {
            s__1153: s__1153.clone()
        };
        let fn__14238 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14238())
        };
        test___89.assert(t___14264, fn__14238.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlChangeOverridesDefaultValue__2178() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___90 = temper_std::testing::Test::new();
        let mut t___7648: SqlFragment;
        let mut t___7649: SqlFragment;
        let tbl__1155: TableDef = TableDef::new(csid__660("posts"), [FieldDef::new(csid__660("title"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("status"), FieldType::new(StringField::new()), false, Some(SqlPart::new(SqlDefault::new())), false)], None);
        let params__1156: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("title".to_string()), std::sync::Arc::new("Hello".to_string())), (std::sync::Arc::new("status".to_string()), std::sync::Arc::new("published".to_string()))]);
        let mut t___14230: SafeIdentifier = csid__660("title");
        let mut t___14231: SafeIdentifier = csid__660("status");
        let cs__1157: Changeset = changeset(tbl__1155.clone(), params__1156.clone()).cast(std::sync::Arc::new(vec![t___14230.clone(), t___14231.clone()]));
        'ok___15615: {
            'orelse___2549: {
                t___7648 = match cs__1157.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2549
                };
                t___7649 = t___7648.clone();
                break 'ok___15615;
            }
            t___7649 = panic!();
        }
        let s__1158: std::sync::Arc<String> = t___7649.to_string();
        let mut t___14235: bool = temper_core::string::index_of( & s__1158, "'published'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___103 {
            s__1158: std::sync::Arc<String>
        }
        impl ClosureGroup___103 {
            fn fn__14217(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should use provided value: {}", self.s__1158.clone()));
            }
        }
        let closure_group = ClosureGroup___103 {
            s__1158: s__1158.clone()
        };
        let fn__14217 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14217())
        };
        test___90.assert(t___14235, fn__14217.clone());
        test___90.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlWithTimestampsUsesDefault__2179() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___91 = temper_std::testing::Test::new();
        let mut t___7595: temper_core::List<FieldDef>;
        let mut t___7610: SqlFragment;
        let mut t___7611: SqlFragment;
        let ts__1160: temper_core::List<FieldDef>;
        'ok___15616: {
            'orelse___2550: {
                t___7595 = match timestamps() {
                    Ok(x) => x,
                    _ => break 'orelse___2550
                };
                ts__1160 = t___7595.clone();
                break 'ok___15616;
            }
            ts__1160 = panic!();
        }
        let fields__1161: temper_core::ListBuilder<FieldDef> = temper_core::listed::new_builder();
        temper_core::listed::add( & fields__1161, FieldDef::new(csid__660("title"), FieldType::new(StringField::new()), false, None, false), None);
        #[derive(Clone)]
        struct ClosureGroup___104 {
            fields__1161: temper_core::ListBuilder<FieldDef>
        }
        impl ClosureGroup___104 {
            fn fn__14183(& self, t__1162: FieldDef) {
                temper_core::listed::add( & self.fields__1161, t__1162.clone(), None);
            }
        }
        let closure_group = ClosureGroup___104 {
            fields__1161: fields__1161.clone()
        };
        let fn__14183 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | t__1162: FieldDef | closure_group.fn__14183(t__1162))
        };
        temper_core::listed::list_for_each( & ts__1160, & ( * fn__14183.clone()));
        let tbl__1163: TableDef = TableDef::new(csid__660("articles"), temper_core::ListedTrait::to_list( & fields__1161), None);
        let params__1164: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("title".to_string()), std::sync::Arc::new("News".to_string()))]);
        let mut t___14196: SafeIdentifier = csid__660("title");
        let cs__1165: Changeset = changeset(tbl__1163.clone(), params__1164.clone()).cast(std::sync::Arc::new(vec![t___14196.clone()]));
        'ok___15617: {
            'orelse___2551: {
                t___7610 = match cs__1165.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2551
                };
                t___7611 = t___7610.clone();
                break 'ok___15617;
            }
            t___7611 = panic!();
        }
        let s__1166: std::sync::Arc<String> = t___7611.to_string();
        let mut t___14200: bool = temper_core::string::index_of( & s__1166, "inserted_at", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___105 {
            s__1166: std::sync::Arc<String>
        }
        impl ClosureGroup___105 {
            fn fn__14182(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should include inserted_at: {}", self.s__1166.clone()));
            }
        }
        let closure_group = ClosureGroup___105 {
            s__1166: s__1166.clone()
        };
        let fn__14182 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14182())
        };
        test___91.assert(t___14200, fn__14182.clone());
        let mut t___14204: bool = temper_core::string::index_of( & s__1166, "updated_at", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___106 {
            s__1166: std::sync::Arc<String>
        }
        impl ClosureGroup___106 {
            fn fn__14181(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should include updated_at: {}", self.s__1166.clone()));
            }
        }
        let closure_group = ClosureGroup___106 {
            s__1166: s__1166.clone()
        };
        let fn__14181 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14181())
        };
        test___91.assert(t___14204, fn__14181.clone());
        let mut t___14208: bool = temper_core::string::index_of( & s__1166, "DEFAULT", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___107 {
            s__1166: std::sync::Arc<String>
        }
        impl ClosureGroup___107 {
            fn fn__14180(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("timestamps should use DEFAULT: {}", self.s__1166.clone()));
            }
        }
        let closure_group = ClosureGroup___107 {
            s__1166: s__1166.clone()
        };
        let fn__14180 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14180())
        };
        test___91.assert(t___14208, fn__14180.clone());
        test___91.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlSkipsVirtualFields__2180() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___92 = temper_std::testing::Test::new();
        let mut t___7584: SqlFragment;
        let mut t___7585: SqlFragment;
        let tbl__1168: TableDef = TableDef::new(csid__660("users"), [FieldDef::new(csid__660("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("full_name"), FieldType::new(StringField::new()), true, None, true)], None);
        let params__1169: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("full_name".to_string()), std::sync::Arc::new("Alice Smith".to_string()))]);
        let mut t___14166: SafeIdentifier = csid__660("name");
        let mut t___14167: SafeIdentifier = csid__660("full_name");
        let cs__1170: Changeset = changeset(tbl__1168.clone(), params__1169.clone()).cast(std::sync::Arc::new(vec![t___14166.clone(), t___14167.clone()]));
        'ok___15618: {
            'orelse___2552: {
                t___7584 = match cs__1170.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2552
                };
                t___7585 = t___7584.clone();
                break 'ok___15618;
            }
            t___7585 = panic!();
        }
        let s__1171: std::sync::Arc<String> = t___7585.to_string();
        let mut t___14171: bool = temper_core::string::index_of( & s__1171, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___108 {
            s__1171: std::sync::Arc<String>
        }
        impl ClosureGroup___108 {
            fn fn__14154(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("name should be included: {}", self.s__1171.clone()));
            }
        }
        let closure_group = ClosureGroup___108 {
            s__1171: s__1171.clone()
        };
        let fn__14154 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14154())
        };
        test___92.assert(t___14171, fn__14154.clone());
        let mut t___14177: bool = ! temper_core::string::index_of( & s__1171, "full_name", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___109 {
            s__1171: std::sync::Arc<String>
        }
        impl ClosureGroup___109 {
            fn fn__14153(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("virtual field should be excluded: {}", self.s__1171.clone()));
            }
        }
        let closure_group = ClosureGroup___109 {
            s__1171: s__1171.clone()
        };
        let fn__14153 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14153())
        };
        test___92.assert(t___14177, fn__14153.clone());
        test___92.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlAllowsMissingNonNullableVirtualField__2181() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___93 = temper_std::testing::Test::new();
        let mut t___7563: SqlFragment;
        let mut t___7564: SqlFragment;
        let tbl__1173: TableDef = TableDef::new(csid__660("users"), [FieldDef::new(csid__660("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("computed"), FieldType::new(StringField::new()), false, None, true)], None);
        let params__1174: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___14146: SafeIdentifier = csid__660("name");
        let cs__1175: Changeset = changeset(tbl__1173.clone(), params__1174.clone()).cast(std::sync::Arc::new(vec![t___14146.clone()]));
        'ok___15619: {
            'orelse___2553: {
                t___7563 = match cs__1175.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2553
                };
                t___7564 = t___7563.clone();
                break 'ok___15619;
            }
            t___7564 = panic!();
        }
        let s__1176: std::sync::Arc<String> = t___7564.to_string();
        let mut t___14150: bool = temper_core::string::index_of( & s__1176, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___110 {
            s__1176: std::sync::Arc<String>
        }
        impl ClosureGroup___110 {
            fn fn__14135(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should succeed: {}", self.s__1176.clone()));
            }
        }
        let closure_group = ClosureGroup___110 {
            s__1176: s__1176.clone()
        };
        let fn__14135 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14135())
        };
        test___93.assert(t___14150, fn__14135.clone());
        test___93.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlSkipsVirtualFields__2182() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___94 = temper_std::testing::Test::new();
        let mut t___7540: SqlFragment;
        let mut t___7541: SqlFragment;
        let tbl__1178: TableDef = TableDef::new(csid__660("users"), [FieldDef::new(csid__660("name"), FieldType::new(StringField::new()), false, None, false), FieldDef::new(csid__660("display"), FieldType::new(StringField::new()), true, None, true)], None);
        let params__1179: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("display".to_string()), std::sync::Arc::new("Bobby".to_string()))]);
        let mut t___14122: SafeIdentifier = csid__660("name");
        let mut t___14123: SafeIdentifier = csid__660("display");
        let cs__1180: Changeset = changeset(tbl__1178.clone(), params__1179.clone()).cast(std::sync::Arc::new(vec![t___14122.clone(), t___14123.clone()]));
        'ok___15620: {
            'orelse___2554: {
                t___7540 = match cs__1180.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2554
                };
                t___7541 = t___7540.clone();
                break 'ok___15620;
            }
            t___7541 = panic!();
        }
        let s__1181: std::sync::Arc<String> = t___7541.to_string();
        let mut t___14127: bool = temper_core::string::index_of( & s__1181, "name = 'Bob'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___111 {
            s__1181: std::sync::Arc<String>
        }
        impl ClosureGroup___111 {
            fn fn__14110(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("name should be in SET: {}", self.s__1181.clone()));
            }
        }
        let closure_group = ClosureGroup___111 {
            s__1181: s__1181.clone()
        };
        let fn__14110 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14110())
        };
        test___94.assert(t___14127, fn__14110.clone());
        let mut t___14133: bool = ! temper_core::string::index_of( & s__1181, "display", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___112 {
            s__1181: std::sync::Arc<String>
        }
        impl ClosureGroup___112 {
            fn fn__14109(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("virtual field excluded from UPDATE: {}", self.s__1181.clone()));
            }
        }
        let closure_group = ClosureGroup___112 {
            s__1181: s__1181.clone()
        };
        let fn__14109 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14109())
        };
        test___94.assert(t___14133, fn__14109.clone());
        test___94.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlUsesCustomPrimaryKey__2183() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___95 = temper_std::testing::Test::new();
        let mut t___7522: SqlFragment;
        let mut t___7523: SqlFragment;
        let tbl__1183: TableDef = TableDef::new(csid__660("posts"), [FieldDef::new(csid__660("title"), FieldType::new(StringField::new()), false, None, false)], Some(csid__660("post_id")));
        let params__1184: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("title".to_string()), std::sync::Arc::new("Updated".to_string()))]);
        let mut t___14103: SafeIdentifier = csid__660("title");
        let cs__1185: Changeset = changeset(tbl__1183.clone(), params__1184.clone()).cast(std::sync::Arc::new(vec![t___14103.clone()]));
        'ok___15621: {
            'orelse___2555: {
                t___7522 = match cs__1185.to_update_sql(99) {
                    Ok(x) => x,
                    _ => break 'orelse___2555
                };
                t___7523 = t___7522.clone();
                break 'ok___15621;
            }
            t___7523 = panic!();
        }
        let s__1186: std::sync::Arc<String> = t___7523.to_string();
        let mut t___14107: bool = Some(s__1186.as_str()) == Some("UPDATE posts SET title = 'Updated' WHERE post_id = 99");
        #[derive(Clone)]
        struct ClosureGroup___113 {
            s__1186: std::sync::Arc<String>
        }
        impl ClosureGroup___113 {
            fn fn__14093(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1186.clone()));
            }
        }
        let closure_group = ClosureGroup___113 {
            s__1186: s__1186.clone()
        };
        let fn__14093 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14093())
        };
        test___95.assert(t___14107, fn__14093.clone());
        test___95.soft_fail_to_hard()
    }
    #[test]
    fn deleteSqlUsesCustomPrimaryKey__2184() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___96 = temper_std::testing::Test::new();
        let tbl__1188: TableDef = TableDef::new(csid__660("posts"), [FieldDef::new(csid__660("title"), FieldType::new(StringField::new()), false, None, false)], Some(csid__660("post_id")));
        let s__1189: std::sync::Arc<String> = delete_sql(tbl__1188.clone(), 42).to_string();
        let mut t___14080: bool = Some(s__1189.as_str()) == Some("DELETE FROM posts WHERE post_id = 42");
        #[derive(Clone)]
        struct ClosureGroup___114 {
            s__1189: std::sync::Arc<String>
        }
        impl ClosureGroup___114 {
            fn fn__14069(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1189.clone()));
            }
        }
        let closure_group = ClosureGroup___114 {
            s__1189: s__1189.clone()
        };
        let fn__14069 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14069())
        };
        test___96.assert(t___14080, fn__14069.clone());
        test___96.soft_fail_to_hard()
    }
    #[test]
    fn deleteSqlUsesDefaultIdWhenPrimaryKeyNull__2185() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___97 = temper_std::testing::Test::new();
        let tbl__1191: TableDef = TableDef::new(csid__660("users"), [FieldDef::new(csid__660("name"), FieldType::new(StringField::new()), false, None, false)], None);
        let s__1192: std::sync::Arc<String> = delete_sql(tbl__1191.clone(), 7).to_string();
        let mut t___14067: bool = Some(s__1192.as_str()) == Some("DELETE FROM users WHERE id = 7");
        #[derive(Clone)]
        struct ClosureGroup___115 {
            s__1192: std::sync::Arc<String>
        }
        impl ClosureGroup___115 {
            fn fn__14058(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__1192.clone()));
            }
        }
        let closure_group = ClosureGroup___115 {
            s__1192: s__1192.clone()
        };
        let fn__14058 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__14058())
        };
        test___97.assert(t___14067, fn__14058.clone());
        test___97.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__2267() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___98 = temper_std::testing::Test::new();
        let q__1540: Query = from(sid__662("users"));
        let mut t___13551: bool = Some(q__1540.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___116 {}
        impl ClosureGroup___116 {
            fn fn__13546(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___116 {};
        let fn__13546 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13546())
        };
        test___98.assert(t___13551, fn__13546.clone());
        test___98.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__2268() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___99 = temper_std::testing::Test::new();
        let mut t___13537: SafeIdentifier = sid__662("users");
        let mut t___13538: SafeIdentifier = sid__662("id");
        let mut t___13539: SafeIdentifier = sid__662("name");
        let q__1542: Query = from(t___13537.clone()).select([t___13538.clone(), t___13539.clone()]);
        let mut t___13544: bool = Some(q__1542.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___117 {}
        impl ClosureGroup___117 {
            fn fn__13536(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___117 {};
        let fn__13536 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13536())
        };
        test___99.assert(t___13544, fn__13536.clone());
        test___99.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__2269() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___100 = temper_std::testing::Test::new();
        let mut t___13525: SafeIdentifier = sid__662("users");
        let mut t___13526: SqlBuilder = SqlBuilder::new();
        t___13526.append_safe("age > ");
        t___13526.append_int32(18);
        let mut t___13529: SqlFragment = t___13526.accumulated();
        let q__1544: Query = from(t___13525.clone()).r#where(t___13529.clone());
        let mut t___13534: bool = Some(q__1544.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___118 {}
        impl ClosureGroup___118 {
            fn fn__13524(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___118 {};
        let fn__13524 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13524())
        };
        test___100.assert(t___13534, fn__13524.clone());
        test___100.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__2271() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___101 = temper_std::testing::Test::new();
        let mut t___13513: SafeIdentifier = sid__662("users");
        let mut t___13514: SqlBuilder = SqlBuilder::new();
        t___13514.append_safe("active = ");
        t___13514.append_boolean(true);
        let mut t___13517: SqlFragment = t___13514.accumulated();
        let q__1546: Query = from(t___13513.clone()).r#where(t___13517.clone());
        let mut t___13522: bool = Some(q__1546.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___119 {}
        impl ClosureGroup___119 {
            fn fn__13512(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___119 {};
        let fn__13512 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13512())
        };
        test___101.assert(t___13522, fn__13512.clone());
        test___101.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__2273() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___102 = temper_std::testing::Test::new();
        let mut t___13496: SafeIdentifier = sid__662("users");
        let mut t___13497: SqlBuilder = SqlBuilder::new();
        t___13497.append_safe("age > ");
        t___13497.append_int32(18);
        let mut t___13500: SqlFragment = t___13497.accumulated();
        let mut t___13501: Query = from(t___13496.clone()).r#where(t___13500.clone());
        let mut t___13502: SqlBuilder = SqlBuilder::new();
        t___13502.append_safe("active = ");
        t___13502.append_boolean(true);
        let q__1548: Query = t___13501.r#where(t___13502.accumulated());
        let mut t___13510: bool = Some(q__1548.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___120 {}
        impl ClosureGroup___120 {
            fn fn__13495(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___120 {};
        let fn__13495 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13495())
        };
        test___102.assert(t___13510, fn__13495.clone());
        test___102.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__2276() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___103 = temper_std::testing::Test::new();
        let mut t___13487: SafeIdentifier = sid__662("users");
        let mut t___13488: SafeIdentifier = sid__662("name");
        let q__1550: Query = from(t___13487.clone()).order_by(t___13488.clone(), true);
        let mut t___13493: bool = Some(q__1550.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___121 {}
        impl ClosureGroup___121 {
            fn fn__13486(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___121 {};
        let fn__13486 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13486())
        };
        test___103.assert(t___13493, fn__13486.clone());
        test___103.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__2277() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___104 = temper_std::testing::Test::new();
        let mut t___13478: SafeIdentifier = sid__662("users");
        let mut t___13479: SafeIdentifier = sid__662("created_at");
        let q__1552: Query = from(t___13478.clone()).order_by(t___13479.clone(), false);
        let mut t___13484: bool = Some(q__1552.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___122 {}
        impl ClosureGroup___122 {
            fn fn__13477(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___122 {};
        let fn__13477 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13477())
        };
        test___104.assert(t___13484, fn__13477.clone());
        test___104.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__2278() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___105 = temper_std::testing::Test::new();
        let mut t___6899: Query;
        let mut t___6900: Query;
        let q__1554: Query;
        'ok___15623: {
            'orelse___2557: {
                t___6899 = match from(sid__662("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2557
                };
                t___6900 = match t___6899.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2557
                };
                q__1554 = t___6900.clone();
                break 'ok___15623;
            }
            q__1554 = panic!();
        }
        let mut t___13475: bool = Some(q__1554.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___123 {}
        impl ClosureGroup___123 {
            fn fn__13470(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___123 {};
        let fn__13470 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13470())
        };
        test___105.assert(t___13475, fn__13470.clone());
        test___105.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__2279() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___106 = temper_std::testing::Test::new();
        let didBubble__1556: bool;
        'ok___15624: {
            'orelse___2558: {
                match from(sid__662("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2558
                };
                didBubble__1556 = false;
                break 'ok___15624;
            }
            didBubble__1556 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___124 {}
        impl ClosureGroup___124 {
            fn fn__13466(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___124 {};
        let fn__13466 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13466())
        };
        test___106.assert(didBubble__1556, fn__13466.clone());
        test___106.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__2280() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___107 = temper_std::testing::Test::new();
        let didBubble__1558: bool;
        'ok___15625: {
            'orelse___2559: {
                match from(sid__662("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2559
                };
                didBubble__1558 = false;
                break 'ok___15625;
            }
            didBubble__1558 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___125 {}
        impl ClosureGroup___125 {
            fn fn__13462(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___125 {};
        let fn__13462 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13462())
        };
        test___107.assert(didBubble__1558, fn__13462.clone());
        test___107.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__2281() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___108 = temper_std::testing::Test::new();
        let mut t___13440: SafeIdentifier;
        let mut t___13441: SafeIdentifier;
        let mut t___13442: SafeIdentifier;
        let mut t___13443: SafeIdentifier;
        let mut t___13444: Query;
        let mut t___13445: SqlBuilder;
        let mut t___13449: Query;
        let mut t___13450: SqlBuilder;
        let mut t___6885: Query;
        let mut t___6886: Query;
        let minAge__1560: i32 = 21;
        let q__1561: Query;
        'ok___15626: {
            'orelse___2560: {
                t___13440 = sid__662("users");
                t___13441 = sid__662("id");
                t___13442 = sid__662("name");
                t___13443 = sid__662("email");
                t___13444 = from(t___13440.clone()).select([t___13441.clone(), t___13442.clone(), t___13443.clone()]);
                t___13445 = SqlBuilder::new();
                t___13445.append_safe("age >= ");
                t___13445.append_int32(21);
                t___13449 = t___13444.r#where(t___13445.accumulated());
                t___13450 = SqlBuilder::new();
                t___13450.append_safe("active = ");
                t___13450.append_boolean(true);
                t___6885 = match t___13449.r#where(t___13450.accumulated()).order_by(sid__662("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___2560
                };
                t___6886 = match t___6885.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___2560
                };
                q__1561 = t___6886.clone();
                break 'ok___15626;
            }
            q__1561 = panic!();
        }
        let mut t___13460: bool = Some(q__1561.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___126 {}
        impl ClosureGroup___126 {
            fn fn__13439(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___126 {};
        let fn__13439 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13439())
        };
        test___108.assert(t___13460, fn__13439.clone());
        test___108.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__2284() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___109 = temper_std::testing::Test::new();
        let mut t___6862: SqlFragment;
        let mut t___6863: SqlFragment;
        let q__1563: Query = from(sid__662("users"));
        'ok___15627: {
            'orelse___2561: {
                t___6862 = match q__1563.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2561
                };
                t___6863 = t___6862.clone();
                break 'ok___15627;
            }
            t___6863 = panic!();
        }
        let s__1564: std::sync::Arc<String> = t___6863.to_string();
        let mut t___13437: bool = Some(s__1564.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___127 {
            s__1564: std::sync::Arc<String>
        }
        impl ClosureGroup___127 {
            fn fn__13433(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__1564.clone()));
            }
        }
        let closure_group = ClosureGroup___127 {
            s__1564: s__1564.clone()
        };
        let fn__13433 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13433())
        };
        test___109.assert(t___13437, fn__13433.clone());
        test___109.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__2285() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___110 = temper_std::testing::Test::new();
        let mut t___6854: Query;
        let mut t___6857: SqlFragment;
        let mut t___6858: SqlFragment;
        let q__1566: Query;
        'ok___15628: {
            'orelse___2562: {
                t___6854 = match from(sid__662("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___2562
                };
                q__1566 = t___6854.clone();
                break 'ok___15628;
            }
            q__1566 = panic!();
        }
        'ok___15629: {
            'orelse___2563: {
                t___6857 = match q__1566.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2563
                };
                t___6858 = t___6857.clone();
                break 'ok___15629;
            }
            t___6858 = panic!();
        }
        let s__1567: std::sync::Arc<String> = t___6858.to_string();
        let mut t___13431: bool = Some(s__1567.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___128 {
            s__1567: std::sync::Arc<String>
        }
        impl ClosureGroup___128 {
            fn fn__13427(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__1567.clone()));
            }
        }
        let closure_group = ClosureGroup___128 {
            s__1567: s__1567.clone()
        };
        let fn__13427 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13427())
        };
        test___110.assert(t___13431, fn__13427.clone());
        test___110.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__2286() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___111 = temper_std::testing::Test::new();
        let didBubble__1569: bool;
        'ok___15630: {
            'orelse___2564: {
                match from(sid__662("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___2564
                };
                didBubble__1569 = false;
                break 'ok___15630;
            }
            didBubble__1569 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___129 {}
        impl ClosureGroup___129 {
            fn fn__13423(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___129 {};
        let fn__13423 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13423())
        };
        test___111.assert(didBubble__1569, fn__13423.clone());
        test___111.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__2287() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___112 = temper_std::testing::Test::new();
        let evil__1571: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___13407: SafeIdentifier = sid__662("users");
        let mut t___13408: SqlBuilder = SqlBuilder::new();
        t___13408.append_safe("name = ");
        t___13408.append_string("'; DROP TABLE users; --");
        let mut t___13411: SqlFragment = t___13408.accumulated();
        let q__1572: Query = from(t___13407.clone()).r#where(t___13411.clone());
        let s__1573: std::sync::Arc<String> = q__1572.to_sql().to_string();
        let mut t___13416: bool = temper_core::string::index_of( & s__1573, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___130 {
            s__1573: std::sync::Arc<String>
        }
        impl ClosureGroup___130 {
            fn fn__13406(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__1573.clone()));
            }
        }
        let closure_group = ClosureGroup___130 {
            s__1573: s__1573.clone()
        };
        let fn__13406 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13406())
        };
        test___112.assert(t___13416, fn__13406.clone());
        let mut t___13420: bool = temper_core::string::index_of( & s__1573, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___131 {
            s__1573: std::sync::Arc<String>
        }
        impl ClosureGroup___131 {
            fn fn__13405(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__1573.clone()));
            }
        }
        let closure_group = ClosureGroup___131 {
            s__1573: s__1573.clone()
        };
        let fn__13405 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13405())
        };
        test___112.assert(t___13420, fn__13405.clone());
        test___112.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__2289() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___113 = temper_std::testing::Test::new();
        let attack__1575: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__1576: bool;
        'ok___15631: {
            'orelse___2565: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___2565
                };
                didBubble__1576 = false;
                break 'ok___15631;
            }
            didBubble__1576 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___132 {}
        impl ClosureGroup___132 {
            fn fn__13402(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___132 {};
        let fn__13402 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13402())
        };
        test___113.assert(didBubble__1576, fn__13402.clone());
        test___113.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__2290() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___114 = temper_std::testing::Test::new();
        let mut t___13391: SafeIdentifier = sid__662("users");
        let mut t___13392: SafeIdentifier = sid__662("orders");
        let mut t___13393: SqlBuilder = SqlBuilder::new();
        t___13393.append_safe("users.id = orders.user_id");
        let mut t___13395: SqlFragment = t___13393.accumulated();
        let q__1578: Query = from(t___13391.clone()).inner_join(t___13392.clone(), t___13395.clone());
        let mut t___13400: bool = Some(q__1578.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___133 {}
        impl ClosureGroup___133 {
            fn fn__13390(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___133 {};
        let fn__13390 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13390())
        };
        test___114.assert(t___13400, fn__13390.clone());
        test___114.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__2292() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___115 = temper_std::testing::Test::new();
        let mut t___13379: SafeIdentifier = sid__662("users");
        let mut t___13380: SafeIdentifier = sid__662("profiles");
        let mut t___13381: SqlBuilder = SqlBuilder::new();
        t___13381.append_safe("users.id = profiles.user_id");
        let mut t___13383: SqlFragment = t___13381.accumulated();
        let q__1580: Query = from(t___13379.clone()).left_join(t___13380.clone(), t___13383.clone());
        let mut t___13388: bool = Some(q__1580.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___134 {}
        impl ClosureGroup___134 {
            fn fn__13378(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___134 {};
        let fn__13378 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13378())
        };
        test___115.assert(t___13388, fn__13378.clone());
        test___115.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__2294() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___116 = temper_std::testing::Test::new();
        let mut t___13367: SafeIdentifier = sid__662("orders");
        let mut t___13368: SafeIdentifier = sid__662("users");
        let mut t___13369: SqlBuilder = SqlBuilder::new();
        t___13369.append_safe("orders.user_id = users.id");
        let mut t___13371: SqlFragment = t___13369.accumulated();
        let q__1582: Query = from(t___13367.clone()).right_join(t___13368.clone(), t___13371.clone());
        let mut t___13376: bool = Some(q__1582.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___135 {}
        impl ClosureGroup___135 {
            fn fn__13366(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___135 {};
        let fn__13366 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13366())
        };
        test___116.assert(t___13376, fn__13366.clone());
        test___116.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__2296() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___117 = temper_std::testing::Test::new();
        let mut t___13355: SafeIdentifier = sid__662("users");
        let mut t___13356: SafeIdentifier = sid__662("orders");
        let mut t___13357: SqlBuilder = SqlBuilder::new();
        t___13357.append_safe("users.id = orders.user_id");
        let mut t___13359: SqlFragment = t___13357.accumulated();
        let q__1584: Query = from(t___13355.clone()).full_join(t___13356.clone(), t___13359.clone());
        let mut t___13364: bool = Some(q__1584.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___136 {}
        impl ClosureGroup___136 {
            fn fn__13354(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___136 {};
        let fn__13354 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13354())
        };
        test___117.assert(t___13364, fn__13354.clone());
        test___117.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__2298() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___118 = temper_std::testing::Test::new();
        let mut t___13338: SafeIdentifier = sid__662("users");
        let mut t___13339: SafeIdentifier = sid__662("orders");
        let mut t___13340: SqlBuilder = SqlBuilder::new();
        t___13340.append_safe("users.id = orders.user_id");
        let mut t___13342: SqlFragment = t___13340.accumulated();
        let mut t___13343: Query = from(t___13338.clone()).inner_join(t___13339.clone(), t___13342.clone());
        let mut t___13344: SafeIdentifier = sid__662("profiles");
        let mut t___13345: SqlBuilder = SqlBuilder::new();
        t___13345.append_safe("users.id = profiles.user_id");
        let q__1586: Query = t___13343.left_join(t___13344.clone(), t___13345.accumulated());
        let mut t___13352: bool = Some(q__1586.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___137 {}
        impl ClosureGroup___137 {
            fn fn__13337(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___137 {};
        let fn__13337 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13337())
        };
        test___118.assert(t___13352, fn__13337.clone());
        test___118.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__2301() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___119 = temper_std::testing::Test::new();
        let mut t___13319: SafeIdentifier;
        let mut t___13320: SafeIdentifier;
        let mut t___13321: SqlBuilder;
        let mut t___13323: SqlFragment;
        let mut t___13324: Query;
        let mut t___13325: SqlBuilder;
        let mut t___6769: Query;
        let q__1588: Query;
        'ok___15632: {
            'orelse___2566: {
                t___13319 = sid__662("users");
                t___13320 = sid__662("orders");
                t___13321 = SqlBuilder::new();
                t___13321.append_safe("users.id = orders.user_id");
                t___13323 = t___13321.accumulated();
                t___13324 = from(t___13319.clone()).inner_join(t___13320.clone(), t___13323.clone());
                t___13325 = SqlBuilder::new();
                t___13325.append_safe("orders.total > ");
                t___13325.append_int32(100);
                t___6769 = match t___13324.r#where(t___13325.accumulated()).order_by(sid__662("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2566
                };
                q__1588 = t___6769.clone();
                break 'ok___15632;
            }
            q__1588 = panic!();
        }
        let mut t___13335: bool = Some(q__1588.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___138 {}
        impl ClosureGroup___138 {
            fn fn__13318(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___138 {};
        let fn__13318 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13318())
        };
        test___119.assert(t___13335, fn__13318.clone());
        test___119.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__2304() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___120 = temper_std::testing::Test::new();
        let c__1590: SqlFragment = col(sid__662("users"), sid__662("id"));
        let mut t___13316: bool = Some(c__1590.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___139 {}
        impl ClosureGroup___139 {
            fn fn__13310(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___139 {};
        let fn__13310 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13310())
        };
        test___120.assert(t___13316, fn__13310.clone());
        test___120.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__2305() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___121 = temper_std::testing::Test::new();
        let onCond__1592: SqlFragment = col(sid__662("users"), sid__662("id"));
        let b__1593: SqlBuilder = SqlBuilder::new();
        b__1593.append_fragment(onCond__1592.clone());
        b__1593.append_safe(" = ");
        b__1593.append_fragment(col(sid__662("orders"), sid__662("user_id")));
        let mut t___13301: SafeIdentifier = sid__662("users");
        let mut t___13302: SafeIdentifier = sid__662("orders");
        let mut t___13303: SqlFragment = b__1593.accumulated();
        let q__1594: Query = from(t___13301.clone()).inner_join(t___13302.clone(), t___13303.clone());
        let mut t___13308: bool = Some(q__1594.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___140 {}
        impl ClosureGroup___140 {
            fn fn__13290(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___140 {};
        let fn__13290 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13290())
        };
        test___121.assert(t___13308, fn__13290.clone());
        test___121.soft_fail_to_hard()
    }
    #[test]
    fn orWhereBasic__2306() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___122 = temper_std::testing::Test::new();
        let mut t___13279: SafeIdentifier = sid__662("users");
        let mut t___13280: SqlBuilder = SqlBuilder::new();
        t___13280.append_safe("status = ");
        t___13280.append_string("active");
        let mut t___13283: SqlFragment = t___13280.accumulated();
        let q__1596: Query = from(t___13279.clone()).or_where(t___13283.clone());
        let mut t___13288: bool = Some(q__1596.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE status = 'active'");
        #[derive(Clone)]
        struct ClosureGroup___141 {}
        impl ClosureGroup___141 {
            fn fn__13278(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orWhere basic".to_string());
            }
        }
        let closure_group = ClosureGroup___141 {};
        let fn__13278 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13278())
        };
        test___122.assert(t___13288, fn__13278.clone());
        test___122.soft_fail_to_hard()
    }
    #[test]
    fn whereThenOrWhere__2308() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___123 = temper_std::testing::Test::new();
        let mut t___13262: SafeIdentifier = sid__662("users");
        let mut t___13263: SqlBuilder = SqlBuilder::new();
        t___13263.append_safe("age > ");
        t___13263.append_int32(18);
        let mut t___13266: SqlFragment = t___13263.accumulated();
        let mut t___13267: Query = from(t___13262.clone()).r#where(t___13266.clone());
        let mut t___13268: SqlBuilder = SqlBuilder::new();
        t___13268.append_safe("vip = ");
        t___13268.append_boolean(true);
        let q__1598: Query = t___13267.or_where(t___13268.accumulated());
        let mut t___13276: bool = Some(q__1598.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___142 {}
        impl ClosureGroup___142 {
            fn fn__13261(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where then orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___142 {};
        let fn__13261 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13261())
        };
        test___123.assert(t___13276, fn__13261.clone());
        test___123.soft_fail_to_hard()
    }
    #[test]
    fn multipleOrWhere__2311() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___124 = temper_std::testing::Test::new();
        let mut t___13240: SafeIdentifier = sid__662("users");
        let mut t___13241: SqlBuilder = SqlBuilder::new();
        t___13241.append_safe("active = ");
        t___13241.append_boolean(true);
        let mut t___13244: SqlFragment = t___13241.accumulated();
        let mut t___13245: Query = from(t___13240.clone()).r#where(t___13244.clone());
        let mut t___13246: SqlBuilder = SqlBuilder::new();
        t___13246.append_safe("role = ");
        t___13246.append_string("admin");
        let mut t___13250: Query = t___13245.or_where(t___13246.accumulated());
        let mut t___13251: SqlBuilder = SqlBuilder::new();
        t___13251.append_safe("role = ");
        t___13251.append_string("moderator");
        let q__1600: Query = t___13250.or_where(t___13251.accumulated());
        let mut t___13259: bool = Some(q__1600.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'");
        #[derive(Clone)]
        struct ClosureGroup___143 {}
        impl ClosureGroup___143 {
            fn fn__13239(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("multiple orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___143 {};
        let fn__13239 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13239())
        };
        test___124.assert(t___13259, fn__13239.clone());
        test___124.soft_fail_to_hard()
    }
    #[test]
    fn mixedWhereAndOrWhere__2315() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___125 = temper_std::testing::Test::new();
        let mut t___13218: SafeIdentifier = sid__662("users");
        let mut t___13219: SqlBuilder = SqlBuilder::new();
        t___13219.append_safe("age > ");
        t___13219.append_int32(18);
        let mut t___13222: SqlFragment = t___13219.accumulated();
        let mut t___13223: Query = from(t___13218.clone()).r#where(t___13222.clone());
        let mut t___13224: SqlBuilder = SqlBuilder::new();
        t___13224.append_safe("active = ");
        t___13224.append_boolean(true);
        let mut t___13228: Query = t___13223.r#where(t___13224.accumulated());
        let mut t___13229: SqlBuilder = SqlBuilder::new();
        t___13229.append_safe("vip = ");
        t___13229.append_boolean(true);
        let q__1602: Query = t___13228.or_where(t___13229.accumulated());
        let mut t___13237: bool = Some(q__1602.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___144 {}
        impl ClosureGroup___144 {
            fn fn__13217(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed where and orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___144 {};
        let fn__13217 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13217())
        };
        test___125.assert(t___13237, fn__13217.clone());
        test___125.soft_fail_to_hard()
    }
    #[test]
    fn whereNull__2319() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___126 = temper_std::testing::Test::new();
        let mut t___13209: SafeIdentifier = sid__662("users");
        let mut t___13210: SafeIdentifier = sid__662("deleted_at");
        let q__1604: Query = from(t___13209.clone()).where_null(t___13210.clone());
        let mut t___13215: bool = Some(q__1604.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___145 {}
        impl ClosureGroup___145 {
            fn fn__13208(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull".to_string());
            }
        }
        let closure_group = ClosureGroup___145 {};
        let fn__13208 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13208())
        };
        test___126.assert(t___13215, fn__13208.clone());
        test___126.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNull__2320() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___127 = temper_std::testing::Test::new();
        let mut t___13200: SafeIdentifier = sid__662("users");
        let mut t___13201: SafeIdentifier = sid__662("email");
        let q__1606: Query = from(t___13200.clone()).where_not_null(t___13201.clone());
        let mut t___13206: bool = Some(q__1606.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email IS NOT NULL");
        #[derive(Clone)]
        struct ClosureGroup___146 {}
        impl ClosureGroup___146 {
            fn fn__13199(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull".to_string());
            }
        }
        let closure_group = ClosureGroup___146 {};
        let fn__13199 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13199())
        };
        test___127.assert(t___13206, fn__13199.clone());
        test___127.soft_fail_to_hard()
    }
    #[test]
    fn whereNullChainedWithWhere__2321() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___128 = temper_std::testing::Test::new();
        let mut t___13186: SafeIdentifier = sid__662("users");
        let mut t___13187: SqlBuilder = SqlBuilder::new();
        t___13187.append_safe("active = ");
        t___13187.append_boolean(true);
        let mut t___13190: SqlFragment = t___13187.accumulated();
        let q__1608: Query = from(t___13186.clone()).r#where(t___13190.clone()).where_null(sid__662("deleted_at"));
        let mut t___13197: bool = Some(q__1608.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___147 {}
        impl ClosureGroup___147 {
            fn fn__13185(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull chained".to_string());
            }
        }
        let closure_group = ClosureGroup___147 {};
        let fn__13185 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13185())
        };
        test___128.assert(t___13197, fn__13185.clone());
        test___128.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNullChainedWithOrWhere__2323() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___129 = temper_std::testing::Test::new();
        let mut t___13172: SafeIdentifier = sid__662("users");
        let mut t___13173: SafeIdentifier = sid__662("deleted_at");
        let mut t___13174: Query = from(t___13172.clone()).where_null(t___13173.clone());
        let mut t___13175: SqlBuilder = SqlBuilder::new();
        t___13175.append_safe("role = ");
        t___13175.append_string("admin");
        let q__1610: Query = t___13174.or_where(t___13175.accumulated());
        let mut t___13183: bool = Some(q__1610.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'");
        #[derive(Clone)]
        struct ClosureGroup___148 {}
        impl ClosureGroup___148 {
            fn fn__13171(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull with orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___148 {};
        let fn__13171 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13171())
        };
        test___129.assert(t___13183, fn__13171.clone());
        test___129.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithIntValues__2325() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___130 = temper_std::testing::Test::new();
        let mut t___13160: SafeIdentifier = sid__662("users");
        let mut t___13161: SafeIdentifier = sid__662("id");
        let mut t___13162: SqlInt32 = SqlInt32::new(1);
        let mut t___13163: SqlInt32 = SqlInt32::new(2);
        let mut t___13164: SqlInt32 = SqlInt32::new(3);
        let q__1612: Query = from(t___13160.clone()).where_in(t___13161.clone(), [SqlPart::new(t___13162.clone()), SqlPart::new(t___13163.clone()), SqlPart::new(t___13164.clone())]);
        let mut t___13169: bool = Some(q__1612.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___149 {}
        impl ClosureGroup___149 {
            fn fn__13159(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn ints".to_string());
            }
        }
        let closure_group = ClosureGroup___149 {};
        let fn__13159 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13159())
        };
        test___130.assert(t___13169, fn__13159.clone());
        test___130.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithStringValuesEscaping__2326() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___131 = temper_std::testing::Test::new();
        let mut t___13149: SafeIdentifier = sid__662("users");
        let mut t___13150: SafeIdentifier = sid__662("name");
        let mut t___13151: SqlString = SqlString::new("Alice");
        let mut t___13152: SqlString = SqlString::new("Bob's");
        let q__1614: Query = from(t___13149.clone()).where_in(t___13150.clone(), [SqlPart::new(t___13151.clone()), SqlPart::new(t___13152.clone())]);
        let mut t___13157: bool = Some(q__1614.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')");
        #[derive(Clone)]
        struct ClosureGroup___150 {}
        impl ClosureGroup___150 {
            fn fn__13148(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn strings".to_string());
            }
        }
        let closure_group = ClosureGroup___150 {};
        let fn__13148 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13148())
        };
        test___131.assert(t___13157, fn__13148.clone());
        test___131.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithEmptyListProduces1_0__2327() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___132 = temper_std::testing::Test::new();
        let mut t___13140: SafeIdentifier = sid__662("users");
        let mut t___13141: SafeIdentifier = sid__662("id");
        let q__1616: Query = from(t___13140.clone()).where_in(t___13141.clone(), []);
        let mut t___13146: bool = Some(q__1616.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___151 {}
        impl ClosureGroup___151 {
            fn fn__13139(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn empty".to_string());
            }
        }
        let closure_group = ClosureGroup___151 {};
        let fn__13139 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13139())
        };
        test___132.assert(t___13146, fn__13139.clone());
        test___132.soft_fail_to_hard()
    }
    #[test]
    fn whereInChained__2328() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___133 = temper_std::testing::Test::new();
        let mut t___13124: SafeIdentifier = sid__662("users");
        let mut t___13125: SqlBuilder = SqlBuilder::new();
        t___13125.append_safe("active = ");
        t___13125.append_boolean(true);
        let mut t___13128: SqlFragment = t___13125.accumulated();
        let q__1618: Query = from(t___13124.clone()).r#where(t___13128.clone()).where_in(sid__662("role"), [SqlPart::new(SqlString::new("admin")), SqlPart::new(SqlString::new("user"))]);
        let mut t___13137: bool = Some(q__1618.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')");
        #[derive(Clone)]
        struct ClosureGroup___152 {}
        impl ClosureGroup___152 {
            fn fn__13123(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn chained".to_string());
            }
        }
        let closure_group = ClosureGroup___152 {};
        let fn__13123 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13123())
        };
        test___133.assert(t___13137, fn__13123.clone());
        test___133.soft_fail_to_hard()
    }
    #[test]
    fn whereInSingleElement__2330() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___134 = temper_std::testing::Test::new();
        let mut t___13114: SafeIdentifier = sid__662("users");
        let mut t___13115: SafeIdentifier = sid__662("id");
        let mut t___13116: SqlInt32 = SqlInt32::new(42);
        let q__1620: Query = from(t___13114.clone()).where_in(t___13115.clone(), [SqlPart::new(t___13116.clone())]);
        let mut t___13121: bool = Some(q__1620.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___153 {}
        impl ClosureGroup___153 {
            fn fn__13113(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn single".to_string());
            }
        }
        let closure_group = ClosureGroup___153 {};
        let fn__13113 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13113())
        };
        test___134.assert(t___13121, fn__13113.clone());
        test___134.soft_fail_to_hard()
    }
    #[test]
    fn whereNotBasic__2331() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___135 = temper_std::testing::Test::new();
        let mut t___13102: SafeIdentifier = sid__662("users");
        let mut t___13103: SqlBuilder = SqlBuilder::new();
        t___13103.append_safe("active = ");
        t___13103.append_boolean(true);
        let mut t___13106: SqlFragment = t___13103.accumulated();
        let q__1622: Query = from(t___13102.clone()).where_not(t___13106.clone());
        let mut t___13111: bool = Some(q__1622.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE NOT (active = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___154 {}
        impl ClosureGroup___154 {
            fn fn__13101(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot".to_string());
            }
        }
        let closure_group = ClosureGroup___154 {};
        let fn__13101 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13101())
        };
        test___135.assert(t___13111, fn__13101.clone());
        test___135.soft_fail_to_hard()
    }
    #[test]
    fn whereNotChained__2333() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___136 = temper_std::testing::Test::new();
        let mut t___13085: SafeIdentifier = sid__662("users");
        let mut t___13086: SqlBuilder = SqlBuilder::new();
        t___13086.append_safe("age > ");
        t___13086.append_int32(18);
        let mut t___13089: SqlFragment = t___13086.accumulated();
        let mut t___13090: Query = from(t___13085.clone()).r#where(t___13089.clone());
        let mut t___13091: SqlBuilder = SqlBuilder::new();
        t___13091.append_safe("banned = ");
        t___13091.append_boolean(true);
        let q__1624: Query = t___13090.where_not(t___13091.accumulated());
        let mut t___13099: bool = Some(q__1624.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___155 {}
        impl ClosureGroup___155 {
            fn fn__13084(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot chained".to_string());
            }
        }
        let closure_group = ClosureGroup___155 {};
        let fn__13084 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13084())
        };
        test___136.assert(t___13099, fn__13084.clone());
        test___136.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenIntegers__2336() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___137 = temper_std::testing::Test::new();
        let mut t___13074: SafeIdentifier = sid__662("users");
        let mut t___13075: SafeIdentifier = sid__662("age");
        let mut t___13076: SqlInt32 = SqlInt32::new(18);
        let mut t___13077: SqlInt32 = SqlInt32::new(65);
        let q__1626: Query = from(t___13074.clone()).where_between(t___13075.clone(), SqlPart::new(t___13076.clone()), SqlPart::new(t___13077.clone()));
        let mut t___13082: bool = Some(q__1626.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age BETWEEN 18 AND 65");
        #[derive(Clone)]
        struct ClosureGroup___156 {}
        impl ClosureGroup___156 {
            fn fn__13073(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween ints".to_string());
            }
        }
        let closure_group = ClosureGroup___156 {};
        let fn__13073 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13073())
        };
        test___137.assert(t___13082, fn__13073.clone());
        test___137.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenChained__2337() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___138 = temper_std::testing::Test::new();
        let mut t___13058: SafeIdentifier = sid__662("users");
        let mut t___13059: SqlBuilder = SqlBuilder::new();
        t___13059.append_safe("active = ");
        t___13059.append_boolean(true);
        let mut t___13062: SqlFragment = t___13059.accumulated();
        let q__1628: Query = from(t___13058.clone()).r#where(t___13062.clone()).where_between(sid__662("age"), SqlPart::new(SqlInt32::new(21)), SqlPart::new(SqlInt32::new(30)));
        let mut t___13071: bool = Some(q__1628.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30");
        #[derive(Clone)]
        struct ClosureGroup___157 {}
        impl ClosureGroup___157 {
            fn fn__13057(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween chained".to_string());
            }
        }
        let closure_group = ClosureGroup___157 {};
        let fn__13057 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13057())
        };
        test___138.assert(t___13071, fn__13057.clone());
        test___138.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeBasic__2339() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___139 = temper_std::testing::Test::new();
        let mut t___13049: SafeIdentifier = sid__662("users");
        let mut t___13050: SafeIdentifier = sid__662("name");
        let q__1630: Query = from(t___13049.clone()).where_like(t___13050.clone(), "John%");
        let mut t___13055: bool = Some(q__1630.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE 'John%'");
        #[derive(Clone)]
        struct ClosureGroup___158 {}
        impl ClosureGroup___158 {
            fn fn__13048(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike".to_string());
            }
        }
        let closure_group = ClosureGroup___158 {};
        let fn__13048 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13048())
        };
        test___139.assert(t___13055, fn__13048.clone());
        test___139.soft_fail_to_hard()
    }
    #[test]
    fn whereIlikeBasic__2340() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___140 = temper_std::testing::Test::new();
        let mut t___13040: SafeIdentifier = sid__662("users");
        let mut t___13041: SafeIdentifier = sid__662("email");
        let q__1632: Query = from(t___13040.clone()).where_i_like(t___13041.clone(), "%@gmail.com");
        let mut t___13046: bool = Some(q__1632.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email ILIKE '%@gmail.com'");
        #[derive(Clone)]
        struct ClosureGroup___159 {}
        impl ClosureGroup___159 {
            fn fn__13039(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereILike".to_string());
            }
        }
        let closure_group = ClosureGroup___159 {};
        let fn__13039 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13039())
        };
        test___140.assert(t___13046, fn__13039.clone());
        test___140.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWithInjectionAttempt__2341() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___141 = temper_std::testing::Test::new();
        let mut t___13026: SafeIdentifier = sid__662("users");
        let mut t___13027: SafeIdentifier = sid__662("name");
        let q__1634: Query = from(t___13026.clone()).where_like(t___13027.clone(), "'; DROP TABLE users; --");
        let s__1635: std::sync::Arc<String> = q__1634.to_sql().to_string();
        let mut t___13032: bool = temper_core::string::index_of( & s__1635, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___160 {
            s__1635: std::sync::Arc<String>
        }
        impl ClosureGroup___160 {
            fn fn__13025(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like injection escaped: {}", self.s__1635.clone()));
            }
        }
        let closure_group = ClosureGroup___160 {
            s__1635: s__1635.clone()
        };
        let fn__13025 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13025())
        };
        test___141.assert(t___13032, fn__13025.clone());
        let mut t___13036: bool = temper_core::string::index_of( & s__1635, "LIKE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___161 {
            s__1635: std::sync::Arc<String>
        }
        impl ClosureGroup___161 {
            fn fn__13024(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like structure intact: {}", self.s__1635.clone()));
            }
        }
        let closure_group = ClosureGroup___161 {
            s__1635: s__1635.clone()
        };
        let fn__13024 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13024())
        };
        test___141.assert(t___13036, fn__13024.clone());
        test___141.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWildcardPatterns__2342() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___142 = temper_std::testing::Test::new();
        let mut t___13016: SafeIdentifier = sid__662("users");
        let mut t___13017: SafeIdentifier = sid__662("name");
        let q__1637: Query = from(t___13016.clone()).where_like(t___13017.clone(), "%son%");
        let mut t___13022: bool = Some(q__1637.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE '%son%'");
        #[derive(Clone)]
        struct ClosureGroup___162 {}
        impl ClosureGroup___162 {
            fn fn__13015(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike wildcard".to_string());
            }
        }
        let closure_group = ClosureGroup___162 {};
        let fn__13015 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13015())
        };
        test___142.assert(t___13022, fn__13015.clone());
        test___142.soft_fail_to_hard()
    }
    #[test]
    fn countAllProducesCount__2343() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___143 = temper_std::testing::Test::new();
        let f__1639: SqlFragment = count_all();
        let mut t___13013: bool = Some(f__1639.to_string().as_str()) == Some("COUNT(*)");
        #[derive(Clone)]
        struct ClosureGroup___163 {}
        impl ClosureGroup___163 {
            fn fn__13009(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countAll".to_string());
            }
        }
        let closure_group = ClosureGroup___163 {};
        let fn__13009 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13009())
        };
        test___143.assert(t___13013, fn__13009.clone());
        test___143.soft_fail_to_hard()
    }
    #[test]
    fn countColProducesCountField__2344() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___144 = temper_std::testing::Test::new();
        let f__1641: SqlFragment = count_col(sid__662("id"));
        let mut t___13007: bool = Some(f__1641.to_string().as_str()) == Some("COUNT(id)");
        #[derive(Clone)]
        struct ClosureGroup___164 {}
        impl ClosureGroup___164 {
            fn fn__13002(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countCol".to_string());
            }
        }
        let closure_group = ClosureGroup___164 {};
        let fn__13002 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__13002())
        };
        test___144.assert(t___13007, fn__13002.clone());
        test___144.soft_fail_to_hard()
    }
    #[test]
    fn sumColProducesSumField__2345() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___145 = temper_std::testing::Test::new();
        let f__1643: SqlFragment = sum_col(sid__662("amount"));
        let mut t___13000: bool = Some(f__1643.to_string().as_str()) == Some("SUM(amount)");
        #[derive(Clone)]
        struct ClosureGroup___165 {}
        impl ClosureGroup___165 {
            fn fn__12995(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("sumCol".to_string());
            }
        }
        let closure_group = ClosureGroup___165 {};
        let fn__12995 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12995())
        };
        test___145.assert(t___13000, fn__12995.clone());
        test___145.soft_fail_to_hard()
    }
    #[test]
    fn avgColProducesAvgField__2346() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___146 = temper_std::testing::Test::new();
        let f__1645: SqlFragment = avg_col(sid__662("price"));
        let mut t___12993: bool = Some(f__1645.to_string().as_str()) == Some("AVG(price)");
        #[derive(Clone)]
        struct ClosureGroup___166 {}
        impl ClosureGroup___166 {
            fn fn__12988(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("avgCol".to_string());
            }
        }
        let closure_group = ClosureGroup___166 {};
        let fn__12988 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12988())
        };
        test___146.assert(t___12993, fn__12988.clone());
        test___146.soft_fail_to_hard()
    }
    #[test]
    fn minColProducesMinField__2347() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___147 = temper_std::testing::Test::new();
        let f__1647: SqlFragment = min_col(sid__662("created_at"));
        let mut t___12986: bool = Some(f__1647.to_string().as_str()) == Some("MIN(created_at)");
        #[derive(Clone)]
        struct ClosureGroup___167 {}
        impl ClosureGroup___167 {
            fn fn__12981(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("minCol".to_string());
            }
        }
        let closure_group = ClosureGroup___167 {};
        let fn__12981 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12981())
        };
        test___147.assert(t___12986, fn__12981.clone());
        test___147.soft_fail_to_hard()
    }
    #[test]
    fn maxColProducesMaxField__2348() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___148 = temper_std::testing::Test::new();
        let f__1649: SqlFragment = max_col(sid__662("score"));
        let mut t___12979: bool = Some(f__1649.to_string().as_str()) == Some("MAX(score)");
        #[derive(Clone)]
        struct ClosureGroup___168 {}
        impl ClosureGroup___168 {
            fn fn__12974(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("maxCol".to_string());
            }
        }
        let closure_group = ClosureGroup___168 {};
        let fn__12974 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12974())
        };
        test___148.assert(t___12979, fn__12974.clone());
        test___148.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithAggregate__2349() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___149 = temper_std::testing::Test::new();
        let mut t___12966: SafeIdentifier = sid__662("orders");
        let mut t___12967: SqlFragment = count_all();
        let q__1651: Query = from(t___12966.clone()).select_expr([t___12967.clone()]);
        let mut t___12972: bool = Some(q__1651.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM orders");
        #[derive(Clone)]
        struct ClosureGroup___169 {}
        impl ClosureGroup___169 {
            fn fn__12965(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr count".to_string());
            }
        }
        let closure_group = ClosureGroup___169 {};
        let fn__12965 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12965())
        };
        test___149.assert(t___12972, fn__12965.clone());
        test___149.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithMultipleExpressions__2350() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___150 = temper_std::testing::Test::new();
        let nameFrag__1653: SqlFragment = col(sid__662("users"), sid__662("name"));
        let mut t___12957: SafeIdentifier = sid__662("users");
        let mut t___12958: SqlFragment = count_all();
        let q__1654: Query = from(t___12957.clone()).select_expr([nameFrag__1653.clone(), t___12958.clone()]);
        let mut t___12963: bool = Some(q__1654.to_sql().to_string().as_str()) == Some("SELECT users.name, COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___170 {}
        impl ClosureGroup___170 {
            fn fn__12953(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr multi".to_string());
            }
        }
        let closure_group = ClosureGroup___170 {};
        let fn__12953 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12953())
        };
        test___150.assert(t___12963, fn__12953.clone());
        test___150.soft_fail_to_hard()
    }
    #[test]
    fn selectExprOverridesSelectedFields__2351() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___151 = temper_std::testing::Test::new();
        let mut t___12942: SafeIdentifier = sid__662("users");
        let mut t___12943: SafeIdentifier = sid__662("id");
        let mut t___12944: SafeIdentifier = sid__662("name");
        let q__1656: Query = from(t___12942.clone()).select([t___12943.clone(), t___12944.clone()]).select_expr([count_all()]);
        let mut t___12951: bool = Some(q__1656.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___171 {}
        impl ClosureGroup___171 {
            fn fn__12941(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr overrides select".to_string());
            }
        }
        let closure_group = ClosureGroup___171 {};
        let fn__12941 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12941())
        };
        test___151.assert(t___12951, fn__12941.clone());
        test___151.soft_fail_to_hard()
    }
    #[test]
    fn groupBySingleField__2352() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___152 = temper_std::testing::Test::new();
        let mut t___12928: SafeIdentifier = sid__662("orders");
        let mut t___12931: SqlFragment = col(sid__662("orders"), sid__662("status"));
        let mut t___12932: SqlFragment = count_all();
        let q__1658: Query = from(t___12928.clone()).select_expr([t___12931.clone(), t___12932.clone()]).group_by(sid__662("status"));
        let mut t___12939: bool = Some(q__1658.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status");
        #[derive(Clone)]
        struct ClosureGroup___172 {}
        impl ClosureGroup___172 {
            fn fn__12927(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy single".to_string());
            }
        }
        let closure_group = ClosureGroup___172 {};
        let fn__12927 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12927())
        };
        test___152.assert(t___12939, fn__12927.clone());
        test___152.soft_fail_to_hard()
    }
    #[test]
    fn groupByMultipleFields__2353() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___153 = temper_std::testing::Test::new();
        let mut t___12917: SafeIdentifier = sid__662("orders");
        let mut t___12918: SafeIdentifier = sid__662("status");
        let q__1660: Query = from(t___12917.clone()).group_by(t___12918.clone()).group_by(sid__662("category"));
        let mut t___12925: bool = Some(q__1660.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status, category");
        #[derive(Clone)]
        struct ClosureGroup___173 {}
        impl ClosureGroup___173 {
            fn fn__12916(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy multiple".to_string());
            }
        }
        let closure_group = ClosureGroup___173 {};
        let fn__12916 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12916())
        };
        test___153.assert(t___12925, fn__12916.clone());
        test___153.soft_fail_to_hard()
    }
    #[test]
    fn havingBasic__2354() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___154 = temper_std::testing::Test::new();
        let mut t___12898: SafeIdentifier = sid__662("orders");
        let mut t___12901: SqlFragment = col(sid__662("orders"), sid__662("status"));
        let mut t___12902: SqlFragment = count_all();
        let mut t___12905: Query = from(t___12898.clone()).select_expr([t___12901.clone(), t___12902.clone()]).group_by(sid__662("status"));
        let mut t___12906: SqlBuilder = SqlBuilder::new();
        t___12906.append_safe("COUNT(*) > ");
        t___12906.append_int32(5);
        let q__1662: Query = t___12905.having(t___12906.accumulated());
        let mut t___12914: bool = Some(q__1662.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status HAVING COUNT(*) > 5");
        #[derive(Clone)]
        struct ClosureGroup___174 {}
        impl ClosureGroup___174 {
            fn fn__12897(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("having basic".to_string());
            }
        }
        let closure_group = ClosureGroup___174 {};
        let fn__12897 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12897())
        };
        test___154.assert(t___12914, fn__12897.clone());
        test___154.soft_fail_to_hard()
    }
    #[test]
    fn orHaving__2356() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___155 = temper_std::testing::Test::new();
        let mut t___12879: SafeIdentifier = sid__662("orders");
        let mut t___12880: SafeIdentifier = sid__662("status");
        let mut t___12881: Query = from(t___12879.clone()).group_by(t___12880.clone());
        let mut t___12882: SqlBuilder = SqlBuilder::new();
        t___12882.append_safe("COUNT(*) > ");
        t___12882.append_int32(5);
        let mut t___12886: Query = t___12881.having(t___12882.accumulated());
        let mut t___12887: SqlBuilder = SqlBuilder::new();
        t___12887.append_safe("SUM(total) > ");
        t___12887.append_int32(1000);
        let q__1664: Query = t___12886.or_having(t___12887.accumulated());
        let mut t___12895: bool = Some(q__1664.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status HAVING COUNT(*) > 5 OR SUM(total) > 1000");
        #[derive(Clone)]
        struct ClosureGroup___175 {}
        impl ClosureGroup___175 {
            fn fn__12878(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orHaving".to_string());
            }
        }
        let closure_group = ClosureGroup___175 {};
        let fn__12878 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12878())
        };
        test___155.assert(t___12895, fn__12878.clone());
        test___155.soft_fail_to_hard()
    }
    #[test]
    fn distinctBasic__2359() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___156 = temper_std::testing::Test::new();
        let mut t___12869: SafeIdentifier = sid__662("users");
        let mut t___12870: SafeIdentifier = sid__662("name");
        let q__1666: Query = from(t___12869.clone()).select([t___12870.clone()]).distinct();
        let mut t___12876: bool = Some(q__1666.to_sql().to_string().as_str()) == Some("SELECT DISTINCT name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___176 {}
        impl ClosureGroup___176 {
            fn fn__12868(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct".to_string());
            }
        }
        let closure_group = ClosureGroup___176 {};
        let fn__12868 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12868())
        };
        test___156.assert(t___12876, fn__12868.clone());
        test___156.soft_fail_to_hard()
    }
    #[test]
    fn distinctWithWhere__2360() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___157 = temper_std::testing::Test::new();
        let mut t___12854: SafeIdentifier = sid__662("users");
        let mut t___12855: SafeIdentifier = sid__662("email");
        let mut t___12856: Query = from(t___12854.clone()).select([t___12855.clone()]);
        let mut t___12857: SqlBuilder = SqlBuilder::new();
        t___12857.append_safe("active = ");
        t___12857.append_boolean(true);
        let q__1668: Query = t___12856.r#where(t___12857.accumulated()).distinct();
        let mut t___12866: bool = Some(q__1668.to_sql().to_string().as_str()) == Some("SELECT DISTINCT email FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___177 {}
        impl ClosureGroup___177 {
            fn fn__12853(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct with where".to_string());
            }
        }
        let closure_group = ClosureGroup___177 {};
        let fn__12853 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12853())
        };
        test___157.assert(t___12866, fn__12853.clone());
        test___157.soft_fail_to_hard()
    }
    #[test]
    fn countSqlBare__2362() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___158 = temper_std::testing::Test::new();
        let q__1670: Query = from(sid__662("users"));
        let mut t___12851: bool = Some(q__1670.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___178 {}
        impl ClosureGroup___178 {
            fn fn__12846(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql bare".to_string());
            }
        }
        let closure_group = ClosureGroup___178 {};
        let fn__12846 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12846())
        };
        test___158.assert(t___12851, fn__12846.clone());
        test___158.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithWhere__2363() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___159 = temper_std::testing::Test::new();
        let mut t___12835: SafeIdentifier = sid__662("users");
        let mut t___12836: SqlBuilder = SqlBuilder::new();
        t___12836.append_safe("active = ");
        t___12836.append_boolean(true);
        let mut t___12839: SqlFragment = t___12836.accumulated();
        let q__1672: Query = from(t___12835.clone()).r#where(t___12839.clone());
        let mut t___12844: bool = Some(q__1672.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___179 {}
        impl ClosureGroup___179 {
            fn fn__12834(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with where".to_string());
            }
        }
        let closure_group = ClosureGroup___179 {};
        let fn__12834 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12834())
        };
        test___159.assert(t___12844, fn__12834.clone());
        test___159.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithJoin__2365() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___160 = temper_std::testing::Test::new();
        let mut t___12818: SafeIdentifier = sid__662("users");
        let mut t___12819: SafeIdentifier = sid__662("orders");
        let mut t___12820: SqlBuilder = SqlBuilder::new();
        t___12820.append_safe("users.id = orders.user_id");
        let mut t___12822: SqlFragment = t___12820.accumulated();
        let mut t___12823: Query = from(t___12818.clone()).inner_join(t___12819.clone(), t___12822.clone());
        let mut t___12824: SqlBuilder = SqlBuilder::new();
        t___12824.append_safe("orders.total > ");
        t___12824.append_int32(100);
        let q__1674: Query = t___12823.r#where(t___12824.accumulated());
        let mut t___12832: bool = Some(q__1674.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100");
        #[derive(Clone)]
        struct ClosureGroup___180 {}
        impl ClosureGroup___180 {
            fn fn__12817(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with join".to_string());
            }
        }
        let closure_group = ClosureGroup___180 {};
        let fn__12817 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12817())
        };
        test___160.assert(t___12832, fn__12817.clone());
        test___160.soft_fail_to_hard()
    }
    #[test]
    fn countSqlDropsOrderByLimitOffset__2368() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___161 = temper_std::testing::Test::new();
        let mut t___12804: SafeIdentifier;
        let mut t___12805: SqlBuilder;
        let mut t___12808: SqlFragment;
        let mut t___6345: Query;
        let mut t___6346: Query;
        let q__1676: Query;
        'ok___15633: {
            'orelse___2567: {
                t___12804 = sid__662("users");
                t___12805 = SqlBuilder::new();
                t___12805.append_safe("active = ");
                t___12805.append_boolean(true);
                t___12808 = t___12805.accumulated();
                t___6345 = match from(t___12804.clone()).r#where(t___12808.clone()).order_by(sid__662("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___2567
                };
                t___6346 = match t___6345.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___2567
                };
                q__1676 = t___6346.clone();
                break 'ok___15633;
            }
            q__1676 = panic!();
        }
        let s__1677: std::sync::Arc<String> = q__1676.count_sql().to_string();
        let mut t___12815: bool = Some(s__1677.as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___181 {
            s__1677: std::sync::Arc<String>
        }
        impl ClosureGroup___181 {
            fn fn__12803(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("countSql drops extras: {}", self.s__1677.clone()));
            }
        }
        let closure_group = ClosureGroup___181 {
            s__1677: s__1677.clone()
        };
        let fn__12803 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12803())
        };
        test___161.assert(t___12815, fn__12803.clone());
        test___161.soft_fail_to_hard()
    }
    #[test]
    fn fullAggregationQuery__2370() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___162 = temper_std::testing::Test::new();
        let mut t___12771: SafeIdentifier = sid__662("orders");
        let mut t___12774: SqlFragment = col(sid__662("orders"), sid__662("status"));
        let mut t___12775: SqlFragment = count_all();
        let mut t___12777: SqlFragment = sum_col(sid__662("total"));
        let mut t___12778: Query = from(t___12771.clone()).select_expr([t___12774.clone(), t___12775.clone(), t___12777.clone()]);
        let mut t___12779: SafeIdentifier = sid__662("users");
        let mut t___12780: SqlBuilder = SqlBuilder::new();
        t___12780.append_safe("orders.user_id = users.id");
        let mut t___12783: Query = t___12778.inner_join(t___12779.clone(), t___12780.accumulated());
        let mut t___12784: SqlBuilder = SqlBuilder::new();
        t___12784.append_safe("users.active = ");
        t___12784.append_boolean(true);
        let mut t___12790: Query = t___12783.r#where(t___12784.accumulated()).group_by(sid__662("status"));
        let mut t___12791: SqlBuilder = SqlBuilder::new();
        t___12791.append_safe("COUNT(*) > ");
        t___12791.append_int32(3);
        let q__1679: Query = t___12790.having(t___12791.accumulated()).order_by(sid__662("status"), true);
        let expected__1680: std::sync::Arc<String> = std::sync::Arc::new("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC".to_string());
        let mut t___12801: bool = Some(q__1679.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC");
        #[derive(Clone)]
        struct ClosureGroup___182 {}
        impl ClosureGroup___182 {
            fn fn__12770(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full aggregation".to_string());
            }
        }
        let closure_group = ClosureGroup___182 {};
        let fn__12770 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12770())
        };
        test___162.assert(t___12801, fn__12770.clone());
        test___162.soft_fail_to_hard()
    }
    #[test]
    fn unionSql__2374() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___163 = temper_std::testing::Test::new();
        let mut t___12753: SafeIdentifier = sid__662("users");
        let mut t___12754: SqlBuilder = SqlBuilder::new();
        t___12754.append_safe("role = ");
        t___12754.append_string("admin");
        let mut t___12757: SqlFragment = t___12754.accumulated();
        let a__1682: Query = from(t___12753.clone()).r#where(t___12757.clone());
        let mut t___12759: SafeIdentifier = sid__662("users");
        let mut t___12760: SqlBuilder = SqlBuilder::new();
        t___12760.append_safe("role = ");
        t___12760.append_string("moderator");
        let mut t___12763: SqlFragment = t___12760.accumulated();
        let b__1683: Query = from(t___12759.clone()).r#where(t___12763.clone());
        let s__1684: std::sync::Arc<String> = union_sql(a__1682.clone(), b__1683.clone()).to_string();
        let mut t___12768: bool = Some(s__1684.as_str()) == Some("(SELECT * FROM users WHERE role = 'admin') UNION (SELECT * FROM users WHERE role = 'moderator')");
        #[derive(Clone)]
        struct ClosureGroup___183 {
            s__1684: std::sync::Arc<String>
        }
        impl ClosureGroup___183 {
            fn fn__12752(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionSql: {}", self.s__1684.clone()));
            }
        }
        let closure_group = ClosureGroup___183 {
            s__1684: s__1684.clone()
        };
        let fn__12752 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12752())
        };
        test___163.assert(t___12768, fn__12752.clone());
        test___163.soft_fail_to_hard()
    }
    #[test]
    fn unionAllSql__2377() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___164 = temper_std::testing::Test::new();
        let mut t___12741: SafeIdentifier = sid__662("users");
        let mut t___12742: SafeIdentifier = sid__662("name");
        let a__1686: Query = from(t___12741.clone()).select([t___12742.clone()]);
        let mut t___12744: SafeIdentifier = sid__662("contacts");
        let mut t___12745: SafeIdentifier = sid__662("name");
        let b__1687: Query = from(t___12744.clone()).select([t___12745.clone()]);
        let s__1688: std::sync::Arc<String> = union_all_sql(a__1686.clone(), b__1687.clone()).to_string();
        let mut t___12750: bool = Some(s__1688.as_str()) == Some("(SELECT name FROM users) UNION ALL (SELECT name FROM contacts)");
        #[derive(Clone)]
        struct ClosureGroup___184 {
            s__1688: std::sync::Arc<String>
        }
        impl ClosureGroup___184 {
            fn fn__12740(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionAllSql: {}", self.s__1688.clone()));
            }
        }
        let closure_group = ClosureGroup___184 {
            s__1688: s__1688.clone()
        };
        let fn__12740 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12740())
        };
        test___164.assert(t___12750, fn__12740.clone());
        test___164.soft_fail_to_hard()
    }
    #[test]
    fn intersectSql__2378() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___165 = temper_std::testing::Test::new();
        let mut t___12729: SafeIdentifier = sid__662("users");
        let mut t___12730: SafeIdentifier = sid__662("email");
        let a__1690: Query = from(t___12729.clone()).select([t___12730.clone()]);
        let mut t___12732: SafeIdentifier = sid__662("subscribers");
        let mut t___12733: SafeIdentifier = sid__662("email");
        let b__1691: Query = from(t___12732.clone()).select([t___12733.clone()]);
        let s__1692: std::sync::Arc<String> = intersect_sql(a__1690.clone(), b__1691.clone()).to_string();
        let mut t___12738: bool = Some(s__1692.as_str()) == Some("(SELECT email FROM users) INTERSECT (SELECT email FROM subscribers)");
        #[derive(Clone)]
        struct ClosureGroup___185 {
            s__1692: std::sync::Arc<String>
        }
        impl ClosureGroup___185 {
            fn fn__12728(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("intersectSql: {}", self.s__1692.clone()));
            }
        }
        let closure_group = ClosureGroup___185 {
            s__1692: s__1692.clone()
        };
        let fn__12728 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12728())
        };
        test___165.assert(t___12738, fn__12728.clone());
        test___165.soft_fail_to_hard()
    }
    #[test]
    fn exceptSql__2379() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___166 = temper_std::testing::Test::new();
        let mut t___12717: SafeIdentifier = sid__662("users");
        let mut t___12718: SafeIdentifier = sid__662("id");
        let a__1694: Query = from(t___12717.clone()).select([t___12718.clone()]);
        let mut t___12720: SafeIdentifier = sid__662("banned");
        let mut t___12721: SafeIdentifier = sid__662("id");
        let b__1695: Query = from(t___12720.clone()).select([t___12721.clone()]);
        let s__1696: std::sync::Arc<String> = except_sql(a__1694.clone(), b__1695.clone()).to_string();
        let mut t___12726: bool = Some(s__1696.as_str()) == Some("(SELECT id FROM users) EXCEPT (SELECT id FROM banned)");
        #[derive(Clone)]
        struct ClosureGroup___186 {
            s__1696: std::sync::Arc<String>
        }
        impl ClosureGroup___186 {
            fn fn__12716(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exceptSql: {}", self.s__1696.clone()));
            }
        }
        let closure_group = ClosureGroup___186 {
            s__1696: s__1696.clone()
        };
        let fn__12716 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12716())
        };
        test___166.assert(t___12726, fn__12716.clone());
        test___166.soft_fail_to_hard()
    }
    #[test]
    fn subqueryWithAlias__2380() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___167 = temper_std::testing::Test::new();
        let mut t___12702: SafeIdentifier = sid__662("orders");
        let mut t___12703: SafeIdentifier = sid__662("user_id");
        let mut t___12704: Query = from(t___12702.clone()).select([t___12703.clone()]);
        let mut t___12705: SqlBuilder = SqlBuilder::new();
        t___12705.append_safe("total > ");
        t___12705.append_int32(100);
        let inner__1698: Query = t___12704.r#where(t___12705.accumulated());
        let s__1699: std::sync::Arc<String> = subquery(inner__1698.clone(), sid__662("big_orders")).to_string();
        let mut t___12714: bool = Some(s__1699.as_str()) == Some("(SELECT user_id FROM orders WHERE total > 100) AS big_orders");
        #[derive(Clone)]
        struct ClosureGroup___187 {
            s__1699: std::sync::Arc<String>
        }
        impl ClosureGroup___187 {
            fn fn__12701(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("subquery: {}", self.s__1699.clone()));
            }
        }
        let closure_group = ClosureGroup___187 {
            s__1699: s__1699.clone()
        };
        let fn__12701 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12701())
        };
        test___167.assert(t___12714, fn__12701.clone());
        test___167.soft_fail_to_hard()
    }
    #[test]
    fn existsSql__2382() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___168 = temper_std::testing::Test::new();
        let mut t___12691: SafeIdentifier = sid__662("orders");
        let mut t___12692: SqlBuilder = SqlBuilder::new();
        t___12692.append_safe("orders.user_id = users.id");
        let mut t___12694: SqlFragment = t___12692.accumulated();
        let inner__1701: Query = from(t___12691.clone()).r#where(t___12694.clone());
        let s__1702: std::sync::Arc<String> = exists_sql(inner__1701.clone()).to_string();
        let mut t___12699: bool = Some(s__1702.as_str()) == Some("EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___188 {
            s__1702: std::sync::Arc<String>
        }
        impl ClosureGroup___188 {
            fn fn__12690(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("existsSql: {}", self.s__1702.clone()));
            }
        }
        let closure_group = ClosureGroup___188 {
            s__1702: s__1702.clone()
        };
        let fn__12690 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12690())
        };
        test___168.assert(t___12699, fn__12690.clone());
        test___168.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubquery__2384() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___169 = temper_std::testing::Test::new();
        let mut t___12674: SafeIdentifier = sid__662("orders");
        let mut t___12675: SafeIdentifier = sid__662("user_id");
        let mut t___12676: Query = from(t___12674.clone()).select([t___12675.clone()]);
        let mut t___12677: SqlBuilder = SqlBuilder::new();
        t___12677.append_safe("total > ");
        t___12677.append_int32(1000);
        let sub__1704: Query = t___12676.r#where(t___12677.accumulated());
        let mut t___12682: SafeIdentifier = sid__662("users");
        let mut t___12683: SafeIdentifier = sid__662("id");
        let q__1705: Query = from(t___12682.clone()).where_in_subquery(t___12683.clone(), sub__1704.clone());
        let s__1706: std::sync::Arc<String> = q__1705.to_sql().to_string();
        let mut t___12688: bool = Some(s__1706.as_str()) == Some("SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 1000)");
        #[derive(Clone)]
        struct ClosureGroup___189 {
            s__1706: std::sync::Arc<String>
        }
        impl ClosureGroup___189 {
            fn fn__12673(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery: {}", self.s__1706.clone()));
            }
        }
        let closure_group = ClosureGroup___189 {
            s__1706: s__1706.clone()
        };
        let fn__12673 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12673())
        };
        test___169.assert(t___12688, fn__12673.clone());
        test___169.soft_fail_to_hard()
    }
    #[test]
    fn setOperationWithWhereOnEachSide__2386() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___170 = temper_std::testing::Test::new();
        let mut t___12651: SafeIdentifier = sid__662("users");
        let mut t___12652: SqlBuilder = SqlBuilder::new();
        t___12652.append_safe("age > ");
        t___12652.append_int32(18);
        let mut t___12655: SqlFragment = t___12652.accumulated();
        let mut t___12656: Query = from(t___12651.clone()).r#where(t___12655.clone());
        let mut t___12657: SqlBuilder = SqlBuilder::new();
        t___12657.append_safe("active = ");
        t___12657.append_boolean(true);
        let a__1708: Query = t___12656.r#where(t___12657.accumulated());
        let mut t___12662: SafeIdentifier = sid__662("users");
        let mut t___12663: SqlBuilder = SqlBuilder::new();
        t___12663.append_safe("role = ");
        t___12663.append_string("vip");
        let mut t___12666: SqlFragment = t___12663.accumulated();
        let b__1709: Query = from(t___12662.clone()).r#where(t___12666.clone());
        let s__1710: std::sync::Arc<String> = union_sql(a__1708.clone(), b__1709.clone()).to_string();
        let mut t___12671: bool = Some(s__1710.as_str()) == Some("(SELECT * FROM users WHERE age > 18 AND active = TRUE) UNION (SELECT * FROM users WHERE role = 'vip')");
        #[derive(Clone)]
        struct ClosureGroup___190 {
            s__1710: std::sync::Arc<String>
        }
        impl ClosureGroup___190 {
            fn fn__12650(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("union with where: {}", self.s__1710.clone()));
            }
        }
        let closure_group = ClosureGroup___190 {
            s__1710: s__1710.clone()
        };
        let fn__12650 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12650())
        };
        test___170.assert(t___12671, fn__12650.clone());
        test___170.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubqueryChainedWithWhere__2390() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___171 = temper_std::testing::Test::new();
        let mut t___12634: SafeIdentifier = sid__662("orders");
        let mut t___12635: SafeIdentifier = sid__662("user_id");
        let sub__1712: Query = from(t___12634.clone()).select([t___12635.clone()]);
        let mut t___12637: SafeIdentifier = sid__662("users");
        let mut t___12638: SqlBuilder = SqlBuilder::new();
        t___12638.append_safe("active = ");
        t___12638.append_boolean(true);
        let mut t___12641: SqlFragment = t___12638.accumulated();
        let q__1713: Query = from(t___12637.clone()).r#where(t___12641.clone()).where_in_subquery(sid__662("id"), sub__1712.clone());
        let s__1714: std::sync::Arc<String> = q__1713.to_sql().to_string();
        let mut t___12648: bool = Some(s__1714.as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND id IN (SELECT user_id FROM orders)");
        #[derive(Clone)]
        struct ClosureGroup___191 {
            s__1714: std::sync::Arc<String>
        }
        impl ClosureGroup___191 {
            fn fn__12633(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery chained: {}", self.s__1714.clone()));
            }
        }
        let closure_group = ClosureGroup___191 {
            s__1714: s__1714.clone()
        };
        let fn__12633 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12633())
        };
        test___171.assert(t___12648, fn__12633.clone());
        test___171.soft_fail_to_hard()
    }
    #[test]
    fn existsSqlUsedInWhere__2392() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___172 = temper_std::testing::Test::new();
        let mut t___12620: SafeIdentifier = sid__662("orders");
        let mut t___12621: SqlBuilder = SqlBuilder::new();
        t___12621.append_safe("orders.user_id = users.id");
        let mut t___12623: SqlFragment = t___12621.accumulated();
        let sub__1716: Query = from(t___12620.clone()).r#where(t___12623.clone());
        let mut t___12625: SafeIdentifier = sid__662("users");
        let mut t___12626: SqlFragment = exists_sql(sub__1716.clone());
        let q__1717: Query = from(t___12625.clone()).r#where(t___12626.clone());
        let s__1718: std::sync::Arc<String> = q__1717.to_sql().to_string();
        let mut t___12631: bool = Some(s__1718.as_str()) == Some("SELECT * FROM users WHERE EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___192 {
            s__1718: std::sync::Arc<String>
        }
        impl ClosureGroup___192 {
            fn fn__12619(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exists in where: {}", self.s__1718.clone()));
            }
        }
        let closure_group = ClosureGroup___192 {
            s__1718: s__1718.clone()
        };
        let fn__12619 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12619())
        };
        test___172.assert(t___12631, fn__12619.clone());
        test___172.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBasic__2394() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___173 = temper_std::testing::Test::new();
        let mut t___12606: SafeIdentifier;
        let mut t___12607: SafeIdentifier;
        let mut t___12608: SqlString;
        let mut t___12609: UpdateQuery;
        let mut t___12610: SqlBuilder;
        let mut t___6167: SqlFragment;
        let q__1720: SqlFragment;
        'ok___15634: {
            'orelse___2568: {
                t___12606 = sid__662("users");
                t___12607 = sid__662("name");
                t___12608 = SqlString::new("Alice");
                t___12609 = update(t___12606.clone()).set(t___12607.clone(), SqlPart::new(t___12608.clone()));
                t___12610 = SqlBuilder::new();
                t___12610.append_safe("id = ");
                t___12610.append_int32(1);
                t___6167 = match t___12609.r#where(t___12610.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2568
                };
                q__1720 = t___6167.clone();
                break 'ok___15634;
            }
            q__1720 = panic!();
        }
        let mut t___12617: bool = Some(q__1720.to_string().as_str()) == Some("UPDATE users SET name = 'Alice' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___193 {}
        impl ClosureGroup___193 {
            fn fn__12605(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update basic".to_string());
            }
        }
        let closure_group = ClosureGroup___193 {};
        let fn__12605 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12605())
        };
        test___173.assert(t___12617, fn__12605.clone());
        test___173.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleSet__2396() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___174 = temper_std::testing::Test::new();
        let mut t___12589: SafeIdentifier;
        let mut t___12590: SafeIdentifier;
        let mut t___12591: SqlString;
        let mut t___12595: UpdateQuery;
        let mut t___12596: SqlBuilder;
        let mut t___6152: SqlFragment;
        let q__1722: SqlFragment;
        'ok___15635: {
            'orelse___2569: {
                t___12589 = sid__662("users");
                t___12590 = sid__662("name");
                t___12591 = SqlString::new("Bob");
                t___12595 = update(t___12589.clone()).set(t___12590.clone(), SqlPart::new(t___12591.clone())).set(sid__662("age"), SqlPart::new(SqlInt32::new(30)));
                t___12596 = SqlBuilder::new();
                t___12596.append_safe("id = ");
                t___12596.append_int32(2);
                t___6152 = match t___12595.r#where(t___12596.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2569
                };
                q__1722 = t___6152.clone();
                break 'ok___15635;
            }
            q__1722 = panic!();
        }
        let mut t___12603: bool = Some(q__1722.to_string().as_str()) == Some("UPDATE users SET name = 'Bob', age = 30 WHERE id = 2");
        #[derive(Clone)]
        struct ClosureGroup___194 {}
        impl ClosureGroup___194 {
            fn fn__12588(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi set".to_string());
            }
        }
        let closure_group = ClosureGroup___194 {};
        let fn__12588 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12588())
        };
        test___174.assert(t___12603, fn__12588.clone());
        test___174.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleWhere__2398() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___175 = temper_std::testing::Test::new();
        let mut t___12570: SafeIdentifier;
        let mut t___12571: SafeIdentifier;
        let mut t___12572: SqlBoolean;
        let mut t___12573: UpdateQuery;
        let mut t___12574: SqlBuilder;
        let mut t___12578: UpdateQuery;
        let mut t___12579: SqlBuilder;
        let mut t___6134: SqlFragment;
        let q__1724: SqlFragment;
        'ok___15636: {
            'orelse___2570: {
                t___12570 = sid__662("users");
                t___12571 = sid__662("active");
                t___12572 = SqlBoolean::new(false);
                t___12573 = update(t___12570.clone()).set(t___12571.clone(), SqlPart::new(t___12572.clone()));
                t___12574 = SqlBuilder::new();
                t___12574.append_safe("age < ");
                t___12574.append_int32(18);
                t___12578 = t___12573.r#where(t___12574.accumulated());
                t___12579 = SqlBuilder::new();
                t___12579.append_safe("role = ");
                t___12579.append_string("guest");
                t___6134 = match t___12578.r#where(t___12579.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2570
                };
                q__1724 = t___6134.clone();
                break 'ok___15636;
            }
            q__1724 = panic!();
        }
        let mut t___12586: bool = Some(q__1724.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE age < 18 AND role = 'guest'");
        #[derive(Clone)]
        struct ClosureGroup___195 {}
        impl ClosureGroup___195 {
            fn fn__12569(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___195 {};
        let fn__12569 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12569())
        };
        test___175.assert(t___12586, fn__12569.clone());
        test___175.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryOrWhere__2401() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___176 = temper_std::testing::Test::new();
        let mut t___12551: SafeIdentifier;
        let mut t___12552: SafeIdentifier;
        let mut t___12553: SqlString;
        let mut t___12554: UpdateQuery;
        let mut t___12555: SqlBuilder;
        let mut t___12559: UpdateQuery;
        let mut t___12560: SqlBuilder;
        let mut t___6113: SqlFragment;
        let q__1726: SqlFragment;
        'ok___15637: {
            'orelse___2571: {
                t___12551 = sid__662("users");
                t___12552 = sid__662("status");
                t___12553 = SqlString::new("banned");
                t___12554 = update(t___12551.clone()).set(t___12552.clone(), SqlPart::new(t___12553.clone()));
                t___12555 = SqlBuilder::new();
                t___12555.append_safe("spam_count > ");
                t___12555.append_int32(10);
                t___12559 = t___12554.r#where(t___12555.accumulated());
                t___12560 = SqlBuilder::new();
                t___12560.append_safe("reported = ");
                t___12560.append_boolean(true);
                t___6113 = match t___12559.or_where(t___12560.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2571
                };
                q__1726 = t___6113.clone();
                break 'ok___15637;
            }
            q__1726 = panic!();
        }
        let mut t___12567: bool = Some(q__1726.to_string().as_str()) == Some("UPDATE users SET status = 'banned' WHERE spam_count > 10 OR reported = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___196 {}
        impl ClosureGroup___196 {
            fn fn__12550(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___196 {};
        let fn__12550 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12550())
        };
        test___176.assert(t___12567, fn__12550.clone());
        test___176.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutWhere__2404() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___177 = temper_std::testing::Test::new();
        let mut t___12544: SafeIdentifier;
        let mut t___12545: SafeIdentifier;
        let mut t___12546: SqlInt32;
        let didBubble__1728: bool;
        'ok___15638: {
            'orelse___2572: {
                t___12544 = sid__662("users");
                t___12545 = sid__662("x");
                t___12546 = SqlInt32::new(1);
                match update(t___12544.clone()).set(t___12545.clone(), SqlPart::new(t___12546.clone())).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2572
                };
                didBubble__1728 = false;
                break 'ok___15638;
            }
            didBubble__1728 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___197 {}
        impl ClosureGroup___197 {
            fn fn__12543(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___197 {};
        let fn__12543 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12543())
        };
        test___177.assert(didBubble__1728, fn__12543.clone());
        test___177.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutSet__2405() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___178 = temper_std::testing::Test::new();
        let mut t___12535: SafeIdentifier;
        let mut t___12536: SqlBuilder;
        let mut t___12539: SqlFragment;
        let didBubble__1730: bool;
        'ok___15639: {
            'orelse___2573: {
                t___12535 = sid__662("users");
                t___12536 = SqlBuilder::new();
                t___12536.append_safe("id = ");
                t___12536.append_int32(1);
                t___12539 = t___12536.accumulated();
                match update(t___12535.clone()).r#where(t___12539.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2573
                };
                didBubble__1730 = false;
                break 'ok___15639;
            }
            didBubble__1730 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___198 {}
        impl ClosureGroup___198 {
            fn fn__12534(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without SET should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___198 {};
        let fn__12534 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12534())
        };
        test___178.assert(didBubble__1730, fn__12534.clone());
        test___178.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryWithLimit__2407() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___179 = temper_std::testing::Test::new();
        let mut t___12521: SafeIdentifier;
        let mut t___12522: SafeIdentifier;
        let mut t___12523: SqlBoolean;
        let mut t___12524: UpdateQuery;
        let mut t___12525: SqlBuilder;
        let mut t___6076: UpdateQuery;
        let mut t___6077: SqlFragment;
        let q__1732: SqlFragment;
        'ok___15640: {
            'orelse___2574: {
                t___12521 = sid__662("users");
                t___12522 = sid__662("active");
                t___12523 = SqlBoolean::new(false);
                t___12524 = update(t___12521.clone()).set(t___12522.clone(), SqlPart::new(t___12523.clone()));
                t___12525 = SqlBuilder::new();
                t___12525.append_safe("last_login < ");
                t___12525.append_string("2024-01-01");
                t___6076 = match t___12524.r#where(t___12525.accumulated()).limit(100) {
                    Ok(x) => x,
                    _ => break 'orelse___2574
                };
                t___6077 = match t___6076.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2574
                };
                q__1732 = t___6077.clone();
                break 'ok___15640;
            }
            q__1732 = panic!();
        }
        let mut t___12532: bool = Some(q__1732.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE last_login < '2024-01-01' LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___199 {}
        impl ClosureGroup___199 {
            fn fn__12520(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update limit".to_string());
            }
        }
        let closure_group = ClosureGroup___199 {};
        let fn__12520 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12520())
        };
        test___179.assert(t___12532, fn__12520.clone());
        test___179.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryEscaping__2409() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___180 = temper_std::testing::Test::new();
        let mut t___12507: SafeIdentifier;
        let mut t___12508: SafeIdentifier;
        let mut t___12509: SqlString;
        let mut t___12510: UpdateQuery;
        let mut t___12511: SqlBuilder;
        let mut t___6061: SqlFragment;
        let q__1734: SqlFragment;
        'ok___15641: {
            'orelse___2575: {
                t___12507 = sid__662("users");
                t___12508 = sid__662("bio");
                t___12509 = SqlString::new("It's a test");
                t___12510 = update(t___12507.clone()).set(t___12508.clone(), SqlPart::new(t___12509.clone()));
                t___12511 = SqlBuilder::new();
                t___12511.append_safe("id = ");
                t___12511.append_int32(1);
                t___6061 = match t___12510.r#where(t___12511.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2575
                };
                q__1734 = t___6061.clone();
                break 'ok___15641;
            }
            q__1734 = panic!();
        }
        let mut t___12518: bool = Some(q__1734.to_string().as_str()) == Some("UPDATE users SET bio = 'It''s a test' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___200 {}
        impl ClosureGroup___200 {
            fn fn__12506(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update escaping".to_string());
            }
        }
        let closure_group = ClosureGroup___200 {};
        let fn__12506 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12506())
        };
        test___180.assert(t___12518, fn__12506.clone());
        test___180.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBasic__2411() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___181 = temper_std::testing::Test::new();
        let mut t___12496: SafeIdentifier;
        let mut t___12497: SqlBuilder;
        let mut t___12500: SqlFragment;
        let mut t___6046: SqlFragment;
        let q__1736: SqlFragment;
        'ok___15642: {
            'orelse___2576: {
                t___12496 = sid__662("users");
                t___12497 = SqlBuilder::new();
                t___12497.append_safe("id = ");
                t___12497.append_int32(1);
                t___12500 = t___12497.accumulated();
                t___6046 = match delete_from(t___12496.clone()).r#where(t___12500.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2576
                };
                q__1736 = t___6046.clone();
                break 'ok___15642;
            }
            q__1736 = panic!();
        }
        let mut t___12504: bool = Some(q__1736.to_string().as_str()) == Some("DELETE FROM users WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___201 {}
        impl ClosureGroup___201 {
            fn fn__12495(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete basic".to_string());
            }
        }
        let closure_group = ClosureGroup___201 {};
        let fn__12495 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12495())
        };
        test___181.assert(t___12504, fn__12495.clone());
        test___181.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryMultipleWhere__2413() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___182 = temper_std::testing::Test::new();
        let mut t___12480: SafeIdentifier;
        let mut t___12481: SqlBuilder;
        let mut t___12484: SqlFragment;
        let mut t___12485: DeleteQuery;
        let mut t___12486: SqlBuilder;
        let mut t___6034: SqlFragment;
        let q__1738: SqlFragment;
        'ok___15643: {
            'orelse___2577: {
                t___12480 = sid__662("logs");
                t___12481 = SqlBuilder::new();
                t___12481.append_safe("created_at < ");
                t___12481.append_string("2024-01-01");
                t___12484 = t___12481.accumulated();
                t___12485 = delete_from(t___12480.clone()).r#where(t___12484.clone());
                t___12486 = SqlBuilder::new();
                t___12486.append_safe("level = ");
                t___12486.append_string("debug");
                t___6034 = match t___12485.r#where(t___12486.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2577
                };
                q__1738 = t___6034.clone();
                break 'ok___15643;
            }
            q__1738 = panic!();
        }
        let mut t___12493: bool = Some(q__1738.to_string().as_str()) == Some("DELETE FROM logs WHERE created_at < '2024-01-01' AND level = 'debug'");
        #[derive(Clone)]
        struct ClosureGroup___202 {}
        impl ClosureGroup___202 {
            fn fn__12479(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___202 {};
        let fn__12479 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12479())
        };
        test___182.assert(t___12493, fn__12479.clone());
        test___182.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBubblesWithoutWhere__2416() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___183 = temper_std::testing::Test::new();
        let didBubble__1740: bool;
        'ok___15644: {
            'orelse___2578: {
                match delete_from(sid__662("users")).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2578
                };
                didBubble__1740 = false;
                break 'ok___15644;
            }
            didBubble__1740 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___203 {}
        impl ClosureGroup___203 {
            fn fn__12475(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___203 {};
        let fn__12475 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12475())
        };
        test___183.assert(didBubble__1740, fn__12475.clone());
        test___183.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryOrWhere__2417() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___184 = temper_std::testing::Test::new();
        let mut t___12460: SafeIdentifier;
        let mut t___12461: SqlBuilder;
        let mut t___12464: SqlFragment;
        let mut t___12465: DeleteQuery;
        let mut t___12466: SqlBuilder;
        let mut t___6013: SqlFragment;
        let q__1742: SqlFragment;
        'ok___15645: {
            'orelse___2579: {
                t___12460 = sid__662("sessions");
                t___12461 = SqlBuilder::new();
                t___12461.append_safe("expired = ");
                t___12461.append_boolean(true);
                t___12464 = t___12461.accumulated();
                t___12465 = delete_from(t___12460.clone()).r#where(t___12464.clone());
                t___12466 = SqlBuilder::new();
                t___12466.append_safe("created_at < ");
                t___12466.append_string("2023-01-01");
                t___6013 = match t___12465.or_where(t___12466.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2579
                };
                q__1742 = t___6013.clone();
                break 'ok___15645;
            }
            q__1742 = panic!();
        }
        let mut t___12473: bool = Some(q__1742.to_string().as_str()) == Some("DELETE FROM sessions WHERE expired = TRUE OR created_at < '2023-01-01'");
        #[derive(Clone)]
        struct ClosureGroup___204 {}
        impl ClosureGroup___204 {
            fn fn__12459(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___204 {};
        let fn__12459 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12459())
        };
        test___184.assert(t___12473, fn__12459.clone());
        test___184.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryWithLimit__2420() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___185 = temper_std::testing::Test::new();
        let mut t___12449: SafeIdentifier;
        let mut t___12450: SqlBuilder;
        let mut t___12453: SqlFragment;
        let mut t___5994: DeleteQuery;
        let mut t___5995: SqlFragment;
        let q__1744: SqlFragment;
        'ok___15646: {
            'orelse___2580: {
                t___12449 = sid__662("logs");
                t___12450 = SqlBuilder::new();
                t___12450.append_safe("level = ");
                t___12450.append_string("debug");
                t___12453 = t___12450.accumulated();
                t___5994 = match delete_from(t___12449.clone()).r#where(t___12453.clone()).limit(1000) {
                    Ok(x) => x,
                    _ => break 'orelse___2580
                };
                t___5995 = match t___5994.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___2580
                };
                q__1744 = t___5995.clone();
                break 'ok___15646;
            }
            q__1744 = panic!();
        }
        let mut t___12457: bool = Some(q__1744.to_string().as_str()) == Some("DELETE FROM logs WHERE level = 'debug' LIMIT 1000");
        #[derive(Clone)]
        struct ClosureGroup___205 {}
        impl ClosureGroup___205 {
            fn fn__12448(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete limit".to_string());
            }
        }
        let closure_group = ClosureGroup___205 {};
        let fn__12448 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12448())
        };
        test___185.assert(t___12457, fn__12448.clone());
        test___185.soft_fail_to_hard()
    }
    #[test]
    fn orderByNullsNullsFirst__2422() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___186 = temper_std::testing::Test::new();
        let mut t___12439: SafeIdentifier = sid__662("users");
        let mut t___12440: SafeIdentifier = sid__662("email");
        let mut t___12441: NullsFirst = NullsFirst::new();
        let q__1746: Query = from(t___12439.clone()).order_by_nulls(t___12440.clone(), true, NullsPosition::new(t___12441.clone()));
        let mut t___12446: bool = Some(q__1746.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY email ASC NULLS FIRST");
        #[derive(Clone)]
        struct ClosureGroup___206 {}
        impl ClosureGroup___206 {
            fn fn__12438(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("nulls first".to_string());
            }
        }
        let closure_group = ClosureGroup___206 {};
        let fn__12438 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12438())
        };
        test___186.assert(t___12446, fn__12438.clone());
        test___186.soft_fail_to_hard()
    }
    #[test]
    fn orderByNullsNullsLast__2423() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___187 = temper_std::testing::Test::new();
        let mut t___12429: SafeIdentifier = sid__662("users");
        let mut t___12430: SafeIdentifier = sid__662("score");
        let mut t___12431: NullsLast = NullsLast::new();
        let q__1748: Query = from(t___12429.clone()).order_by_nulls(t___12430.clone(), false, NullsPosition::new(t___12431.clone()));
        let mut t___12436: bool = Some(q__1748.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY score DESC NULLS LAST");
        #[derive(Clone)]
        struct ClosureGroup___207 {}
        impl ClosureGroup___207 {
            fn fn__12428(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("nulls last".to_string());
            }
        }
        let closure_group = ClosureGroup___207 {};
        let fn__12428 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12428())
        };
        test___187.assert(t___12436, fn__12428.clone());
        test___187.soft_fail_to_hard()
    }
    #[test]
    fn mixedOrderByAndOrderByNulls__2424() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___188 = temper_std::testing::Test::new();
        let mut t___12417: SafeIdentifier = sid__662("users");
        let mut t___12418: SafeIdentifier = sid__662("name");
        let q__1750: Query = from(t___12417.clone()).order_by(t___12418.clone(), true).order_by_nulls(sid__662("email"), true, NullsPosition::new(NullsFirst::new()));
        let mut t___12426: bool = Some(q__1750.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC, email ASC NULLS FIRST");
        #[derive(Clone)]
        struct ClosureGroup___208 {}
        impl ClosureGroup___208 {
            fn fn__12416(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed order".to_string());
            }
        }
        let closure_group = ClosureGroup___208 {};
        let fn__12416 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12416())
        };
        test___188.assert(t___12426, fn__12416.clone());
        test___188.soft_fail_to_hard()
    }
    #[test]
    fn crossJoin__2425() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___189 = temper_std::testing::Test::new();
        let mut t___12408: SafeIdentifier = sid__662("users");
        let mut t___12409: SafeIdentifier = sid__662("colors");
        let q__1752: Query = from(t___12408.clone()).cross_join(t___12409.clone());
        let mut t___12414: bool = Some(q__1752.to_sql().to_string().as_str()) == Some("SELECT * FROM users CROSS JOIN colors");
        #[derive(Clone)]
        struct ClosureGroup___209 {}
        impl ClosureGroup___209 {
            fn fn__12407(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("cross join".to_string());
            }
        }
        let closure_group = ClosureGroup___209 {};
        let fn__12407 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12407())
        };
        test___189.assert(t___12414, fn__12407.clone());
        test___189.soft_fail_to_hard()
    }
    #[test]
    fn crossJoinCombinedWithOtherJoins__2426() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___190 = temper_std::testing::Test::new();
        let mut t___12394: SafeIdentifier = sid__662("users");
        let mut t___12395: SafeIdentifier = sid__662("orders");
        let mut t___12396: SqlBuilder = SqlBuilder::new();
        t___12396.append_safe("users.id = orders.user_id");
        let mut t___12398: SqlFragment = t___12396.accumulated();
        let q__1754: Query = from(t___12394.clone()).inner_join(t___12395.clone(), t___12398.clone()).cross_join(sid__662("colors"));
        let mut t___12405: bool = Some(q__1754.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id CROSS JOIN colors");
        #[derive(Clone)]
        struct ClosureGroup___210 {}
        impl ClosureGroup___210 {
            fn fn__12393(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("cross + inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___210 {};
        let fn__12393 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12393())
        };
        test___190.assert(t___12405, fn__12393.clone());
        test___190.soft_fail_to_hard()
    }
    #[test]
    fn lockForUpdate__2428() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___191 = temper_std::testing::Test::new();
        let mut t___12380: SafeIdentifier = sid__662("users");
        let mut t___12381: SqlBuilder = SqlBuilder::new();
        t___12381.append_safe("id = ");
        t___12381.append_int32(1);
        let mut t___12384: SqlFragment = t___12381.accumulated();
        let q__1756: Query = from(t___12380.clone()).r#where(t___12384.clone()).lock(LockMode::new(ForUpdate::new()));
        let mut t___12391: bool = Some(q__1756.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id = 1 FOR UPDATE");
        #[derive(Clone)]
        struct ClosureGroup___211 {}
        impl ClosureGroup___211 {
            fn fn__12379(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("for update".to_string());
            }
        }
        let closure_group = ClosureGroup___211 {};
        let fn__12379 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12379())
        };
        test___191.assert(t___12391, fn__12379.clone());
        test___191.soft_fail_to_hard()
    }
    #[test]
    fn lockForShare__2430() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___192 = temper_std::testing::Test::new();
        let mut t___12369: SafeIdentifier = sid__662("users");
        let mut t___12370: SafeIdentifier = sid__662("name");
        let q__1758: Query = from(t___12369.clone()).select([t___12370.clone()]).lock(LockMode::new(ForShare::new()));
        let mut t___12377: bool = Some(q__1758.to_sql().to_string().as_str()) == Some("SELECT name FROM users FOR SHARE");
        #[derive(Clone)]
        struct ClosureGroup___212 {}
        impl ClosureGroup___212 {
            fn fn__12368(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("for share".to_string());
            }
        }
        let closure_group = ClosureGroup___212 {};
        let fn__12368 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12368())
        };
        test___192.assert(t___12377, fn__12368.clone());
        test___192.soft_fail_to_hard()
    }
    #[test]
    fn lockWithFullQuery__2431() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___193 = temper_std::testing::Test::new();
        let mut t___12355: SafeIdentifier;
        let mut t___12356: SqlBuilder;
        let mut t___12359: SqlFragment;
        let mut t___12362: Query;
        let mut t___5918: Query;
        let q__1760: Query;
        'ok___15647: {
            'orelse___2581: {
                t___12355 = sid__662("accounts");
                t___12356 = SqlBuilder::new();
                t___12356.append_safe("id = ");
                t___12356.append_int32(42);
                t___12359 = t___12356.accumulated();
                t___5918 = match from(t___12355.clone()).r#where(t___12359.clone()).limit(1) {
                    Ok(x) => x,
                    _ => break 'orelse___2581
                };
                t___12362 = t___5918.lock(LockMode::new(ForUpdate::new()));
                q__1760 = t___12362.clone();
                break 'ok___15647;
            }
            q__1760 = panic!();
        }
        let mut t___12366: bool = Some(q__1760.to_sql().to_string().as_str()) == Some("SELECT * FROM accounts WHERE id = 42 LIMIT 1 FOR UPDATE");
        #[derive(Clone)]
        struct ClosureGroup___213 {}
        impl ClosureGroup___213 {
            fn fn__12354(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("lock full query".to_string());
            }
        }
        let closure_group = ClosureGroup___213 {};
        let fn__12354 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12354())
        };
        test___193.assert(t___12366, fn__12354.clone());
        test___193.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__2433() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___200 = temper_std::testing::Test::new();
        let mut t___5907: SafeIdentifier;
        let id__1808: SafeIdentifier;
        'ok___15648: {
            'orelse___2582: {
                t___5907 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___2582
                };
                id__1808 = t___5907.clone();
                break 'ok___15648;
            }
            id__1808 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12352: bool = Some(id__1808.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___214 {}
        impl ClosureGroup___214 {
            fn fn__12349(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___214 {};
        let fn__12349 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12349())
        };
        test___200.assert(t___12352, fn__12349.clone());
        test___200.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__2434() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___201 = temper_std::testing::Test::new();
        let didBubble__1810: bool;
        'ok___15649: {
            'orelse___2583: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___2583
                };
                didBubble__1810 = false;
                break 'ok___15649;
            }
            didBubble__1810 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___215 {}
        impl ClosureGroup___215 {
            fn fn__12346(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___215 {};
        let fn__12346 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12346())
        };
        test___201.assert(didBubble__1810, fn__12346.clone());
        test___201.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__2435() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___202 = temper_std::testing::Test::new();
        let didBubble__1812: bool;
        'ok___15650: {
            'orelse___2584: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___2584
                };
                didBubble__1812 = false;
                break 'ok___15650;
            }
            didBubble__1812 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___216 {}
        impl ClosureGroup___216 {
            fn fn__12343(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___216 {};
        let fn__12343 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12343())
        };
        test___202.assert(didBubble__1812, fn__12343.clone());
        test___202.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__2436() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___203 = temper_std::testing::Test::new();
        let cases__1814: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___217 {
            test___203: temper_std::testing::Test
        }
        impl ClosureGroup___217 {
            fn fn__12340(& self, c__1815: impl temper_core::ToArcString) {
                let c__1815 = c__1815.to_arc_string();
                let didBubble__1816: bool;
                'ok___15651: {
                    'orelse___2585: {
                        match safe_identifier(c__1815.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___2585
                        };
                        didBubble__1816 = false;
                        break 'ok___15651;
                    }
                    didBubble__1816 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___218 {
                    c__1815: std::sync::Arc<String>
                }
                impl ClosureGroup___218 {
                    fn fn__12337(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__1815.clone()));
                    }
                }
                let closure_group = ClosureGroup___218 {
                    c__1815: c__1815.clone()
                };
                let fn__12337 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__12337())
                };
                self.test___203.assert(didBubble__1816, fn__12337.clone());
            }
        }
        let closure_group = ClosureGroup___217 {
            test___203: test___203.clone()
        };
        let fn__12340 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1815: std::sync::Arc<String> | closure_group.fn__12340(c__1815))
        };
        temper_core::listed::list_for_each( & cases__1814, & ( * fn__12340.clone()));
        test___203.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__2437() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___204 = temper_std::testing::Test::new();
        let mut t___5884: SafeIdentifier;
        let mut t___5885: SafeIdentifier;
        let mut t___5886: SafeIdentifier;
        let mut t___5887: SafeIdentifier;
        let mut t___5890: SafeIdentifier;
        let mut t___5891: SafeIdentifier;
        let mut t___5895: FieldDef;
        'ok___15652: {
            'orelse___2586: {
                t___5884 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2586
                };
                t___5885 = t___5884.clone();
                break 'ok___15652;
            }
            t___5885 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___15653: {
            'orelse___2587: {
                t___5886 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2587
                };
                t___5887 = t___5886.clone();
                break 'ok___15653;
            }
            t___5887 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12327: StringField = StringField::new();
        let mut t___12328: FieldDef = FieldDef::new(t___5887.clone(), FieldType::new(t___12327.clone()), false, None, false);
        'ok___15654: {
            'orelse___2588: {
                t___5890 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___2588
                };
                t___5891 = t___5890.clone();
                break 'ok___15654;
            }
            t___5891 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12329: IntField = IntField::new();
        let mut t___12330: FieldDef = FieldDef::new(t___5891.clone(), FieldType::new(t___12329.clone()), false, None, false);
        let td__1818: TableDef = TableDef::new(t___5885.clone(), [t___12328.clone(), t___12330.clone()], None);
        let f__1819: FieldDef;
        'ok___15655: {
            'orelse___2589: {
                t___5895 = match td__1818.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___2589
                };
                f__1819 = t___5895.clone();
                break 'ok___15655;
            }
            f__1819 = panic!();
        }
        let mut t___12335: bool = Some(f__1819.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___219 {}
        impl ClosureGroup___219 {
            fn fn__12326(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___219 {};
        let fn__12326 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12326())
        };
        test___204.assert(t___12335, fn__12326.clone());
        test___204.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__2438() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___205 = temper_std::testing::Test::new();
        let mut t___5875: SafeIdentifier;
        let mut t___5876: SafeIdentifier;
        let mut t___5877: SafeIdentifier;
        let mut t___5878: SafeIdentifier;
        'ok___15656: {
            'orelse___2590: {
                t___5875 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2590
                };
                t___5876 = t___5875.clone();
                break 'ok___15656;
            }
            t___5876 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___15657: {
            'orelse___2591: {
                t___5877 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2591
                };
                t___5878 = t___5877.clone();
                break 'ok___15657;
            }
            t___5878 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12321: StringField = StringField::new();
        let mut t___12322: FieldDef = FieldDef::new(t___5878.clone(), FieldType::new(t___12321.clone()), false, None, false);
        let td__1821: TableDef = TableDef::new(t___5876.clone(), [t___12322.clone()], None);
        let didBubble__1822: bool;
        'ok___15658: {
            'orelse___2592: {
                match td__1821.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___2592
                };
                didBubble__1822 = false;
                break 'ok___15658;
            }
            didBubble__1822 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___220 {}
        impl ClosureGroup___220 {
            fn fn__12320(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___220 {};
        let fn__12320 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12320())
        };
        test___205.assert(didBubble__1822, fn__12320.clone());
        test___205.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__2439() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___206 = temper_std::testing::Test::new();
        let mut t___5863: SafeIdentifier;
        let mut t___5864: SafeIdentifier;
        let mut t___5867: SafeIdentifier;
        let mut t___5868: SafeIdentifier;
        'ok___15659: {
            'orelse___2593: {
                t___5863 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___2593
                };
                t___5864 = t___5863.clone();
                break 'ok___15659;
            }
            t___5864 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12309: StringField = StringField::new();
        let required__1824: FieldDef = FieldDef::new(t___5864.clone(), FieldType::new(t___12309.clone()), false, None, false);
        'ok___15660: {
            'orelse___2594: {
                t___5867 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___2594
                };
                t___5868 = t___5867.clone();
                break 'ok___15660;
            }
            t___5868 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12311: StringField = StringField::new();
        let optional__1825: FieldDef = FieldDef::new(t___5868.clone(), FieldType::new(t___12311.clone()), true, None, false);
        let mut t___12315: bool = ! required__1824.nullable();
        #[derive(Clone)]
        struct ClosureGroup___221 {}
        impl ClosureGroup___221 {
            fn fn__12308(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___221 {};
        let fn__12308 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12308())
        };
        test___206.assert(t___12315, fn__12308.clone());
        let mut t___12317: bool = optional__1825.nullable();
        #[derive(Clone)]
        struct ClosureGroup___222 {}
        impl ClosureGroup___222 {
            fn fn__12307(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___222 {};
        let fn__12307 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12307())
        };
        test___206.assert(t___12317, fn__12307.clone());
        test___206.soft_fail_to_hard()
    }
    #[test]
    fn pkNameDefaultsToIdWhenPrimaryKeyIsNull__2440() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___207 = temper_std::testing::Test::new();
        let mut t___5854: SafeIdentifier;
        let mut t___5855: SafeIdentifier;
        let mut t___5856: SafeIdentifier;
        let mut t___5857: SafeIdentifier;
        'ok___15661: {
            'orelse___2595: {
                t___5854 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2595
                };
                t___5855 = t___5854.clone();
                break 'ok___15661;
            }
            t___5855 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___15662: {
            'orelse___2596: {
                t___5856 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2596
                };
                t___5857 = t___5856.clone();
                break 'ok___15662;
            }
            t___5857 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12300: StringField = StringField::new();
        let mut t___12301: FieldDef = FieldDef::new(t___5857.clone(), FieldType::new(t___12300.clone()), false, None, false);
        let td__1827: TableDef = TableDef::new(t___5855.clone(), [t___12301.clone()], None);
        let mut t___12305: bool = Some(td__1827.pk_name().as_str()) == Some("id");
        #[derive(Clone)]
        struct ClosureGroup___223 {}
        impl ClosureGroup___223 {
            fn fn__12299(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("default pk should be id".to_string());
            }
        }
        let closure_group = ClosureGroup___223 {};
        let fn__12299 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12299())
        };
        test___207.assert(t___12305, fn__12299.clone());
        test___207.soft_fail_to_hard()
    }
    #[test]
    fn pkNameReturnsCustomPrimaryKey__2441() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___208 = temper_std::testing::Test::new();
        let mut t___5842: SafeIdentifier;
        let mut t___5843: SafeIdentifier;
        let mut t___5844: SafeIdentifier;
        let mut t___5845: SafeIdentifier;
        let mut t___5848: SafeIdentifier;
        let mut t___5849: SafeIdentifier;
        'ok___15663: {
            'orelse___2597: {
                t___5842 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___2597
                };
                t___5843 = t___5842.clone();
                break 'ok___15663;
            }
            t___5843 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___15664: {
            'orelse___2598: {
                t___5844 = match safe_identifier("user_id") {
                    Ok(x) => x,
                    _ => break 'orelse___2598
                };
                t___5845 = t___5844.clone();
                break 'ok___15664;
            }
            t___5845 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12292: IntField = IntField::new();
        let mut t___5850: temper_core::List<FieldDef> = std::sync::Arc::new(vec![FieldDef::new(t___5845.clone(), FieldType::new(t___12292.clone()), false, None, false)]);
        'ok___15665: {
            'orelse___2599: {
                t___5848 = match safe_identifier("user_id") {
                    Ok(x) => x,
                    _ => break 'orelse___2599
                };
                t___5849 = t___5848.clone();
                break 'ok___15665;
            }
            t___5849 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let td__1829: TableDef = TableDef::new(t___5843.clone(), t___5850.clone(), Some(t___5849.clone()));
        let mut t___12297: bool = Some(td__1829.pk_name().as_str()) == Some("user_id");
        #[derive(Clone)]
        struct ClosureGroup___224 {}
        impl ClosureGroup___224 {
            fn fn__12291(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("custom pk should be user_id".to_string());
            }
        }
        let closure_group = ClosureGroup___224 {};
        let fn__12291 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12291())
        };
        test___208.assert(t___12297, fn__12291.clone());
        test___208.soft_fail_to_hard()
    }
    #[test]
    fn timestampsReturnsTwoDateFieldDefs__2442() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___209 = temper_std::testing::Test::new();
        let mut t___5818: temper_core::List<FieldDef>;
        let ts__1831: temper_core::List<FieldDef>;
        'ok___15666: {
            'orelse___2600: {
                t___5818 = match timestamps() {
                    Ok(x) => x,
                    _ => break 'orelse___2600
                };
                ts__1831 = t___5818.clone();
                break 'ok___15666;
            }
            ts__1831 = panic!();
        }
        let mut t___12259: bool = Some(temper_core::ListedTrait::len( & ts__1831)) == Some(2);
        #[derive(Clone)]
        struct ClosureGroup___225 {}
        impl ClosureGroup___225 {
            fn fn__12256(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should return 2 fields".to_string());
            }
        }
        let closure_group = ClosureGroup___225 {};
        let fn__12256 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12256())
        };
        test___209.assert(t___12259, fn__12256.clone());
        let mut t___12265: bool = Some(temper_core::ListedTrait::get( & ts__1831, 0).name().sql_value().as_str()) == Some("inserted_at");
        #[derive(Clone)]
        struct ClosureGroup___226 {}
        impl ClosureGroup___226 {
            fn fn__12255(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("first should be inserted_at".to_string());
            }
        }
        let closure_group = ClosureGroup___226 {};
        let fn__12255 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12255())
        };
        test___209.assert(t___12265, fn__12255.clone());
        let mut t___12271: bool = Some(temper_core::ListedTrait::get( & ts__1831, 1).name().sql_value().as_str()) == Some("updated_at");
        #[derive(Clone)]
        struct ClosureGroup___227 {}
        impl ClosureGroup___227 {
            fn fn__12254(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("second should be updated_at".to_string());
            }
        }
        let closure_group = ClosureGroup___227 {};
        let fn__12254 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12254())
        };
        test___209.assert(t___12271, fn__12254.clone());
        let mut t___12274: bool = temper_core::ListedTrait::get( & ts__1831, 0).nullable();
        #[derive(Clone)]
        struct ClosureGroup___228 {}
        impl ClosureGroup___228 {
            fn fn__12253(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inserted_at should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___228 {};
        let fn__12253 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12253())
        };
        test___209.assert(t___12274, fn__12253.clone());
        let mut t___12278: bool = temper_core::ListedTrait::get( & ts__1831, 1).nullable();
        #[derive(Clone)]
        struct ClosureGroup___229 {}
        impl ClosureGroup___229 {
            fn fn__12252(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("updated_at should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___229 {};
        let fn__12252 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12252())
        };
        test___209.assert(t___12278, fn__12252.clone());
        let mut t___12284: bool = ! temper_core::ListedTrait::get( & ts__1831, 0).default_value().is_none();
        #[derive(Clone)]
        struct ClosureGroup___230 {}
        impl ClosureGroup___230 {
            fn fn__12251(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inserted_at should have default".to_string());
            }
        }
        let closure_group = ClosureGroup___230 {};
        let fn__12251 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12251())
        };
        test___209.assert(t___12284, fn__12251.clone());
        let mut t___12289: bool = ! temper_core::ListedTrait::get( & ts__1831, 1).default_value().is_none();
        #[derive(Clone)]
        struct ClosureGroup___231 {}
        impl ClosureGroup___231 {
            fn fn__12250(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("updated_at should have default".to_string());
            }
        }
        let closure_group = ClosureGroup___231 {};
        let fn__12250 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12250())
        };
        test___209.assert(t___12289, fn__12250.clone());
        test___209.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefDefaultValueField__2443() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___210 = temper_std::testing::Test::new();
        let mut t___5805: SafeIdentifier;
        let mut t___5806: SafeIdentifier;
        let mut t___5810: SafeIdentifier;
        let mut t___5811: SafeIdentifier;
        'ok___15667: {
            'orelse___2601: {
                t___5805 = match safe_identifier("status") {
                    Ok(x) => x,
                    _ => break 'orelse___2601
                };
                t___5806 = t___5805.clone();
                break 'ok___15667;
            }
            t___5806 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12237: StringField = StringField::new();
        let mut t___12238: SqlDefault = SqlDefault::new();
        let withDefault__1833: FieldDef = FieldDef::new(t___5806.clone(), FieldType::new(t___12237.clone()), false, Some(SqlPart::new(t___12238.clone())), false);
        'ok___15668: {
            'orelse___2602: {
                t___5810 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2602
                };
                t___5811 = t___5810.clone();
                break 'ok___15668;
            }
            t___5811 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12240: StringField = StringField::new();
        let withoutDefault__1834: FieldDef = FieldDef::new(t___5811.clone(), FieldType::new(t___12240.clone()), false, None, false);
        let mut t___12244: bool = ! withDefault__1833.default_value().is_none();
        #[derive(Clone)]
        struct ClosureGroup___232 {}
        impl ClosureGroup___232 {
            fn fn__12236(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have default".to_string());
            }
        }
        let closure_group = ClosureGroup___232 {};
        let fn__12236 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12236())
        };
        test___210.assert(t___12244, fn__12236.clone());
        let mut t___12248: bool = withoutDefault__1834.default_value().is_none();
        #[derive(Clone)]
        struct ClosureGroup___233 {}
        impl ClosureGroup___233 {
            fn fn__12235(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should not have default".to_string());
            }
        }
        let closure_group = ClosureGroup___233 {};
        let fn__12235 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12235())
        };
        test___210.assert(t___12248, fn__12235.clone());
        test___210.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefVirtualFlag__2444() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___211 = temper_std::testing::Test::new();
        let mut t___5793: SafeIdentifier;
        let mut t___5794: SafeIdentifier;
        let mut t___5797: SafeIdentifier;
        let mut t___5798: SafeIdentifier;
        'ok___15669: {
            'orelse___2603: {
                t___5793 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___2603
                };
                t___5794 = t___5793.clone();
                break 'ok___15669;
            }
            t___5794 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12224: StringField = StringField::new();
        let normal__1836: FieldDef = FieldDef::new(t___5794.clone(), FieldType::new(t___12224.clone()), false, None, false);
        'ok___15670: {
            'orelse___2604: {
                t___5797 = match safe_identifier("full_name") {
                    Ok(x) => x,
                    _ => break 'orelse___2604
                };
                t___5798 = t___5797.clone();
                break 'ok___15670;
            }
            t___5798 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___12226: StringField = StringField::new();
        let virt__1837: FieldDef = FieldDef::new(t___5798.clone(), FieldType::new(t___12226.clone()), true, None, true);
        let mut t___12230: bool = ! normal__1836.r#virtual();
        #[derive(Clone)]
        struct ClosureGroup___234 {}
        impl ClosureGroup___234 {
            fn fn__12223(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("normal field should not be virtual".to_string());
            }
        }
        let closure_group = ClosureGroup___234 {};
        let fn__12223 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12223())
        };
        test___211.assert(t___12230, fn__12223.clone());
        let mut t___12232: bool = virt__1837.r#virtual();
        #[derive(Clone)]
        struct ClosureGroup___235 {}
        impl ClosureGroup___235 {
            fn fn__12222(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("virtual field should be virtual".to_string());
            }
        }
        let closure_group = ClosureGroup___235 {};
        let fn__12222 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12222())
        };
        test___211.assert(t___12232, fn__12222.clone());
        test___211.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__2445() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___215 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___236 {}
        impl ClosureGroup___236 {
            fn build__1967(& self, name__1969: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1969 = name__1969.to_arc_string();
                let mut t___12204: SqlBuilder = SqlBuilder::new();
                t___12204.append_safe("select * from hi where name = ");
                t___12204.append_string(name__1969.clone());
                return t___12204.accumulated().to_string();
            }
            fn buildWrong__1968(& self, name__1971: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1971 = name__1971.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__1971.clone()));
            }
        }
        let closure_group = ClosureGroup___236 {};
        let build__1967 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1969: std::sync::Arc<String> | closure_group.build__1967(name__1969))
        };
        let buildWrong__1968 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1971: std::sync::Arc<String> | closure_group.buildWrong__1968(name__1971))
        };
        let actual___2447: std::sync::Arc<String> = build__1967(std::sync::Arc::new("world".to_string()));
        let mut t___12214: bool = Some(actual___2447.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___237 {
            actual___2447: std::sync::Arc<String>
        }
        impl ClosureGroup___237 {
            fn fn__12211(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___2447.clone()));
            }
        }
        let closure_group = ClosureGroup___237 {
            actual___2447: actual___2447.clone()
        };
        let fn__12211 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12211())
        };
        test___215.assert(t___12214, fn__12211.clone());
        let bobbyTables__1973: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___2449: std::sync::Arc<String> = build__1967(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___12218: bool = Some(actual___2449.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___238 {
            actual___2449: std::sync::Arc<String>
        }
        impl ClosureGroup___238 {
            fn fn__12210(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___2449.clone()));
            }
        }
        let closure_group = ClosureGroup___238 {
            actual___2449: actual___2449.clone()
        };
        let fn__12210 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12210())
        };
        test___215.assert(t___12218, fn__12210.clone());
        #[derive(Clone)]
        struct ClosureGroup___239 {}
        impl ClosureGroup___239 {
            fn fn__12209(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___239 {};
        let fn__12209 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12209())
        };
        test___215.assert(true, fn__12209.clone());
        test___215.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__2453() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___216 = temper_std::testing::Test::new();
        let mut t___12172: SqlBuilder = SqlBuilder::new();
        t___12172.append_safe("v = ");
        t___12172.append_string("");
        let actual___2454: std::sync::Arc<String> = t___12172.accumulated().to_string();
        let mut t___12178: bool = Some(actual___2454.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___240 {
            actual___2454: std::sync::Arc<String>
        }
        impl ClosureGroup___240 {
            fn fn__12171(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___2454.clone()));
            }
        }
        let closure_group = ClosureGroup___240 {
            actual___2454: actual___2454.clone()
        };
        let fn__12171 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12171())
        };
        test___216.assert(t___12178, fn__12171.clone());
        let mut t___12180: SqlBuilder = SqlBuilder::new();
        t___12180.append_safe("v = ");
        t___12180.append_string("a''b");
        let actual___2457: std::sync::Arc<String> = t___12180.accumulated().to_string();
        let mut t___12186: bool = Some(actual___2457.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___241 {
            actual___2457: std::sync::Arc<String>
        }
        impl ClosureGroup___241 {
            fn fn__12170(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___2457.clone()));
            }
        }
        let closure_group = ClosureGroup___241 {
            actual___2457: actual___2457.clone()
        };
        let fn__12170 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12170())
        };
        test___216.assert(t___12186, fn__12170.clone());
        let mut t___12188: SqlBuilder = SqlBuilder::new();
        t___12188.append_safe("v = ");
        t___12188.append_string("Hello 世界");
        let actual___2460: std::sync::Arc<String> = t___12188.accumulated().to_string();
        let mut t___12194: bool = Some(actual___2460.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___242 {
            actual___2460: std::sync::Arc<String>
        }
        impl ClosureGroup___242 {
            fn fn__12169(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___2460.clone()));
            }
        }
        let closure_group = ClosureGroup___242 {
            actual___2460: actual___2460.clone()
        };
        let fn__12169 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12169())
        };
        test___216.assert(t___12194, fn__12169.clone());
        let mut t___12196: SqlBuilder = SqlBuilder::new();
        t___12196.append_safe("v = ");
        t___12196.append_string("Line1\x0aLine2");
        let actual___2463: std::sync::Arc<String> = t___12196.accumulated().to_string();
        let mut t___12202: bool = Some(actual___2463.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___243 {
            actual___2463: std::sync::Arc<String>
        }
        impl ClosureGroup___243 {
            fn fn__12168(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___2463.clone()));
            }
        }
        let closure_group = ClosureGroup___243 {
            actual___2463: actual___2463.clone()
        };
        let fn__12168 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12168())
        };
        test___216.assert(t___12202, fn__12168.clone());
        test___216.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__2466() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___217 = temper_std::testing::Test::new();
        let mut t___5738: temper_std::temporal::Date;
        let mut t___12143: SqlBuilder = SqlBuilder::new();
        t___12143.append_safe("select ");
        t___12143.append_int32(42);
        t___12143.append_safe(", ");
        t___12143.append_int64(43);
        t___12143.append_safe(", ");
        t___12143.append_float64(19.99f64);
        t___12143.append_safe(", ");
        t___12143.append_boolean(true);
        t___12143.append_safe(", ");
        t___12143.append_boolean(false);
        let actual___2467: std::sync::Arc<String> = t___12143.accumulated().to_string();
        let mut t___12157: bool = Some(actual___2467.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___244 {
            actual___2467: std::sync::Arc<String>
        }
        impl ClosureGroup___244 {
            fn fn__12142(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___2467.clone()));
            }
        }
        let closure_group = ClosureGroup___244 {
            actual___2467: actual___2467.clone()
        };
        let fn__12142 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12142())
        };
        test___217.assert(t___12157, fn__12142.clone());
        let date__1976: temper_std::temporal::Date;
        'ok___15671: {
            'orelse___2605: {
                t___5738 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___2605
                };
                date__1976 = t___5738.clone();
                break 'ok___15671;
            }
            date__1976 = panic!();
        }
        let mut t___12159: SqlBuilder = SqlBuilder::new();
        t___12159.append_safe("insert into t values (");
        t___12159.append_date(date__1976.clone());
        t___12159.append_safe(")");
        let actual___2470: std::sync::Arc<String> = t___12159.accumulated().to_string();
        let mut t___12166: bool = Some(actual___2470.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___245 {
            actual___2470: std::sync::Arc<String>
        }
        impl ClosureGroup___245 {
            fn fn__12141(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___2470.clone()));
            }
        }
        let closure_group = ClosureGroup___245 {
            actual___2470: actual___2470.clone()
        };
        let fn__12141 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12141())
        };
        test___217.assert(t___12166, fn__12141.clone());
        test___217.soft_fail_to_hard()
    }
    #[test]
    fn lists__2473() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___218 = temper_std::testing::Test::new();
        let mut t___5710: temper_std::temporal::Date;
        let mut t___5711: temper_std::temporal::Date;
        let mut t___5712: temper_std::temporal::Date;
        let mut t___5713: temper_std::temporal::Date;
        let mut t___12087: SqlBuilder = SqlBuilder::new();
        t___12087.append_safe("v IN (");
        t___12087.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___12087.append_safe(")");
        let actual___2474: std::sync::Arc<String> = t___12087.accumulated().to_string();
        let mut t___12094: bool = Some(actual___2474.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___246 {
            actual___2474: std::sync::Arc<String>
        }
        impl ClosureGroup___246 {
            fn fn__12086(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___2474.clone()));
            }
        }
        let closure_group = ClosureGroup___246 {
            actual___2474: actual___2474.clone()
        };
        let fn__12086 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12086())
        };
        test___218.assert(t___12094, fn__12086.clone());
        let mut t___12096: SqlBuilder = SqlBuilder::new();
        t___12096.append_safe("v IN (");
        t___12096.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___12096.append_safe(")");
        let actual___2477: std::sync::Arc<String> = t___12096.accumulated().to_string();
        let mut t___12103: bool = Some(actual___2477.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___247 {
            actual___2477: std::sync::Arc<String>
        }
        impl ClosureGroup___247 {
            fn fn__12085(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___2477.clone()));
            }
        }
        let closure_group = ClosureGroup___247 {
            actual___2477: actual___2477.clone()
        };
        let fn__12085 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12085())
        };
        test___218.assert(t___12103, fn__12085.clone());
        let mut t___12105: SqlBuilder = SqlBuilder::new();
        t___12105.append_safe("v IN (");
        t___12105.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___12105.append_safe(")");
        let actual___2480: std::sync::Arc<String> = t___12105.accumulated().to_string();
        let mut t___12112: bool = Some(actual___2480.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___248 {
            actual___2480: std::sync::Arc<String>
        }
        impl ClosureGroup___248 {
            fn fn__12084(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___2480.clone()));
            }
        }
        let closure_group = ClosureGroup___248 {
            actual___2480: actual___2480.clone()
        };
        let fn__12084 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12084())
        };
        test___218.assert(t___12112, fn__12084.clone());
        let mut t___12114: SqlBuilder = SqlBuilder::new();
        t___12114.append_safe("v IN (");
        t___12114.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___12114.append_safe(")");
        let actual___2483: std::sync::Arc<String> = t___12114.accumulated().to_string();
        let mut t___12121: bool = Some(actual___2483.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___249 {
            actual___2483: std::sync::Arc<String>
        }
        impl ClosureGroup___249 {
            fn fn__12083(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___2483.clone()));
            }
        }
        let closure_group = ClosureGroup___249 {
            actual___2483: actual___2483.clone()
        };
        let fn__12083 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12083())
        };
        test___218.assert(t___12121, fn__12083.clone());
        let mut t___12123: SqlBuilder = SqlBuilder::new();
        t___12123.append_safe("v IN (");
        t___12123.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___12123.append_safe(")");
        let actual___2486: std::sync::Arc<String> = t___12123.accumulated().to_string();
        let mut t___12130: bool = Some(actual___2486.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___250 {
            actual___2486: std::sync::Arc<String>
        }
        impl ClosureGroup___250 {
            fn fn__12082(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___2486.clone()));
            }
        }
        let closure_group = ClosureGroup___250 {
            actual___2486: actual___2486.clone()
        };
        let fn__12082 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12082())
        };
        test___218.assert(t___12130, fn__12082.clone());
        'ok___15672: {
            'orelse___2606: {
                t___5710 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___2606
                };
                t___5711 = t___5710.clone();
                break 'ok___15672;
            }
            t___5711 = panic!();
        }
        'ok___15673: {
            'orelse___2607: {
                t___5712 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___2607
                };
                t___5713 = t___5712.clone();
                break 'ok___15673;
            }
            t___5713 = panic!();
        }
        let dates__1978: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___5711.clone(), t___5713.clone()]);
        let mut t___12132: SqlBuilder = SqlBuilder::new();
        t___12132.append_safe("v IN (");
        t___12132.append_date_list(temper_core::ToListed::to_listed(dates__1978.clone()));
        t___12132.append_safe(")");
        let actual___2489: std::sync::Arc<String> = t___12132.accumulated().to_string();
        let mut t___12139: bool = Some(actual___2489.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___251 {
            actual___2489: std::sync::Arc<String>
        }
        impl ClosureGroup___251 {
            fn fn__12081(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___2489.clone()));
            }
        }
        let closure_group = ClosureGroup___251 {
            actual___2489: actual___2489.clone()
        };
        let fn__12081 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12081())
        };
        test___218.assert(t___12139, fn__12081.clone());
        test___218.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__2492() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___219 = temper_std::testing::Test::new();
        let nan__1980: f64;
        nan__1980 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___12073: SqlBuilder = SqlBuilder::new();
        t___12073.append_safe("v = ");
        t___12073.append_float64(nan__1980);
        let actual___2493: std::sync::Arc<String> = t___12073.accumulated().to_string();
        let mut t___12079: bool = Some(actual___2493.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___252 {
            actual___2493: std::sync::Arc<String>
        }
        impl ClosureGroup___252 {
            fn fn__12072(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___2493.clone()));
            }
        }
        let closure_group = ClosureGroup___252 {
            actual___2493: actual___2493.clone()
        };
        let fn__12072 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12072())
        };
        test___219.assert(t___12079, fn__12072.clone());
        test___219.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__2496() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___220 = temper_std::testing::Test::new();
        let inf__1982: f64;
        inf__1982 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___12064: SqlBuilder = SqlBuilder::new();
        t___12064.append_safe("v = ");
        t___12064.append_float64(inf__1982);
        let actual___2497: std::sync::Arc<String> = t___12064.accumulated().to_string();
        let mut t___12070: bool = Some(actual___2497.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___253 {
            actual___2497: std::sync::Arc<String>
        }
        impl ClosureGroup___253 {
            fn fn__12063(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___2497.clone()));
            }
        }
        let closure_group = ClosureGroup___253 {
            actual___2497: actual___2497.clone()
        };
        let fn__12063 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12063())
        };
        test___220.assert(t___12070, fn__12063.clone());
        test___220.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__2500() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___221 = temper_std::testing::Test::new();
        let ninf__1984: f64;
        ninf__1984 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___12055: SqlBuilder = SqlBuilder::new();
        t___12055.append_safe("v = ");
        t___12055.append_float64(ninf__1984);
        let actual___2501: std::sync::Arc<String> = t___12055.accumulated().to_string();
        let mut t___12061: bool = Some(actual___2501.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___254 {
            actual___2501: std::sync::Arc<String>
        }
        impl ClosureGroup___254 {
            fn fn__12054(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___2501.clone()));
            }
        }
        let closure_group = ClosureGroup___254 {
            actual___2501: actual___2501.clone()
        };
        let fn__12054 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12054())
        };
        test___221.assert(t___12061, fn__12054.clone());
        test___221.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__2504() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___222 = temper_std::testing::Test::new();
        let mut t___12030: SqlBuilder = SqlBuilder::new();
        t___12030.append_safe("v = ");
        t___12030.append_float64(3.14f64);
        let actual___2505: std::sync::Arc<String> = t___12030.accumulated().to_string();
        let mut t___12036: bool = Some(actual___2505.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___255 {
            actual___2505: std::sync::Arc<String>
        }
        impl ClosureGroup___255 {
            fn fn__12029(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___2505.clone()));
            }
        }
        let closure_group = ClosureGroup___255 {
            actual___2505: actual___2505.clone()
        };
        let fn__12029 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12029())
        };
        test___222.assert(t___12036, fn__12029.clone());
        let mut t___12038: SqlBuilder = SqlBuilder::new();
        t___12038.append_safe("v = ");
        t___12038.append_float64(0.0f64);
        let actual___2508: std::sync::Arc<String> = t___12038.accumulated().to_string();
        let mut t___12044: bool = Some(actual___2508.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___256 {
            actual___2508: std::sync::Arc<String>
        }
        impl ClosureGroup___256 {
            fn fn__12028(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___2508.clone()));
            }
        }
        let closure_group = ClosureGroup___256 {
            actual___2508: actual___2508.clone()
        };
        let fn__12028 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12028())
        };
        test___222.assert(t___12044, fn__12028.clone());
        let mut t___12046: SqlBuilder = SqlBuilder::new();
        t___12046.append_safe("v = ");
        t___12046.append_float64(-42.5f64);
        let actual___2511: std::sync::Arc<String> = t___12046.accumulated().to_string();
        let mut t___12052: bool = Some(actual___2511.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___257 {
            actual___2511: std::sync::Arc<String>
        }
        impl ClosureGroup___257 {
            fn fn__12027(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___2511.clone()));
            }
        }
        let closure_group = ClosureGroup___257 {
            actual___2511: actual___2511.clone()
        };
        let fn__12027 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12027())
        };
        test___222.assert(t___12052, fn__12027.clone());
        test___222.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__2514() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___223 = temper_std::testing::Test::new();
        let mut t___5606: temper_std::temporal::Date;
        let d__1987: temper_std::temporal::Date;
        'ok___15674: {
            'orelse___2608: {
                t___5606 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___2608
                };
                d__1987 = t___5606.clone();
                break 'ok___15674;
            }
            d__1987 = panic!();
        }
        let mut t___12019: SqlBuilder = SqlBuilder::new();
        t___12019.append_safe("v = ");
        t___12019.append_date(d__1987.clone());
        let actual___2515: std::sync::Arc<String> = t___12019.accumulated().to_string();
        let mut t___12025: bool = Some(actual___2515.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___258 {
            actual___2515: std::sync::Arc<String>
        }
        impl ClosureGroup___258 {
            fn fn__12018(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___2515.clone()));
            }
        }
        let closure_group = ClosureGroup___258 {
            actual___2515: actual___2515.clone()
        };
        let fn__12018 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__12018())
        };
        test___223.assert(t___12025, fn__12018.clone());
        test___223.soft_fail_to_hard()
    }
    #[test]
    fn nesting__2518() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___224 = temper_std::testing::Test::new();
        let name__1989: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___11987: SqlBuilder = SqlBuilder::new();
        t___11987.append_safe("where p.last_name = ");
        t___11987.append_string("Someone");
        let condition__1990: SqlFragment = t___11987.accumulated();
        let mut t___11991: SqlBuilder = SqlBuilder::new();
        t___11991.append_safe("select p.id from person p ");
        t___11991.append_fragment(condition__1990.clone());
        let actual___2520: std::sync::Arc<String> = t___11991.accumulated().to_string();
        let mut t___11997: bool = Some(actual___2520.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___259 {
            actual___2520: std::sync::Arc<String>
        }
        impl ClosureGroup___259 {
            fn fn__11986(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___2520.clone()));
            }
        }
        let closure_group = ClosureGroup___259 {
            actual___2520: actual___2520.clone()
        };
        let fn__11986 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11986())
        };
        test___224.assert(t___11997, fn__11986.clone());
        let mut t___11999: SqlBuilder = SqlBuilder::new();
        t___11999.append_safe("select p.id from person p ");
        t___11999.append_part(SqlPart::new(condition__1990.to_source()));
        let actual___2523: std::sync::Arc<String> = t___11999.accumulated().to_string();
        let mut t___12006: bool = Some(actual___2523.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___260 {
            actual___2523: std::sync::Arc<String>
        }
        impl ClosureGroup___260 {
            fn fn__11985(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___2523.clone()));
            }
        }
        let closure_group = ClosureGroup___260 {
            actual___2523: actual___2523.clone()
        };
        let fn__11985 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11985())
        };
        test___224.assert(t___12006, fn__11985.clone());
        let parts__1991: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___12010: SqlBuilder = SqlBuilder::new();
        t___12010.append_safe("select ");
        t___12010.append_part_list(parts__1991.clone());
        let actual___2526: std::sync::Arc<String> = t___12010.accumulated().to_string();
        let mut t___12016: bool = Some(actual___2526.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___261 {
            actual___2526: std::sync::Arc<String>
        }
        impl ClosureGroup___261 {
            fn fn__11984(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___2526.clone()));
            }
        }
        let closure_group = ClosureGroup___261 {
            actual___2526: actual___2526.clone()
        };
        let fn__11984 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__11984())
        };
        test___224.assert(t___12016, fn__11984.clone());
        test___224.soft_fail_to_hard()
    }
    use super::*;
}
