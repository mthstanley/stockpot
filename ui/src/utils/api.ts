import axios, { AxiosInstance } from "axios";
import { config } from "../config";

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

export interface GetRecipeIngredientResponse {
  id: number;
  ingredient: string;
  quantity: number;
  units: string;
  preparation: string;
}

export interface GetStepResponse {
  id: number;
  ordinal: number;
  instruction: string;
}

export interface GetRecipeResponse {
  id: number;
  title: string;
  description: string | null;
  author: GetUserResponse;
  prepTime: number;
  cookTime: number;
  inactiveTime: number;
  yieldQuantity: number;
  yieldUnits: string;
  ingredients: Set<GetRecipeIngredientResponse>;
  steps: Set<GetStepResponse>;
}

class ApiClient {
  client: AxiosInstance;
  tokenExpirationCallback: VoidFunction;

  constructor(urlBase: URL) {
    this.client = axios.create({ baseURL: urlBase.toString() });
    this.client.defaults.headers.post["Content-Type"] = "application/json";
    this.tokenExpirationCallback = () => {};
    this.client.interceptors.response.use(
      (response) => response,
      (error) => {
        if (error.response) {
          if (error.response.status === 401) {
            this.tokenExpirationCallback();
          }
        }
        return Promise.reject(error);
      },
    );
  }

  async createUser(request: CreateUserRequest): Promise<GetUserResponse> {
    return this.client
      .post<GetUserResponse>("/user", request)
      .then((response) => response.data);
  }

  async login(
    username: string,
    password: string,
    tokenExpirationCallback: VoidFunction,
  ): Promise<GetTokenResponse> {
    return this.client
      .post<GetTokenResponse>(
        "/user/token",
        {},
        { auth: { username, password } },
      )
      .then((response) => {
        this.client.defaults.headers.common["Authorization"] =
          `Bearer ${response.data.token}`;
        this.tokenExpirationCallback = tokenExpirationCallback;
        return response.data;
      });
  }

  logout() {
    this.client.defaults.headers.common["Authorization"] = "";
  }

  async getRecipes(): Promise<Set<GetRecipeResponse>> {
    return this.client
      .get<Set<GetRecipeResponse>>("/recipe")
      .then((response) => response.data);
  }

  async getRecipe(id: number): Promise<GetRecipeResponse> {
    return this.client
      .get<GetRecipeResponse>(`/recipe/${id}`)
      .then((response) => response.data);
  }
}

export const apiClient = new ApiClient(config.apiBaseUrl);
