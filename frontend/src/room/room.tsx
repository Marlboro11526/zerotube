import React, { ChangeEvent,Component } from "react";
import { toast } from "react-toastify";
import { Button,Input } from "trunx";

import ErrorResponse from "../messages/error";
import { AddMediaLocation,Media, RoomMediaResponse } from "../messages/media";
import { RoomResponse } from "../messages/room";

interface RoomProperties {
    room: RoomResponse,
}

interface RoomState {
    newMediaInput: string,
    nowPlaying: Media,
    playlist: Array<Media>,
}

export default class Room extends Component<RoomProperties, RoomState> {
    constructor(props: RoomProperties) {
        super(props);

        this.handleNewMediaChange = this.handleNewMediaChange.bind(this);
    }

    async componentDidMount(): Promise<void> {
        try {
            await this.updateMediaState();
        } catch (e) {
            toast("Error on getting media for room.", { type: "error" });
        }
    }

    async updateMediaState(): Promise<void> {
        const response = await this.getMediaForRoom();

        this.setState({
            nowPlaying: response.media[0], // temp hack, not managing playing on the server/client yet
            playlist: response.media,
        });
    }

    async getMediaForRoom(): Promise<RoomMediaResponse> {
        const response = await fetch("https://localhost:8443/room/media/get/" + this.props.room.url, {
            method: "GET",
            credentials: "include",
        });

        if (response.ok) {
            return await response.json() as RoomMediaResponse;
        } else {
            const error = await response.json() as ErrorResponse;
            console.log("Error on getting media for room: " + error);

            throw new Error("Error on getting media for room: " + error);
        }
    }

    handleNewMediaChange(event: ChangeEvent<HTMLInputElement>): void {
        this.setState({
            newMediaInput: event.target.value
        });
    }

    async addNewMedia(location: AddMediaLocation): Promise<void> {
        const current = this.state.nowPlaying?.index ?? 0;
        const url = this.state.newMediaInput;

        if (!url || url.length === 0) {
            return;
        }

        const response = await fetch("https://localhost:8443/room/media/add/" + this.props.room.url, {
            method: "POST",
            credentials: "include",
            headers: new Headers([["Content-Type", "application/json"]]),
            body: JSON.stringify({ current, location, url })
        });

        if (response.ok) {
            await this.updateMediaState();
        } else {
            const error = await response.json() as ErrorResponse;
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
                        <div className="buttons has-addons flex-children">
                            <Button isPrimary onClick={() => this.addNewMedia("Next")}>Next</Button>
                            <Button isPrimary onClick={() => this.addNewMedia("Last")}>Last</Button>
                        </div>
                    </div>
                </div>
            </>
        );
    }

    static async getRoom(url: string): Promise<RoomResponse> {
        const response = await fetch("https://localhost:8443/rooms/get/" + url, {
            method: "GET",
            credentials: "include",
        });

        if (response.ok) {
            return await response.json() as RoomResponse;
        } else {
            const error = await response.json() as ErrorResponse;
            console.log("Error on getting room: " + error);

            throw new Error("Error on getting room: " + error);
        }
    }

    static isPotentialRoomUrl(url: string): boolean {
        const RESERVED_NAMES = ["confirm", "rooms"];
        const urlParts = url.split("/");

        return (urlParts[0].length > 0) && !RESERVED_NAMES.includes(urlParts[0]);
    }
}

