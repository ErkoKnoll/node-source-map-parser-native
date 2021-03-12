const addon: NativeAddon = require('../native');

export function parseSourceMap(sourceMap: string): ParsedSourceMap {
    const index = addon.parseSourceMap(sourceMap);

    return {
        lookupOriginalPosition: (line, column) => addon.lookupOriginalPosition(index, line, column),
        dispose: () => addon.dispose(index)
    }
}

export interface ParsedSourceMap {
    lookupOriginalPosition(line: number, column: number): LookupResult;
    dispose(): void;
}

export interface LookupResult {
    readonly line: number;
    readonly column: number;
    readonly source: string;
}

interface NativeAddon {
    parseSourceMap(file: string): number;
    lookupOriginalPosition(index: number, line: number, column: number): LookupResult;
    dispose(index: number): number;
}