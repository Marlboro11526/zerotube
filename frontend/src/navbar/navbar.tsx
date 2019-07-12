import React, { Component } from "react";
import Auth from "../auth/auth";
import UserResponse from "../messages/user";
import { Navbar } from "trunx";

interface NavbarProperties {
    authHandler: any,
    isLoggedIn?: boolean,
    username?: string,
}

export default class NavbarComponent extends Component<NavbarProperties> {
    componentDidMount(): void {
        document.querySelectorAll(".navbar-burger").forEach(element => {
            element.addEventListener("click", this.handleBurgerToggle.bind(null, element))
        });
    }

    componentWillUnmount(): void {
        document.querySelectorAll(".navbar-burger").forEach(element => {
            element.removeEventListener("click", this.handleBurgerToggle.bind(null, element))
        });
    }

    handleBurgerToggle(element: Element): void {
        if (element instanceof HTMLElement) {
            let target = document.querySelectorAll(".navbar-menu");

            if (target) {
                element.classList.toggle("is-active");
                target.forEach(element => {
                    element.classList.toggle("is-active");
                })
            }
        }
    }

    render(): JSX.Element {
        let rooms;

        if (this.props.isLoggedIn) {
            rooms =
                <Navbar.Item href="/rooms">
                    Rooms
                </Navbar.Item>
        }

        return (
            <Navbar role="navigation" aria-label="main navigation">
                <Navbar.Brand>
                    <Navbar.Item href="/">
                        <h1 className="title">ZeroTube</h1>
                    </Navbar.Item>
                    <Navbar.Burger />
                </Navbar.Brand>
                <Navbar.Menu>
                    <Navbar.Start>
                        {rooms}
                    </Navbar.Start>
                    <Navbar.End>
                        <Auth
                            authHandler={(response: UserResponse) => this.props.authHandler(response)}
                            isLoggedIn={this.props.isLoggedIn}
                            username={this.props.username}
                        />
                    </Navbar.End>
                </Navbar.Menu>
            </Navbar>
        )
    }
}