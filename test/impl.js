/**
 * 
 * @param {unknown} item The item to stringify
 * @param {Array<() => Promise<void>>} toCreate The components that are async
 * @returns {unknown} The item to stringify
 */
global.___FRAMEWORK_JS_STRINGIFY___ = (item, toCreate) => {
  if (Array.isArray(item)) {
    if (
      item.length == 2 &&
      typeof item[0] === 'string' &&
      typeof item[1] === 'function'
    ) {
      toCreate.push(item[1]);
      return item[0];
    }
    return item.map(value => {
      if (
        Array.isArray(value) &&
        value.length == 2 &&
        typeof value[0] === 'string' &&
        typeof value[1] === 'function'
      ) {
        toCreate.push(value[1]);
        return value[0];
      } else {
        return value;
      }
    }).join('');
  } else if (typeof item === 'object') {
    throw new Error('Objects are not valid as a JSX child!');
  } else {
    return item;
  }
}

const CAPITAL_A = 'A'.charCodeAt(0);
const CAPITAL_Z = 'Z'.charCodeAt(0);

/**
 * 
 * @param {string} name Style name to hyphenate
 * @returns {string} Hyphenated style name
 */
function hyphenateStyleName(name) {
  const hyphenated = new Array(name.length + 10);

  let pos = 0;

  const thirdChar = name.charCodeAt(2);
  if (name.startsWith('ms') && CAPITAL_A <= thirdChar && thirdChar <= CAPITAL_Z) {
    hyphenated[pos++] = '-';
  }

  for (let i = 0; i < name.length; i++) {
    const char = name.charCodeAt(i);
    if (CAPITAL_A <= char && char <= CAPITAL_Z) {
      hyphenated[pos++] = '-';
      hyphenated[pos++] = name.charAt(i).toLowerCase();
    } else {
      hyphenated[pos++] = name.charAt(i);
    }
  }

  return hyphenated.join('');
}

/**
 * 
 * @param {string} value String to be escaped
 * @returns {string} Escaped string
 */
function escapeHtml(value) {
  const hyphenated = new Array(value.length + 20);

  let pos = 0;

  for (let i = 0; i < value.length; i++) {
    const char = value.charCodeAt(i);
    switch (char) {
      case 34: // "
        hyphenated[pos++] = '&';
        hyphenated[pos++] = 'q';
        hyphenated[pos++] = 'u';
        hyphenated[pos++] = 'o';
        hyphenated[pos++] = 't';
        hyphenated[pos++] = ';';
        break;
      case 38: // &
        hyphenated[pos++] = '&';
        hyphenated[pos++] = 'a';
        hyphenated[pos++] = 'm';
        hyphenated[pos++] = 'p';
        hyphenated[pos++] = ';';
        break;
      case 39: // '
        // modified from escape-html; used to be '&#39'
        hyphenated[pos++] = '&';
        hyphenated[pos++] = '#';
        hyphenated[pos++] = 'x';
        hyphenated[pos++] = '2';
        hyphenated[pos++] = '7';
        hyphenated[pos++] = ';';
        break;
      case 60: // <
        hyphenated[pos++] = '&';
        hyphenated[pos++] = 'l';
        hyphenated[pos++] = 't';
        hyphenated[pos++] = ';';
        break;
      case 62: // >
        hyphenated[pos++] = '&';
        hyphenated[pos++] = 'g';
        hyphenated[pos++] = 't';
        hyphenated[pos++] = ';';
        break;
      default:
        hyphenated[pos++] = value[i];
        break;
    }
  }

  return hyphenated.join('');
}

/**
 * Escapes text to prevent scripting attacks.
 *
 * @param {unknown} text Text value to escape.
 * @return {string} An escaped string.
 */
function escapeTextForBrowser(text) {
  if (
    typeof text === 'boolean' ||
    typeof text === 'number' ||
    typeof text === 'bigint'
  ) {
    // this shortcircuit helps perf for types that we know will never have
    // special characters, especially given that this function is used often
    // for numeric dom ids.
    return '' + text;
  }

  return escapeHtml(text);
}

