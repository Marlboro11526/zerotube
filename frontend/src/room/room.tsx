import { Component } from "react";
import { RoomResponse } from "../messages/room";
import ErrorResponse from "../messages/error";

export default class Room extends Component {


    static async getRoom(url: string): Promise<RoomResponse> {
        return fetch("http://localhost:8081/rooms/get/" + url, {
            method: "GET",
            credentials: "include",
        })
            .then(async response => {
                if (response.ok) {
                    return await response.json() as RoomResponse;
                } else {
                    let error = await response.json() as ErrorResponse;
                    console.log("Error on getting room: " + error);

                    throw new Error("Error on getting room: " + error);
                }
            });
    }

    static isPotentialRoomUrl(url: string): boolean {
        const RESERVED_NAMES = ["confirm", "rooms"];

        return (url.length > 0) && !RESERVED_NAMES.includes(url);
    }
}

