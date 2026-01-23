import axios, { type AxiosRequestConfig } from 'axios'

const BASE_URL = 'http://localhost:8080/api'

const axiosInstance = axios.create({
  baseURL: BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
})

export async function request<T>(
  endpoint: string,
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' = 'GET',
  body?: unknown,
  token?: string
): Promise<T> {
  const config: AxiosRequestConfig = {
    url: endpoint,
    method,
    data: body,
    headers: {},
  }

  if (token) {
    config.headers!['Authorization'] = `Bearer ${token}`
  }

  try {
    const response = await axiosInstance.request<T>(config)
    return response.data
  } catch (error: unknown) {
    if (axios.isAxiosError(error)) {
      if (error.response) {
        console.error(error);
        throw new Error(error.response.data.error || 'Something went wrong')
      }
    }
    if (error instanceof Error) {
      throw new Error(error.message || 'Network error')
    }
    throw new Error('An unknown error occurred')
  }
}
