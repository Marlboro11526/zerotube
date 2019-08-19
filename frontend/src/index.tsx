import React, { Component } from "react";
import ReactDOM from "react-dom";
import "react-toastify/dist/ReactToastify.css";
import "./static/styles/style.scss";
import * as serviceWorker from "./serviceWorker";
import UserResponse from "./messages/user";
import Time from "./time/time";
import ConfirmRegister from "./confirm-register/confirm-register";
import { toast } from "react-toastify";
import Navbar from "./navbar/navbar";
import RoomsList from "./rooms-list/rooms-list";
import { Section, Container, Button } from "trunx";
import Error404 from "./error/error404";
import { RoomResponse } from "./messages/room";
import Room from "./room/room";
import Error401 from "./error/error401";

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
    componentDidMount(): void {
        fetch("https://localhost:8443/auth/start", {
            method: "GET",
            credentials: "include",
        })
            .then(response => response.json() as Promise<UserResponse>)
            .then(response => {
                if (response !== null && response.username !== null) {
                    this.setState({
                        isLoggedIn: true,
                        username: response.username,
                    });
                } else {
                    this.setState({
                        isLoggedIn: false
                    });
                }
            })
            .catch(() => toast("Unable to connect to server.", { type: "error" }));


        let url = document.location.pathname.substring(1);

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

    showSecret(): void {
        fetch("https://localhost:8443/web/secret", {
            method: "GET",
            credentials: "include",
        })
            .then(response => {
                if (response.ok) {
                    response.json().then(json => {
                        this.setState({
                            text: json
                        })
                    });

                    return;
                } else {
                    this.setState({
                        text: "NOT AUTH'D!!!"
                    });
                }
            })
            .catch(e => console.log(e))
    }

    handleAuth(response: UserResponse): void {
        console.log("AUTHING");
        console.log(response);
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
        let time;

        // special case for confirming registration, standalone "page"
        if (window.location.pathname.startsWith("/confirm")) {
            return <ConfirmRegister />
        }

        if (this.state === null || this.state.isLoggedIn === undefined) {
            return <span id="loader" />
        }

        if (window.location.pathname.startsWith("/rooms")) {
            if (this.state.isLoggedIn) {
                pageContent = <RoomsList />
            } else {
                return <Error401 />
            }
        } else if (window.location.pathname === "/") {
            if (this.state.isLoggedIn) {
                time = <Time />
            }

            pageContent =
                <>
                    <Button isPrimary onClick={() => this.showSecret()}>Secret</Button>
                    <p>{this.state.text}</p>
                    {time}
                </>
        } else {
            if (this.state.room) {
                pageContent =
                    <>
                        <Room room={this.state.room} />
                    </>
            }
            else {
                return <Error404 />
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
        )
    }
}

toast.configure();
ReactDOM.render(<App />, document.getElementById("root"));

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
