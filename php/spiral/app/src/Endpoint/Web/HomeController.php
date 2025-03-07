<?php

declare(strict_types=1);

namespace App\Endpoint\Web;

use Spiral\Router\Annotation\Route;

/**
 * Simple home page controller. It renders home page template and also provides
 * an example of exception page.
 */
final class HomeController
{
    #[Route(route: '/', name: 'index', methods: "GET")]
    public function index(): string
    {
        return '';
    }

    #[Route(route: '/user', name: 'create_user', methods: 'POST')]
    public function create_user($id = null): string
    {
        return '';
    }

    #[Route(route: '/user/<id:int>', name: 'show_user', methods: 'GET')]
    public function show_user($id): string
    {
        return (string)$id;
    }
}
