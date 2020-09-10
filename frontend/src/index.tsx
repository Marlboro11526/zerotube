import "react-toastify/dist/ReactToastify.css";
import "./static/styles/style.scss";

import React, { Component } from "react";
import ReactDOM from "react-dom";
import { toast } from "react-toastify";
import { Button,Container, Section } from "trunx";

import ConfirmRegister from "./confirm-register/confirm-register";
import Error401 from "./error/error401";
import Error404 from "./error/error404";
import { RoomResponse } from "./messages/room";
import UserResponse from "./messages/user";
import Navbar from "./navbar/navbar";
import Room from "./room/room";
import RoomsList from "./rooms-list/rooms-list";
import Time from "./time/time";

interface AppProperties { }

interface AppState {
    error?: string,
    isLoggedIn?: boolean,
    room?: RoomResponse,
    serverTime?: Date,
    text?: string,
    username?: string,
}

class App extends Component<AppProperties, AppState> {
    async componentDidMount(): Promise<void> {
        let response;

        try {
            response = await fetch("https://localhost:8443/auth/start", {
                method: "GET",
                credentials: "include",
            });

            if (!response.ok) {
                throw new Error("Server did not respond with 200 OK.");
            }
        }
        catch (err) {
            console.log(err);
            toast("Unable to connect to server.", { type: "error" });

            return;
        }

        if (response.ok) {
            const user = await response.json() as UserResponse;
            this.handleAuth(user);
        }

        const url = document.location.pathname.substring(1);

        if (Room.isPotentialRoomUrl(url)) {
            Room.getRoom(url)
                .then(response => {
                    if (response !== null) {
                        this.setState({
                            room: response
                        });
                    }
                });
        }
    }

    async showSecret(): Promise<void> {
        const response = await fetch("https://localhost:8443/web/secret", {
            method: "GET",
            credentials: "include",
        });

        if (response.ok) {
            const text = await response.json();

            this.setState({ text: text });
        } else {
            this.setState({ text: "NOT AUTH'D!!!" });
        }

    }

    handleAuth(response: UserResponse): void {
        if (response && response.username) {
            this.setState({
                isLoggedIn: true,
                username: response.username,
            });
        } else {
            this.setState({
                isLoggedIn: false,
                username: undefined,
            });
        }
    }

    render(): JSX.Element {
        let pageContent;

        // special case for confirming registration, standalone "page"
        if (window.location.pathname.startsWith("/confirm")) {
            return <ConfirmRegister />;
        }

        if (this.state === null || this.state.isLoggedIn === undefined) {
            return <span id="loader" />;
        }

        if (window.location.pathname.startsWith("/rooms")) {
            if (this.state.isLoggedIn) {
                pageContent = <RoomsList />;
            } else {
                return <Error401 />;
            }
        } else if (window.location.pathname === "/") {
            let time;

            if (this.state.isLoggedIn) {
                time = <Time />;
            }

            pageContent =
                <>
                    <Button isPrimary onClick={() => this.showSecret()}>Secret</Button>
                    <p>{this.state.text}</p>
                    {time}
                </>;
        } else {
            if (this.state.room) {
                pageContent = <Room room={this.state.room} />;
            }
            else {
                return <Error404 />;
            }
        }

        console.log("CURRENT STATE:");
        console.log(this.state);

        return (
            <>
                <Navbar
                    authHandler={(response: UserResponse) => this.handleAuth(response)}
                    isLoggedIn={this.state.isLoggedIn}
                    username={this.state.username}
                />
                <Section>
                    <Container>
                        {pageContent}
                    </Container>
                </Section>
            </>
        );
    }
}

toast.configure();
ReactDOM.render(<App />, document.getElementById("root"));