/**
 * @type Map<string, string>
 */
const ___FRAMEWORK_STYLE_NAME_CACHE___ = new Map();

/**
 * 
 * @param {string} name Style name to process
 * @returns {string} Processed style name
 */
global.___FRAMEWORK_JS_STYLE_NAME___ = (name) => {
  const processed = ___FRAMEWORK_STYLE_NAME_CACHE___.get(name);
  if (processed !== undefined) {
    return processed;
  }
  if (name.startsWith("--")) {
    const result = escapeTextForBrowser(name);
    ___FRAMEWORK_STYLE_NAME_CACHE___.set(name, result);
    return result;
  }
  const result = escapeTextForBrowser(hyphenateStyleName(name));
  ___FRAMEWORK_STYLE_NAME_CACHE___.set(name, result);
  return result;
}

const unitlessNumbers = new Set([
  'animationIterationCount',
  'aspectRatio',
  'borderImageOutset',
  'borderImageSlice',
  'borderImageWidth',
  'boxFlex',
  'boxFlexGroup',
  'boxOrdinalGroup',
  'columnCount',
  'columns',
  'flex',
  'flexGrow',
  'flexPositive',
  'flexShrink',
  'flexNegative',
  'flexOrder',
  'gridArea',
  'gridRow',
  'gridRowEnd',
  'gridRowSpan',
  'gridRowStart',
  'gridColumn',
  'gridColumnEnd',
  'gridColumnSpan',
  'gridColumnStart',
  'fontWeight',
  'lineClamp',
  'lineHeight',
  'opacity',
  'order',
  'orphans',
  'scale',
  'tabSize',
  'widows',
  'zIndex',
  'zoom',
  'fillOpacity', // SVG-related properties
  'floodOpacity',
  'stopOpacity',
  'strokeDasharray',
  'strokeDashoffset',
  'strokeMiterlimit',
  'strokeOpacity',
  'strokeWidth',
  'MozAnimationIterationCount', // Known Prefixed Properties
  'MozBoxFlex', // TODO: Remove these since they shouldn't be used in modern code
  'MozBoxFlexGroup',
  'MozLineClamp',
  'msAnimationIterationCount',
  'msFlex',
  'msZoom',
  'msFlexGrow',
  'msFlexNegative',
  'msFlexOrder',
  'msFlexPositive',
  'msFlexShrink',
  'msGridColumn',
  'msGridColumnSpan',
  'msGridRow',
  'msGridRowSpan',
  'WebkitAnimationIterationCount',
  'WebkitBoxFlex',
  'WebKitBoxFlexGroup',
  'WebkitBoxOrdinalGroup',
  'WebkitColumnCount',
  'WebkitColumns',
  'WebkitFlex',
  'WebkitFlexGrow',
  'WebkitFlexPositive',
  'WebkitFlexShrink',
  'WebkitLineClamp',
]);

/**
 * 
 * @param {unknown} styleValue Style value to process
 * @param {string} styleName Name of the style, used for processing
 * @returns {string} Processed style value
 */
global.___FRAMEWORK_JS_STYLE_VALUE___ = (styleValue, styleName) => {
  if (!styleName.startsWith('--') && typeof styleValue === 'number') {
    if (styleValue !== 0 && !unitlessNumbers.has(styleName)) {
      return styleValue + 'px'; // Presumes implicit 'px' suffix for unitless numbers
    } else {
      return '' + styleValue;
    }
  } else {
    return escapeTextForBrowser(('' + styleValue).trim());
  }
}

/**
 * 
 * @param {object} style Style object to process
 * @returns {string} Processed style object
 */
global.___FRAMEWORK_JS_STYLE_OBJECT___ = (style) => {
  return Object.entries(style).map(([key, value]) =>
    `${global.___FRAMEWORK_JS_STYLE_NAME___(key)}: ${global.___FRAMEWORK_JS_STYLE_VALUE___(value, key)}`
  ).join(';');
}
