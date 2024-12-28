/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface DockerOptions {
  socketPath?: string
  url?: string
  sslKey?: string
  sslCert?: string
  sslCa?: string
}
export interface AttachOptions {
  stdin?: boolean
  stdout?: boolean
  stderr?: boolean
  stream?: boolean
  logs?: boolean
}
export declare class AttachOutput {
  write(buf: Buffer): Promise<bigint>
  close(): Promise<void>
  read(buf: Buffer): Promise<bigint>
}
export declare class Docker {
  constructor(options?: DockerOptions | undefined | null)
  version(): Promise<Buffer>
  attach(id: string, option?: AttachOptions | undefined | null): Promise<AttachOutput>
}