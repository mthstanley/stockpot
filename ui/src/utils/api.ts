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
    urlBase: URL;
    token: string;

    constructor(urlBase: URL) {
        this.urlBase = urlBase;
        this.token = "";
    }

    createUser(request: CreateUserRequest): GetUserResponse {
        return {id: -1, name: request.name};
    }

    login(username: string, password: string): GetTokenResponse {
        console.log(username, password);
        const token: string = "secret";
        this.token = token;
        return {token};
    }

    logout() {
        this.token = "";
    }
}

export const apiClient = new ApiClient(config.apiBaseUrl)
