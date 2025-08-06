import type { ApiResult } from "./http";

import http from "./http";

export interface User {
    id: string;
    name: string;
    gender: "male" | "female";
    account: string;
    password?: string;
    mobilePhone: string;
    birthday: string;
    enabled: boolean;
    createdAt: string;
    updatedAt: string;
}

export interface Page<T> {
    page: number;
    size: number;
    total: number;
    items: T[];
}

export interface UserQueryParams {
    keyword?: string;
    page_size?: number;
}

export interface UserParams {
    id?: string;
    name: string;
    gender: "male" | "female";
    account: string;
    password?: string;
    mobilePhone: string;
    birthday: string;
    enabled: boolean;
}

export interface QueryParams {
    id: string,
    name?: string,
    mobilePhone?: string
}

export async function getUserPage(params?: UserQueryParams): Promise<ApiResult<Page<User>>> {
    const { data } = await http.get<ApiResult<Page<User>>>("/users", { params });

    if (data.code !== 0) {
        throw new Error(data.message);
    }

    return data;
}

export async function createUser(params: UserParams): Promise<ApiResult<User>> {
    const { data } = await http.post<ApiResult<User>>("/users", params);

    if (data.code !== 0) {
        throw new Error(data.message);
    }

    return data;
}

export async function updateUser({ id, ...params }: QueryParams): Promise<ApiResult<User>> {
    const { data } = await http.put<ApiResult<User>>(`/users/${id}`, params);

    if (data.code !== 0) {
        throw new Error(data.message);
    }

    return data;
}

export async function deleteUser(id: string): Promise<ApiResult<void>> {
    const { data } = await http.delete<ApiResult<void>>(`/users/${id}`);

    if (data.code !== 0) {
        throw new Error(data.message);
    }

    return data;
}