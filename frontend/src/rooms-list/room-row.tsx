import React, { Component } from "react";

import { RoomResponse } from "../messages/room";

interface RoomRowProperties {
    key: string,
    room: RoomResponse
}

export default class RoomRow extends Component<RoomRowProperties> {
    render(): JSX.Element {
        return (
            <tr>
                <td><a href={this.props.room.url}>{this.props.room.name}</a></td>
                <td>{this.props.room.description}</td>
            </tr>
        );
    }
}