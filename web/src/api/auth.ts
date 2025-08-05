import type { ApiResult } from "./http";

import http from "./http";

export interface LoginParams {
    account: string;
    password: string;
}

export interface UserInfo {
    id: string;
    name: string;
}

export async function login(params: LoginParams) {
    const { data } = await http.post<ApiResult<string>>("/auth/login", params);

    if (data.code !== 0) {
        throw new Error(data.message);
    }

    return data.data;
}