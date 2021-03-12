export declare function parseSourceMap(sourceMap: string): ParsedSourceMap;
export interface ParsedSourceMap {
    lookupOriginalPosition(line: number, column: number): LookupResult;
    dispose(): void;
}
export interface LookupResult {
    readonly line: number;
    readonly column: number;
    readonly source: string;
}
