import { Version } from "../../../domain";

export interface DriverStatus {
    error?: string;
    version?: Version;
}

export interface BoardStatus {
    decoder: BoardDetailedStatus;
    encoder: BoardDetailedStatus;
}

export interface BoardDetailedStatus {
    error?: string;
    connected: boolean;
    retryBarrierSec: number;
}
