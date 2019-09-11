export interface RoomMediaResponse {
    media: Array<Media>
}

export interface Media {
    index: number,
    name: string,
    seconds: number,
    source: string,
    url: string,
}

export type AddMediaLocation = "Next" | "Last";
