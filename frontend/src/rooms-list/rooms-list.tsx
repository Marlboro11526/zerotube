import React, { Component, ChangeEvent } from "react";
import ErrorResponse from "../messages/error";
import { RoomCreate, RoomResponse, RoomAllResponse } from "../messages/room";
import { Table, Button, Label, Input, Checkbox, Field } from "trunx";
import RoomRow from "./room-row";
import { toast } from "react-toastify";

interface RoomsProperties {

}

interface RoomsState {
    newRoom: RoomCreate,
    rooms?: Array<RoomResponse>,
    showCreateForm: boolean,
}

export default class RoomsList extends Component<RoomsProperties, RoomsState> {
    constructor(props: RoomsProperties) {
        super(props);

        this.state = {
            newRoom: { public: true },
            showCreateForm: false,
        };

        this.handleInputChange = this.handleInputChange.bind(this);
    }

    componentDidMount(): void {
        this.updateRoomList();
    }

    async create(room: RoomCreate): Promise<void> {
        let response = await fetch("https://localhost:8443/rooms/create", {
            method: "POST",
            credentials: "include",
            headers: new Headers([["Content-Type", "application/json"]]),
            body: JSON.stringify(room)
        });

        if (response.ok) {
            return;
        } else {
            let error = await response.json() as ErrorResponse;
            console.log("Error on creating room: " + error);

            throw new Error("Error on creating room: " + error);
        }
    }

    async handleCreateRoom(): Promise<void> {
        if (!this.state.newRoom.description
            || !this.state.newRoom.name
            || !this.state.newRoom.url) {
            toast("Please fill in the form", { type: "warning" });

            return;
        }

        try {
            await this.create(this.state.newRoom);
            await this.updateRoomList();

            this.setState({
                showCreateForm: false
            });
        } catch (error) {
            toast(error, { type: "error" });
        }
    }

    handleInputChange(event: ChangeEvent<HTMLInputElement>): void {
        const target = event.target;
        const name = target.name;
        const value = target.type === "checkbox" ? target.checked : target.value;

        let newRoom = this.state.newRoom || {};
        newRoom[name] = value;

        this.setState({
            newRoom: newRoom,
        }, () => console.log(this.state.newRoom));
    }

    showCreateForm(isActive: boolean): void {
        this.setState({
            newRoom: { public: true },
            showCreateForm: isActive,
        });
    }

    async updateRoomList(): Promise<void> {
        let response = await fetch("https://localhost:8443/rooms/get", {
            method: "GET",
            credentials: "include",
        });

        if (response.ok) {
            let allRooms = await response.json() as RoomAllResponse;

            this.setState({
                rooms: allRooms.rooms
            });
        } else {
            let error = await response.json() as ErrorResponse;
            console.log("Error on retrieving rooms: " + error);

            throw new Error("Error on retrieving rooms: " + error);
        }
    }

    render(): JSX.Element {
        let createFormModal;
        let rooms;

        if (this.state === null) {
            return <span id="loader" />
        }

        if (this.state.rooms) {
            if (this.state.rooms.length > 0) {
                rooms = (
                    <Table isFullwidth>
                        <thead>
                            <tr>
                                <td>Name</td>
                                <td>Description</td>
                            </tr>
                        </thead>
                        <tbody>
                            {this.state.rooms.map(room => <RoomRow key={room.url} room={room} />)}
                        </tbody>
                    </Table>
                );
            } else {
                rooms = <p>Nothing here!!</p>
            }
        }

        if (this.state.showCreateForm) {
            createFormModal = (
                <div className="modal is-active">
                    <div className="modal-background"></div>
                    <div className="modal-card">
                        <header className="modal-card-head">
                            <p className="modal-card-title">Create Room</p>
                            <button className="delete" aria-label="close" onClick={() => this.showCreateForm(false)}></button>
                        </header>
                        <section className="modal-card-body">
                            <Field>
                                <Label>Name</Label>
                                <Input
                                    name="name"
                                    type="text"
                                    onChange={this.handleInputChange}
                                    autoFocus />
                            </Field>
                            <Field>
                                <Label>Channel URL</Label>
                                <Input
                                    name="url"
                                    type="text"
                                    onChange={this.handleInputChange} />
                            </Field>
                            <Field>
                                <Label>Description</Label>
                                <Input
                                    name="description"
                                    type="text"
                                    onChange={this.handleInputChange} />
                            </Field>
                            <Field>
                                <Checkbox
                                    name="public"
                                    type="checkbox"
                                    defaultChecked={this.state.newRoom.public}
                                    onChange={this.handleInputChange}>
                                    Public
                                </Checkbox>
                            </Field>
                        </section>
                        <footer className="modal-card-foot">
                            <Button isPrimary onClick={() => this.handleCreateRoom()}>Create</Button>
                            <Button data-cancel onClick={() => this.showCreateForm(false)}>Cancel</Button>
                        </footer>
                    </div>
                </div>
            )
        }

        return (
            <>
                <h2 className="title">Rooms</h2>
                <button id="button-create-room" className="button is-primary is-pulled-right" onClick={() => this.showCreateForm(true)}>Create</button>
                {rooms}
                {createFormModal}
            </>
        )
    }
}