"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.parseSourceMap = void 0;
const addon = require('../native');
function parseSourceMap(sourceMap) {
    const index = addon.parseSourceMap(sourceMap);
    return {
        lookupOriginalPosition: (line, column) => addon.lookupOriginalPosition(index, line, column),
        dispose: () => addon.dispose(index)
    };
}
exports.parseSourceMap = parseSourceMap;
//# sourceMappingURL=index.js.map