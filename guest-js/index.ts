import {
    list_databases,
    open_database,
    close_database,
    insert,
    insert_one,
    find_all,
    find_many,
    find_one,
    delete_all,
    delete_many,
    delete_one,
    update_all,
    update_many,
    update_one,
    list_collections,
} from "./commands";

import {
    Result,
    ResultFailure,
    ResultSuccess,
    isError,
    isSuccess,
} from "./types";

import { Database, Collection, Document } from "./wrapper";

export {
    list_databases,
    open_database,
    close_database,
    insert,
    insert_one,
    find_all,
    find_many,
    find_one,
    delete_all,
    delete_many,
    delete_one,
    update_all,
    update_many,
    update_one,
    isError,
    isSuccess,
    list_collections,
    Database,
    Collection,
    Document,
};

export type { Result, ResultFailure, ResultSuccess };
