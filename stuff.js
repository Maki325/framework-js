function _assert_this_initialized(self) {
  if (self === void 0) {
    throw new ReferenceError("this hasn't been initialised - super() hasn't been called");
  }
  return self;
}
function _class_call_check(instance, Constructor) {
  if (!(instance instanceof Constructor)) {
    throw new TypeError("Cannot call a class as a function");
  }
}
function _get_prototype_of(o) {
  _get_prototype_of = Object.setPrototypeOf ? Object.getPrototypeOf : function getPrototypeOf(o) {
    return o.__proto__ || Object.getPrototypeOf(o);
  };
  return _get_prototype_of(o);
}
function _inherits(subClass, superClass) {
  if (typeof superClass !== "function" && superClass !== null) {
    throw new TypeError("Super expression must either be null or a function");
  }
  subClass.prototype = Object.create(superClass && superClass.prototype, {
    constructor: {
      value: subClass,
      writable: true,
      configurable: true
    }
  });
  if (superClass) _set_prototype_of(subClass, superClass);
}
function _possible_constructor_return(self, call) {
  if (call && (_type_of(call) === "object" || typeof call === "function")) {
    return call;
  }
  return _assert_this_initialized(self);
}
function _set_prototype_of(o, p) {
  _set_prototype_of = Object.setPrototypeOf || function setPrototypeOf(o, p) {
    o.__proto__ = p;
    return o;
  };
  return _set_prototype_of(o, p);
}
function _type_of(obj) {
  "@swc/helpers - typeof";
  return obj && typeof Symbol !== "undefined" && obj.constructor === Symbol ? "symbol" : typeof obj;
}
function _is_native_reflect_construct() {
  if (typeof Reflect === "undefined" || !Reflect.construct) return false;
  if (Reflect.construct.sham) return false;
  if (typeof Proxy === "function") return true;
  try {
    Boolean.prototype.valueOf.call(Reflect.construct(Boolean, [], function () { }));
    return true;
  } catch (e) {
    return false;
  }
}
function _create_super(Derived) {
  var hasNativeReflectConstruct = _is_native_reflect_construct();
  return function _createSuperInternal() {
    var Super = _get_prototype_of(Derived), result;
    if (hasNativeReflectConstruct) {
      var NewTarget = _get_prototype_of(this).constructor;
      result = Reflect.construct(Super, arguments, NewTarget);
    } else {
      result = Super.apply(this, arguments);
    }
    return _possible_constructor_return(this, result);
  };
}
var Foo = function Foo() {
  "use strict";
  _class_call_check(this, Foo);
};
var Bar = /*#__PURE__*/ function (Foo) {
  "use strict";
  _inherits(Bar, Foo);
  var _super = _create_super(Bar);
  function Bar() {
    _class_call_check(this, Bar);
    return _super.apply(this, arguments);
  }
  return Bar;
}(Foo);
