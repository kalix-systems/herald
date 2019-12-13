export declare function unwrapOr<T>(maybeVal: T, fallback: T): T;
export declare function friendlyFileSize(byteSize: number): string;
export declare function friendlyTimestamp(msEpochTime: number): string;
export declare function safeSwitch<T>(cond: boolean, first: T, second: T): T | undefined;
export declare function safeToQrcURI(url: string): string;
export declare function safeStringOrDefault(maybeString: unknown, fallback?: unknown): string;
export declare function initialize(name: string): string;
export declare function receiptCodeSwitch(receiptCode: MessageReceiptStatus): string;
