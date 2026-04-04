//! `gradient_color_at` at fractional boundaries (0.33, 0.55, 0.80).

use storageshower::types::ColorMode;
use storageshower::ui::{gradient_color_at, palette};

#[test]
fn just_below_first_boundary_is_green() {
    let (_, g, _, _, _, _) = palette(ColorMode::Default);
    assert_eq!(gradient_color_at(0.329, ColorMode::Default), g);
}

#[test]
fn at_first_boundary_is_blue_not_green() {
    let (b, g, _, _, _, _) = palette(ColorMode::Green);
    assert_ne!(gradient_color_at(0.33, ColorMode::Green), g);
    assert_eq!(gradient_color_at(0.33, ColorMode::Green), b);
}

#[test]
fn just_below_second_boundary_stays_blue() {
    let (b, _, _, _, _, _) = palette(ColorMode::Blue);
    assert_eq!(gradient_color_at(0.549, ColorMode::Blue), b);
}

#[test]
fn at_second_boundary_is_purple() {
    let (_, _, p, _, _, _) = palette(ColorMode::Purple);
    assert_eq!(gradient_color_at(0.55, ColorMode::Purple), p);
}

#[test]
fn just_below_third_boundary_is_purple() {
    let (_, _, p, _, _, _) = palette(ColorMode::Amber);
    assert_eq!(gradient_color_at(0.799, ColorMode::Amber), p);
}

#[test]
fn at_third_boundary_is_dark_purple() {
    let (_, _, _, _, _, dp) = palette(ColorMode::Red);
    assert_eq!(gradient_color_at(0.80, ColorMode::Red), dp);
}

#[test]
fn frac_one_is_dark_purple() {
    let (_, _, _, _, _, dp) = palette(ColorMode::Cyan);
    assert_eq!(gradient_color_at(1.0, ColorMode::Cyan), dp);
}
