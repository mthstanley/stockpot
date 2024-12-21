import axios, {AxiosInstance} from "axios";
import {config} from "../config";

export interface GetUserResponse {
    id: number;
    name: string;
}

export interface CreateUserRequest {
    username: string;
    password: string;
    name: string;
}

export interface GetTokenResponse {
    token: string;
}

class ApiClient {
    client: AxiosInstance;
    tokenExpirationCallback: VoidFunction;

    constructor(urlBase: URL) {
        this.client = axios.create({baseURL: urlBase.toString()});
        this.client.defaults.headers.post["Content-Type"] = "application/json";
        this.tokenExpirationCallback = () => {};
        this.client.interceptors.response.use(response => response, error => {
            if (error.response) {
                if (error.response.status === 401) {
                    this.tokenExpirationCallback();
                }
            }
            return Promise.reject(error);
        })
    }

    async createUser(request: CreateUserRequest): Promise<GetUserResponse> {
        return this.client.post<CreateUserRequest, GetUserResponse>("/user", request);
    }

    async login(username: string, password: string, tokenExpirationCallback: VoidFunction): Promise<GetTokenResponse> {
        return this.client.post<null, GetTokenResponse>("/user/token", {}, {auth: {username, password}}).then(response => {
            this.client.defaults.headers.common["Authorization"] = `Bearer ${response.token}`;
            this.tokenExpirationCallback = tokenExpirationCallback;
            return response;
        });
    }

    logout() {
        this.client.defaults.headers.common["Authorization"] = "";
    }
}

export const apiClient = new ApiClient(config.apiBaseUrl)
