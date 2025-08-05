import axios from "axios";

export interface ApiResult<T> {
    code: number;
    message: string;
    data: T;
}

const TOKEN_KEY = "__TOKEN__";

export function setToken(token: string) {
    localStorage.setItem(TOKEN_KEY, token);
}

const instance = axios.create({
    baseURL: "http://0.0.0.0:3000/api",
    headers: {
        "Content-Type": "application/json",
    },
    withCredentials: false,
    timeout: 10000,
    validateStatus: () => true,
});

instance.interceptors.request.use(config => {
    const token = localStorage.getItem(TOKEN_KEY);

    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }

    return config;
});

instance.interceptors.response.use(response => {
    const status = response.status;
    const responseData = response.data;

    const result: ApiResult<any> = {
        code: 0,
        message: "",
        data: null,
    };

    if (status < 200 || status >= 300) {
        result.code = status;
        result.message = responseData?.error || `HTTP Error: ${status}`;
        response.data = result;
        return response;
    }
    
    // 处理业务成功响应: { message, data }
    if (responseData?.data !== undefined) {
        result.code = 0; // 成功码
        result.message = responseData.message || "Success";
        result.data = responseData.data;
        response.data = result;
        return response;
    }
    
    // 处理业务错误响应: { code, error }
    if (responseData?.code !== undefined && responseData?.error !== undefined) {
        result.code = responseData.code;
        result.message = responseData.error;
        response.data = result;
        return response;
    }
    
    // 处理意外的响应格式
    result.code = -1;
    result.message = "Unknown response format";
    result.data = responseData; // 保留原始数据以便调试
    response.data = result;
    return response;
});

export default instance;
