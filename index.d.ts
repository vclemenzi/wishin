/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Config {
  width: number
  height: number
  path: string
  title: string
}
export class Application {
  config: Config
  constructor(config: Config)
  run(): void
}
