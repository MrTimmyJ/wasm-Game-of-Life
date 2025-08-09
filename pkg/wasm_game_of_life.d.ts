/* tslint:disable */
/* eslint-disable */
export enum Cell {
  Dead = 0,
  Alive = 1,
}
export class Universe {
  private constructor();
  free(): void;
  toggle_cell(row: number, column: number): void;
  tick(): void;
  static new(): Universe;
  render(): string;
  width(): number;
  height(): number;
  cells(): number;
  /**
   * Set the width of the universe.
   *
   * Resets all cells to the dead state.
   */
  set_width(width: number): void;
  /**
   * Set the height of the universe.
   *
   * Resets all cells to the dead state.
   */
  set_height(height: number): void;
}
