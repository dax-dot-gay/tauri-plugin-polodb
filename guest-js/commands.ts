import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { Result } from "./types";

async function exec<R = any>(
    command: string,
    args?: InvokeArgs
): Promise<Result<R>> {
    try {
        return {
            success: true,
            data: await invoke<R>(`plugin:polodb|${command}`, args),
        };
    } catch (e) {
        try {
            return {
                success: false,
                error: Object.keys(e as object)[0],
                context: (e as { [key: string]: any })[
                    Object.keys(e as object)[0]
                ],
            };
        } catch (u) {
            return {
                success: false,
                error: "unknown",
                context: u,
            };
        }
    }
}

export async function list_databases(): Promise<Result<string[]>> {
    return await exec<string[]>("list_databases");
}

export async function list_collections(
    database: string
): Promise<Result<string[]>> {
    return await exec<string[]>("list_collections", { database });
}

export async function open_database(
    key: string,
    path: string
): Promise<Result<string>> {
    return await exec<string>("open_database", { key, path });
}

export async function close_database(key: string): Promise<Result<string>> {
    return await exec<string>("close_database", { key });
}

export async function insert<T extends object = any>(
    database: string,
    collection: string,
    documents: T[]
): Promise<Result<number>> {
    return await exec<number>("insert", { database, collection, documents });
}

export async function insert_one<T extends object = any>(
    database: string,
    collection: string,
    document: T
): Promise<Result<number>> {
    return await exec<number>("insert_one", { database, collection, document });
}

export async function find_many<
    Document extends object = any,
    Query extends object = any,
    Sorting extends object = any
>(
    database: string,
    collection: string,
    query: Query,
    sorting?: Sorting | null
): Promise<Result<Document[]>> {
    return await exec<Document[]>("find", {
        database,
        collection,
        query,
        sorting: sorting ?? null,
    });
}

export async function find_all<
    Document extends object = any,
    Sorting extends object = any
>(
    database: string,
    collection: string,
    sorting?: Sorting | null
): Promise<Result<Document[]>> {
    return await exec<Document[]>("find_all", {
        database,
        collection,
        sorting: sorting ?? null,
    });
}

export async function find_one<
    Document extends object = any,
    Query extends object = any
>(
    database: string,
    collection: string,
    query: Query
): Promise<Result<Document>> {
    return await exec<Document>("find_one", {
        database,
        collection,
        query,
    });
}

export async function delete_many<Query extends object = any>(
    database: string,
    collection: string,
    query: Query
): Promise<Result<number>> {
    return await exec<number>("delete", {
        database,
        collection,
        query,
    });
}

export async function delete_all(
    database: string,
    collection: string
): Promise<Result<number>> {
    return await exec<number>("delete_all", {
        database,
        collection,
    });
}

export async function delete_one<Query extends object = any>(
    database: string,
    collection: string,
    query: Query
): Promise<Result<number>> {
    return await exec<number>("delete_one", {
        database,
        collection,
        query,
    });
}

export async function update_many<
    Query extends object = any,
    Update extends object = any
>(
    database: string,
    collection: string,
    query: Query,
    update: Update,
    upsert?: boolean
): Promise<Result<number>> {
    return await exec<number>("update", {
        database,
        collection,
        query,
        update,
        upsert: upsert ?? false,
    });
}

export async function update_all<Update extends object = any>(
    database: string,
    collection: string,
    update: Update,
    upsert?: boolean
): Promise<Result<number>> {
    return await exec<number>("update_all", {
        database,
        collection,
        update,
        upsert: upsert ?? false,
    });
}

export async function update_one<
    Query extends object = any,
    Update extends object = any
>(
    database: string,
    collection: string,
    query: Query,
    update: Update,
    upsert?: boolean
): Promise<Result<number>> {
    return await exec<number>("update_one", {
        database,
        collection,
        query,
        update,
        upsert: upsert ?? false,
    });
}
