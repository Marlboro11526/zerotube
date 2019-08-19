export interface RoomMediaResponse {
    media: Array<Media>
}

export interface Media {
    name: string,
    seconds: number,
    source: string,
    url: string,
}
