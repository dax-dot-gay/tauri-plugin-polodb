import { omit } from "lodash";
import {
    close_database,
    delete_all,
    delete_many,
    delete_one,
    find_all,
    find_many,
    find_one,
    insert,
    list_collections,
    list_databases,
    open_database,
    update_all,
    update_many,
    update_one,
} from "./commands";
import { PartialDeep } from "type-fest";

export type Document<T extends object = any> = {
    _id: string | null;
} & T;

export class Database {
    private _key: string;
    private _closed: boolean;

    private constructor(key: string) {
        this._key = key;
        this._closed = false;
    }

    public get key() {
        return this._key;
    }

    public get closed() {
        return this._closed;
    }

    public check(): void {
        if (this.closed) {
            throw Error("Cannot operate on a closed database.");
        }
    }

    public static async open(
        key: string,
        path: string
    ): Promise<Database | null> {
        const db = await open_database(key, path);
        return db.success ? new Database(key) : null;
    }

    public static async list_databases(): Promise<string[]> {
        const result = await list_databases();
        return result.success ? result.data : [];
    }

    public static async attach(key: string): Promise<Database | null> {
        return (await Database.list_databases()).includes(key)
            ? new Database(key)
            : null;
    }

    public async collections(): Promise<string[]> {
        this.check();
        const res = await list_collections(this.key);
        return res.success ? res.data : [];
    }

    public async close(): Promise<boolean> {
        this.check();
        const res = await close_database(this.key);
        if (res.success) {
            this._closed = true;
            return true;
        } else {
            return false;
        }
    }
}

export class Collection<T extends object = any> {
    constructor(private _database: Database, private _name: string) {}

    public get databaseObject() {
        return this._database;
    }

    public get name() {
        return this._name;
    }

    public get database() {
        return this._database.key;
    }

    public check() {
        this._database.check();
    }

    private makeDocuments<T extends object = any>(
        ...objects: { [key: string]: any }[]
    ): Document<T>[] {
        return objects.map(
            (v) =>
                (Object.keys(v).includes("_id")
                    ? Object.keys(v._id).includes("$oid")
                        ? { _id: v._id["$oid"], ...omit(v, "_id") }
                        : { _id: null, ...omit(v, "_id") }
                    : v) as Document<T>
        );
    }

    public async find<Query extends object = PartialDeep<T>>(
        query: Query,
        sort?: any
    ): Promise<Document<T>[]> {
        this.check();
        const result = await find_many(this.database, this.name, query, sort);
        return result.success ? this.makeDocuments<T>(...result.data) : [];
    }

    public async find_one<Query extends object = PartialDeep<T>>(
        query: Query
    ): Promise<Document<T> | null> {
        this.check();
        const result = await find_one(this.database, this.name, query);
        return result.success ? this.makeDocuments<T>(result.data)[0] : null;
    }

    public async all(sort?: any): Promise<Document<T>[]> {
        this.check();
        const result = await find_all(this.database, this.name, sort);
        return result.success ? this.makeDocuments<T>(...result.data) : [];
    }

    public async get(id: string): Promise<Document<T> | null> {
        return await this.find_one({ _id: { $oid: id } });
    }

    public async insert(...documents: T[]): Promise<number | null> {
        this.check();
        const result = await insert(this.database, this.name, documents);
        return result.success ? result.data : null;
    }

    public async delete_many<Query extends object = PartialDeep<T>>(
        query: Query
    ): Promise<number | null> {
        this.check();
        const result = await delete_many(this.database, this.name, query);
        return result.success ? result.data : null;
    }

    public async delete_one<Query extends object = PartialDeep<T>>(
        query: Query
    ): Promise<number | null> {
        this.check();
        const result = await delete_one(this.database, this.name, query);
        return result.success ? result.data : null;
    }

    public async delete_all(): Promise<number | null> {
        this.check();
        const result = await delete_all(this.database, this.name);
        return result.success ? result.data : null;
    }

    public async update_many<Query extends object = PartialDeep<T>>(
        query: Query,
        update: object,
        upsert?: boolean
    ): Promise<number | null> {
        this.check();
        const result = await update_many(
            this.database,
            this.name,
            query,
            update,
            upsert
        );
        return result.success ? result.data : null;
    }

    public async update_one<Query extends object = PartialDeep<T>>(
        query: Query,
        update: object,
        upsert?: boolean
    ): Promise<number | null> {
        this.check();
        const result = await update_one(
            this.database,
            this.name,
            query,
            update,
            upsert
        );
        return result.success ? result.data : null;
    }

    public async update_all(
        update: object,
        upsert?: boolean
    ): Promise<number | null> {
        this.check();
        const result = await update_all(
            this.database,
            this.name,
            update,
            upsert
        );
        return result.success ? result.data : null;
    }
}
