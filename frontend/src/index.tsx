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
import Rooms from "./rooms/rooms";
import { Section, Container, Button } from "trunx";

interface AppProperties { }

interface AppState {
    error?: string,
    isLoggedIn?: boolean,
    serverTime?: Date,
    text?: string,
    username?: string,
}

class App extends Component<AppProperties, AppState> {
    constructor(props: AppProperties) {
        super(props);

        this.state = {
            error: undefined,
            isLoggedIn: undefined,
            serverTime: undefined,
            text: undefined,
            username: undefined,
        };
    }

    componentDidMount(): void {
        fetch("http://localhost:8081/auth/start", {
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
            .catch(e => toast("Unable to connect to server.", { type: "error" }));
    }

    showSecret(): void {
        fetch("http://localhost:8081/web/secret", {
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
            .catch((e) => console.log(e))
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

        if (this.state.isLoggedIn) {
            time = <Time />
        }

        if (window.location.pathname.startsWith("/rooms")) {
            pageContent = <Rooms />
        } else {
            pageContent =
                <>
                    <Button isPrimary onClick={() => this.showSecret()}>Secret</Button>
                    <p>{this.state.text}</p>
                    {time}
                </>
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
