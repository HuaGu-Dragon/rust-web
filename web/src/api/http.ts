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

    const result: ApiResult<unknown> = {
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
    
    if (responseData?.data !== undefined) {
        result.code = 0; 
        result.message = responseData.message || "Success";
        result.data = responseData.data;
        response.data = result;
        return response;
    }
    
    if (responseData?.code !== undefined && responseData?.error !== undefined) {
        result.code = responseData.code;
        result.message = responseData.error;
        response.data = result;
        return response;
    }
    
    result.code = -1;
    result.message = "Unknown response format";
    result.data = responseData;
    response.data = result;
    return response;
});

export default instance;
