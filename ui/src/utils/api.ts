import {config} from "../config";

interface GetUserResponse {
    id: number;
    name: string;
}

interface CreateUserRequest {
    username: string;
    password: string;
    name: string;
}

interface GetTokenResponse {
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
        console.log(request);
        return {id: -1, name: request.name};
    }

    login(username: string, password: string): GetTokenResponse {
        console.log(username, password);
        let token: string = "secret";
        this.token = token;
        return {token};
    }

    logout() {
        this.token = "";
    }
}

export const apiClient = new ApiClient(config.apiBaseUrl)
