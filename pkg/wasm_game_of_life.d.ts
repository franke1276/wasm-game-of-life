/* tslint:disable */
export enum Cell {Dead,Alive,}
export class Universe {
free(): void;

 width(): number;

 height(): number;

 cells(): number;

 reset(): void;

static  create(): Universe;

 render(): string;

 stop(): void;

 generate_pattern(): void;

 toggle_start_stop(): void;

 toggle_cell(arg0: number, arg1: number): void;

 tick(): void;

}
