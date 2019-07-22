export interface RoomCreate {
    description?: string,
    name?: string,
    public: boolean,
    url?: string,
    [key: string]: boolean | string | undefined,
}

export interface RoomAllResponse {
    rooms: Array<RoomResponse>,
}

export interface RoomResponse {
    description: string,
    name: string,
    url: string,
}
