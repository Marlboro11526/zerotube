import React, { Component, ChangeEvent } from "react";
import { RoomResponse } from "../messages/room";
import ErrorResponse from "../messages/error";
import { Media, RoomMediaResponse } from "../messages/media";
import { Input, Button } from "trunx";
import { toast } from "react-toastify";

interface RoomProperties {
    room: RoomResponse,
}

interface RoomState {
    newMediaInput: string,
    playlist: Array<Media>,
}

export default class Room extends Component<RoomProperties, RoomState> {
    constructor(props: RoomProperties) {
        super(props);

        this.handleNewMediaChange = this.handleNewMediaChange.bind(this);
    }

    async componentDidMount(): Promise<void> {
        try {
            const response = await this.getMediaForRoom();

            this.setState({
                playlist: response.media
            });
        } catch (e) {
            toast("Error on getting media for room.", { type: "error" });
        }
    }

    async getMediaForRoom(): Promise<RoomMediaResponse> {
        let response = await fetch("https://localhost:8443/room/media/get/" + this.props.room.url, {
            method: "GET",
            credentials: "include",
        });

        if (response.ok) {
            return await response.json() as RoomMediaResponse;
        } else {
            let error = await response.json() as ErrorResponse;
            console.log("Error on getting media for room: " + error);

            throw new Error("Error on getting media for room: " + error);
        }
    }

    handleNewMediaChange(event: ChangeEvent<HTMLInputElement>): void {
        this.setState({
            newMediaInput: event.target.value
        });
    }

    async addNewMedia(): Promise<void> {
        const url = this.state.newMediaInput;

        if (!url || url.length === 0) {
            return;
        }

        let response = await fetch("https://localhost:8443/room/media/add/" + this.props.room.url, {
            method: "POST",
            credentials: "include",
            headers: new Headers([["Content-Type", "application/json"]]),
            body: JSON.stringify({ url })
        });

        // not implemented on backend yet
        if (response.ok) {
            await this.getMediaForRoom();
        } else {
            let error = await response.json() as ErrorResponse;
            console.log("Error on adding media to room: " + error);
            toast("Unable to add media to room.", { type: "error" });
        }
    }

    render(): JSX.Element {
        let content;

        if (this.state && this.state.playlist) {
            content = this.state.playlist.map(media => <p key={media.url}>{media.name} - {media.seconds} - {media.url}</p>);
        }

        return (
            <>
                <p>{this.props.room.name} - {this.props.room.description}</p>
                {content}
                <div className="columns">
                    <div className="column is-10">
                        <Input
                            name="newMedia"
                            type="text"
                            onChange={this.handleNewMediaChange} />
                    </div>
                    <div className="column is-2">
                        <Button isPrimary isFullwidth onClick={() => this.addNewMedia()}>Add</Button>
                    </div>
                </div>
            </>
        );
    }

    static async getRoom(url: string): Promise<RoomResponse> {
        let response = await fetch("https://localhost:8443/rooms/get/" + url, {
            method: "GET",
            credentials: "include",
        });

        if (response.ok) {
            return await response.json() as RoomResponse;
        } else {
            let error = await response.json() as ErrorResponse;
            console.log("Error on getting room: " + error);

            throw new Error("Error on getting room: " + error);
        }
    }

    static isPotentialRoomUrl(url: string): boolean {
        const RESERVED_NAMES = ["confirm", "rooms"];
        let urlParts = url.split("/");

        return (urlParts[0].length > 0) && !RESERVED_NAMES.includes(urlParts[0]);
    }
}

